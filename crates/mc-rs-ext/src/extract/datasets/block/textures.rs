use std::fs::File;

use hashbrown::HashMap;
use json::JsonValue;
use log::error;
use zip::ZipArchive;

use crate::types::{ClassMap, Manifest, Version};

use crate::{
    extract::{Dataset, Datasets},
    util::minecraft_jar,
};

use self::{blockstate::BlockStateData, models::ModelData};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockTextures;

impl BlockTextures {
    pub const BLOCKSTATES_PATH: &'static str = "blockstates";
    pub const MODELS_PATH: &'static str = "models/block";
}

impl Dataset for BlockTextures {
    fn min(&self) -> &'static Option<Version> { &None }

    fn deps(&self) -> &'static [Datasets] { &[Datasets::Blocks(super::Blocks)] }

    fn parse(
        &self,
        version: &Version,
        manifest: &Manifest,
        _classmap: &ClassMap,
        _data: &mut JsonValue,
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
        let mut model_assets = Vec::with_capacity(2048);

        for path in zip.file_names() {
            let name = path.trim_start_matches("assets/minecraft/");
            if name.starts_with(Self::BLOCKSTATES_PATH) {
                blockstate_assets.push(path.to_owned());
            } else if name.starts_with(Self::MODELS_PATH) {
                model_assets.push(path.to_owned());
            }
        }

        // TODO: Match blockstates and models
        let _blockstates = parse_blockstates(&blockstate_assets, &mut zip);
        let _models = parse_models(&model_assets, &mut zip);
    }
}

fn parse_blockstates(
    blockstate_assets: &[String],
    zip: &mut ZipArchive<File>,
) -> HashMap<String, BlockStateData> {
    let mut blockstates = HashMap::with_capacity(blockstate_assets.len());
    for path in blockstate_assets {
        match zip.by_name(path) {
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
}

fn parse_models(model_assets: &[String], zip: &mut ZipArchive<File>) -> HashMap<String, ModelData> {
    let mut models = HashMap::with_capacity(model_assets.len());
    for path in model_assets {
        match zip.by_name(path) {
            Ok(mut file) => {
                // Parse the json
                let data: ModelData = match serde_json::from_reader(&mut file) {
                    Ok(data) => data,
                    Err(err) => {
                        error!("Failed to parse json {}: {}", path, err);
                        continue;
                    }
                };

                // Remove the prefix and suffix to get the block name
                let name = file
                    .name()
                    .trim_start_matches("assets/minecraft/models/block/")
                    .trim_end_matches(".json")
                    .to_string();

                models.insert(name, data);
            }
            Err(err) => {
                error!("Failed to open file {}: {}", path, err);
                continue;
            }
        };
    }

    models
}

mod models {
    use hashbrown::HashMap;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ModelData {
        #[serde(default, deserialize_with = "trim_prefix")]
        pub parent: Option<String>,
        #[serde(default)]
        pub textures: HashMap<String, String>,
        #[serde(default)]
        pub elements: Vec<ModelElement>,
        #[serde(default)]
        pub gui_light: Option<String>,
        #[serde(default)]
        pub display: HashMap<ModelDisplayKind, ModelDisplay>,
        #[serde(default = "default_ambientocclusion")]
        pub ambientocclusion: bool,
        #[serde(default)]
        pub groups: Vec<ModelGroup>,
    }

    fn trim_prefix<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Some(
            String::deserialize(deserializer)?
                .trim_start_matches("minecraft:")
                .trim_start_matches("block/")
                .to_string(),
        ))
    }

    fn default_ambientocclusion() -> bool { true }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ModelElement {
        pub from: [f32; 3],
        pub to: [f32; 3],
        pub faces: HashMap<String, ModelFace>,

        #[serde(default)]
        pub name: Option<String>,
        #[serde(default)]
        pub rotation: Option<ModelRotation>,
        #[serde(default = "default_shade")]
        pub shade: bool,
        #[serde(default, rename = "__comment")]
        pub comment: Option<String>,
    }

    fn default_shade() -> bool { true }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ModelFace {
        pub texture: String,

        #[serde(default)]
        pub rotation: Option<i32>,
        #[serde(default)]
        pub uv: Option<[f32; 4]>,
        #[serde(default)]
        pub cullface: Option<String>,
        #[serde(default)]
        pub tintindex: Option<i32>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ModelRotation {
        pub origin: [f32; 3],
        pub axis: Axis,
        pub angle: f32,
        #[serde(default)]
        pub rescale: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum Axis {
        X,
        Y,
        Z,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum ModelDisplayKind {
        #[serde(rename = "thirdperson_lefthand")]
        ThirdPersonLeftHand,
        #[serde(rename = "thirdperson_righthand")]
        ThirdPersonRightHand,
        #[serde(rename = "firstperson_lefthand")]
        FirstPersonLeftHand,
        #[serde(rename = "firstperson_righthand")]
        FirstPersonRightHand,
        Head,
        Gui,
        Ground,
        Fixed,
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ModelDisplay {
        #[serde(default)]
        pub rotation: [i32; 3],
        #[serde(default)]
        pub translation: [f32; 3],
        #[serde(default)]
        pub scale: [f32; 3],
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ModelGroup {
        pub name: String,
        pub origin: [u32; 3],
        pub color: u32,
        pub children: Vec<u32>,
    }
}
