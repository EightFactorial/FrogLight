use bevy::utils::HashMap;
use compact_str::CompactString;
use mc_rs_core::ResourceLocation;
use serde::Deserialize;

/// A Minecraft block model.
///
/// This struct is a direct representation of the json model files,
/// and are not guaranteed to be valid.
///
/// Some models are not used directly, but as templates for other models.
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct Model {
    pub parent: Option<ResourceLocation>,
    pub textures: Option<HashMap<CompactString, CompactString>>,
    pub display: Option<HashMap<CompactString, ModelDisplay>>,
    pub elements: Option<Vec<ModelElement>>,
    pub gui_light: Option<CompactString>,
}

impl Model {
    /// Gets the texture for the given side of the block.
    ///
    /// Optionally, a hashmap of all the textures of the child models can be passed in.
    pub fn get_texture(
        &self,
        name: &str,
        models: &HashMap<ResourceLocation, Model>,
        textures: Option<&mut HashMap<CompactString, CompactString>>,
    ) -> Option<ResourceLocation> {
        // Check if the current model has textures
        if let Some(texture_list) = &self.textures {
            // Check if the current model has the texture.
            if let Some(texture) = texture_list.get(name) {
                // If the texture starts with `#`, it is a reference to a child's texture.
                if let Some(child_texture_name) = texture.strip_prefix('#') {
                    // Check if there are any child textures.
                    if let Some(ref child_textures) = textures {
                        // Check if the child model has the texture.
                        if let Some(child_texture) = child_textures.get(child_texture_name) {
                            return Some(ResourceLocation::from(child_texture.clone()));
                        }
                    }
                } else {
                    // If the texture does not start with #, it is a texture in the current model.
                    return Some(ResourceLocation::from(texture.clone()));
                }
            }
        }

        // Check if the parent model has the texture.
        if let Some(parent) = &self.parent {
            if let Some(parent) = models.get(parent) {
                // If this is the first time we are checking the parent model, create a new hashmap.
                let mut child_textures = HashMap::new();
                let mut child_textures = Some(&mut child_textures);
                if let Some(textures) = textures {
                    child_textures = Some(textures);
                }

                // Add the current model's textures to the hashmap.
                if let Some(textures) = &self.textures {
                    if let Some(t) = child_textures.as_mut() {
                        t.extend(textures.clone())
                    }
                }

                // Check if the parent model has the texture.
                return parent.get_texture(name, models, child_textures);
            }
        }

        None
    }

    /// Gets the display for the given side of the block.
    pub fn get_display(
        &self,
        name: &str,
        models: &HashMap<ResourceLocation, Model>,
    ) -> Option<ModelDisplay> {
        // Check if the model has the display.
        if let Some(display) = self.display.as_ref().and_then(|display| display.get(name)) {
            return Some(*display);
        }

        // Check if the parent model has the display.
        if let Some(parent) = self.parent.as_ref() {
            if let Some(parent) = models.get(parent) {
                return parent.get_display(name, models);
            }
        }

        None
    }

    /// Gets the gui light for the model.
    pub fn get_gui_light(
        &self,
        models: &HashMap<ResourceLocation, Model>,
    ) -> Option<CompactString> {
        // Check if the model has the gui light.
        if let Some(gui_light) = self.gui_light.as_ref() {
            return Some(gui_light.clone());
        }

        // Check if the parent model has the gui light.
        if let Some(parent) = self.parent.as_ref() {
            if let Some(parent) = models.get(parent) {
                return parent.get_gui_light(models);
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct ModelDisplay {
    #[serde(default)]
    pub rotation: [f32; 3],
    #[serde(default)]
    pub translation: [f32; 3],
    #[serde(default)]
    pub scale: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ModelElement {
    pub from: [f32; 3],
    pub to: [f32; 3],
    pub shade: Option<bool>,
    pub rotation: Option<ModelElementRotation>,
    pub faces: Option<HashMap<CompactString, ModelElementFace>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ModelElementRotation {
    pub origin: [f32; 3],
    pub axis: ModelAxis,
    pub angle: f32,
    #[serde(default = "default_true")]
    pub rescale: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelAxis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ModelElementFace {
    #[serde(default)]
    pub uv: [f32; 4],
    pub texture: CompactString,
    #[serde(default)]
    pub rotation: f32,
    pub cullface: Option<CompactString>,
    #[serde(default)]
    pub tintindex: u32,
}

fn default_true() -> bool { true }
