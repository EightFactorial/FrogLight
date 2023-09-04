use std::fs::File;

use hashbrown::HashMap;
use json::JsonValue;
use log::{error, warn};
use zip::ZipArchive;

use crate::types::{ClassMap, Manifest, Version};

use crate::{
    extract::{Dataset, Datasets},
    util::minecraft_jar,
};

use self::blockstate::{BlockStateData, StateValueEnum, StateVariantEnum};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockVariants;

impl BlockVariants {
    pub const BLOCKSTATES_PATH: &'static str = "blockstates";
}

impl Dataset for BlockVariants {
    fn min(&self) -> &'static Option<Version> { &None }

    fn deps(&self) -> &'static [Datasets] { &[Datasets::BlockStates(super::BlockStates)] }

    fn parse(
        &self,
        version: &Version,
        manifest: &Manifest,
        _classmap: &ClassMap,
        data: &mut JsonValue,
    ) {
        let Some(path) = minecraft_jar(version, manifest) else {
            error!("Failed to find jar for version {}", version);
            return;
        };

        let jar = match File::open(&path) {
            Ok(jar) => jar,
            Err(err) => {
                error!("Failed to open jar {}: {}", path.display(), err);
                return;
            }
        };

        let mut zip = match ZipArchive::new(jar) {
            Ok(zip) => zip,
            Err(err) => {
                error!("Failed to open jar {}: {}", path.display(), err);
                return;
            }
        };

        let mut blockstate_assets = Vec::with_capacity(1024);
        for path in zip.file_names() {
            let name = path.trim_start_matches("assets/minecraft/");
            if name.starts_with(Self::BLOCKSTATES_PATH) {
                blockstate_assets.push(path.to_owned());
            }
        }

        // Get the blockstates
        for (name, state) in parse_blockstates(blockstate_assets, &mut zip) {
            let Some((_, block)) = data["blocks"]["blocks"]["blocks"]
                .entries_mut()
                .find(|(k, _)| k == &name)
            else {
                // TODO: Fix this
                if !name.contains("item_frame") {
                    warn!("Failed to find block {} in block list", name);
                }
                continue;
            };

            match state {
                BlockStateData::Variant { variants } => {
                    for (variant, variant_data) in variants {
                        match variant_data {
                            StateVariantEnum::Single(variant) => {
                                block["model_data"] = variant.into();
                            }
                            StateVariantEnum::Multiple(variants) => {
                                block["variants"][variant]["model_data"] = variants
                                    .into_iter()
                                    .map(JsonValue::from)
                                    .collect::<Vec<_>>()
                                    .into();
                            }
                        }
                    }
                }
                BlockStateData::Multipart { multipart } => {
                    let mut list = JsonValue::new_array();
                    for part in multipart {
                        let mut data = JsonValue::new_object();

                        if let Some(when) = part.when {
                            let mut when_data = JsonValue::new_object();
                            match &when {
                                StateValueEnum::Or { .. } => when_data["condition"] = "or".into(),
                                StateValueEnum::And { .. } => when_data["condition"] = "and".into(),
                                StateValueEnum::Value(_) => {}
                            };

                            match when {
                                StateValueEnum::Or { or: data }
                                | StateValueEnum::And { and: data } => {
                                    when_data["data"] = data.into_iter().fold(
                                        JsonValue::new_array(),
                                        |mut acc, value| {
                                            let cond = value.into_iter().fold(
                                                JsonValue::new_object(),
                                                |mut acc, (key, value)| {
                                                    acc[key] = value.into();
                                                    acc
                                                },
                                            );

                                            let _ = acc.push(cond);
                                            acc
                                        },
                                    )
                                }
                                StateValueEnum::Value(value) => {
                                    let cond = value.into_iter().fold(
                                        JsonValue::new_object(),
                                        |mut acc, (key, value)| {
                                            acc[key] = value.into();
                                            acc
                                        },
                                    );

                                    when_data["data"] = vec![cond].into();
                                }
                            }

                            data["conditions"] = when_data;
                        }

                        match part.apply {
                            StateVariantEnum::Multiple(variants) => {
                                data["model_data"] = variants
                                    .into_iter()
                                    .map(JsonValue::from)
                                    .collect::<Vec<_>>()
                                    .into();
                            }
                            StateVariantEnum::Single(variant) => {
                                data["model_data"] = variant.into();
                            }
                        }

                        let _ = list.push(data);
                    }

                    block["multipart"] = list;
                }
            }
        }
    }
}

fn parse_blockstates(
    blockstate_assets: Vec<String>,
    zip: &mut ZipArchive<File>,
) -> HashMap<String, BlockStateData> {
    let mut blockstates = HashMap::with_capacity(blockstate_assets.len());
    for path in blockstate_assets {
        match zip.by_name(&path) {
            Ok(mut file) => {
                // Parse the json
                let data: BlockStateData = match serde_json::from_reader(&mut file) {
                    Ok(data) => data,
                    Err(err) => {
                        error!("Failed to parse json {}: {}", path, err);
                        continue;
                    }
                };

                // Remove the prefix and suffix to get the block name
                let name = file
                    .name()
                    .trim_start_matches("assets/minecraft/blockstates/")
                    .trim_end_matches(".json")
                    .to_string();

                blockstates.insert(name, data);
            }
            Err(err) => {
                error!("Failed to open file {}: {}", path, err);
                continue;
            }
        };
    }

    blockstates
}

mod blockstate {
    use hashbrown::HashMap;
    use json::JsonValue;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum BlockStateData {
        Variant {
            variants: HashMap<String, StateVariantEnum>,
        },
        Multipart {
            multipart: Vec<StateMultipart>,
        },
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct StateMultipart {
        #[serde(default)]
        pub when: Option<StateValueEnum>,
        pub apply: StateVariantEnum,
    }

    pub type StateValue = HashMap<String, String>;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum StateValueEnum {
        Or {
            #[serde(rename = "OR")]
            or: Vec<StateValue>,
        },
        And {
            #[serde(rename = "AND")]
            and: Vec<StateValue>,
        },
        Value(StateValue),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum StateVariantEnum {
        Single(StateVariant),
        Multiple(Vec<StateVariant>),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct StateVariant {
        pub model: String,

        #[serde(default)]
        pub x: Option<i32>,
        #[serde(default)]
        pub y: Option<i32>,
        #[serde(default)]
        pub uvlock: Option<bool>,
        #[serde(default)]
        pub weight: Option<i32>,
    }

    impl From<StateVariant> for JsonValue {
        fn from(value: StateVariant) -> Self {
            let mut data = JsonValue::new_object();
            data["model"] = value.model.into();

            if let Some(x) = value.x {
                data["x"] = x.into();
            }
            if let Some(y) = value.y {
                data["y"] = y.into();
            }
            if let Some(uvlock) = value.uvlock {
                data["uvlock"] = uvlock.into();
            }
            if let Some(weight) = value.weight {
                data["weight"] = weight.into();
            }

            data
        }
    }
}
