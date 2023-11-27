use std::fs::File;

use hashbrown::HashMap;
use json::JsonValue;
use serde::{Deserialize, Serialize};
use strum::Display;
use tracing::error;
use zip::ZipArchive;

use crate::types::{ClassMap, Manifest, Version};

use crate::{
    extract::{Dataset, Datasets},
    util::minecraft_jar,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockModels;

impl BlockModels {
    pub const MODELS_PATH: &'static str = "models/block";
}

impl Dataset for BlockModels {
    fn min(&self) -> &'static Option<Version> { &None }

    fn deps(&self) -> &'static [Datasets] { &[] }

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

        let mut model_assets = Vec::with_capacity(2048);
        for path in zip.file_names() {
            let name = path.trim_start_matches("assets/minecraft/");
            if name.starts_with(Self::MODELS_PATH) {
                model_assets.push(path.to_owned());
            }
        }

        // Get the models
        for (name, model) in parse_models(model_assets, &mut zip) {
            let mut object = JsonValue::new_object();

            if let Some(parent) = model.parent {
                object["parent"] = parent.into();
            }

            model.textures.into_iter().for_each(|(key, value)| {
                object["textures"][key] = value.into();
            });

            // TODO: Insert elements

            model.display.into_iter().for_each(|(key, value)| {
                object["display"][key.to_string()] = value.into();
            });

            if let Some(gui_light) = model.gui_light {
                object["gui_light"] = gui_light.into();
            }
            if !model.ambientocclusion {
                object["ambient_occlusion"] = model.ambientocclusion.into();
            }

            data["models"][name] = object;
        }
    }
}

fn parse_models(
    model_assets: Vec<String>,
    zip: &mut ZipArchive<File>,
) -> HashMap<String, ModelData> {
    let mut models = HashMap::with_capacity(model_assets.len());
    for path in model_assets {
        match zip.by_name(&path) {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelData {
    #[serde(default)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
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
    pub rotation: Option<[i32; 3]>,
    #[serde(default)]
    pub translation: Option<[f32; 3]>,
    #[serde(default)]
    pub scale: Option<[f32; 3]>,
}

impl From<ModelDisplay> for JsonValue {
    fn from(value: ModelDisplay) -> Self {
        let mut data = JsonValue::new_object();

        if let Some(rotation) = value.rotation {
            data["rotation"] = rotation.to_vec().into();
        }

        if let Some(translation) = value.translation {
            data["translation"] = translation.map(Datasets::round_float_f32).to_vec().into();
        }

        if let Some(scale) = value.scale {
            data["scale"] = scale.map(Datasets::round_float_f32).to_vec().into();
        }

        data
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelGroup {
    pub name: String,
    pub origin: [u32; 3],
    pub color: u32,
    pub children: Vec<u32>,
}
