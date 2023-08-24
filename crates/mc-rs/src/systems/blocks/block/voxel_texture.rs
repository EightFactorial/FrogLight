use bevy::prelude::{AssetServer, Handle, Image};
use mc_rs_proto::types::enums::Direction;

/// A voxel texture
///
/// This is used to determine what texture(s) a voxel should use and on which sides
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoxelTexture {
    pub textures: Option<Vec<TextureInfo>>,
    pub pattern: TexturePattern,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextureInfo {
    pub texture: Handle<Image>,
    /// (# of frames, frame time in ms)
    pub frames: Option<(u32, u32)>,
}

impl From<Handle<Image>> for TextureInfo {
    fn from(value: Handle<Image>) -> Self {
        Self {
            texture: value,
            frames: None,
        }
    }
}

/// The pattern for a voxel texture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TexturePattern {
    None,
    Single,
    TopBottom,
    TopBottomSides,
    AllSides,
}

impl VoxelTexture {
    /// Get the texture index for the given direction
    pub fn get_direction_index(&self, direction: &Direction) -> Option<usize> {
        match self.pattern {
            TexturePattern::None => None,
            TexturePattern::Single => Some(0),
            TexturePattern::TopBottom => match direction {
                Direction::Up | Direction::Down => Some(0),
                _ => Some(1),
            },
            TexturePattern::TopBottomSides => match direction {
                Direction::Up => Some(0),
                Direction::Down => Some(1),
                _ => Some(2),
            },
            TexturePattern::AllSides => match direction {
                Direction::Up => Some(0),
                Direction::Down => Some(1),
                Direction::North => Some(2),
                Direction::South => Some(3),
                Direction::West => Some(4),
                Direction::East => Some(5),
            },
        }
    }

    /// Get the texture for the given direction
    #[allow(dead_code)]
    pub fn get_texture(&self, direction: &Direction) -> Option<&TextureInfo> {
        self.get_direction_index(direction)
            .and_then(|i| self.textures.as_ref().and_then(|textures| textures.get(i)))
    }

    /// Creates a new voxel texture from a list of textures
    pub fn from_textures(textures: Vec<Handle<Image>>) -> Option<Self> {
        let pattern = match textures.len() {
            0 => TexturePattern::None,
            1 => TexturePattern::Single,
            2 => TexturePattern::TopBottom,
            3 => TexturePattern::TopBottomSides,
            6 => TexturePattern::AllSides,
            _ => return None,
        };

        Some(Self {
            textures: Some(textures.into_iter().map(Into::into).collect()),
            pattern,
        })
    }

    /// Creates a new voxel texture from a list of textures
    pub fn from_paths(paths: &[&str], assets: &AssetServer) -> Option<Self> {
        Self::from_textures(
            paths
                .iter()
                .map(|&path| Self::load_test_texture(path, assets))
                .collect(),
        )
    }

    /// Creates a new voxel texture from a list of textures with frames
    pub fn from_paths_with_frames(
        paths: &[&str],
        frame_info: &[Option<(u32, u32)>],
        assets: &AssetServer,
    ) -> Option<Self> {
        let mut texture = Self::from_paths(paths, assets)?;
        if let Some(texture) = &mut texture.textures {
            for (frame, info) in texture.iter_mut().zip(frame_info) {
                if let Some(info) = info {
                    frame.frames = Some(*info);
                }
            }
        }

        Some(texture)
    }

    /// Shortcut for loading a test texture
    fn load_test_texture(path: &str, assets: &AssetServer) -> Handle<Image> {
        assets.load(format!("test/textures/block/{path}"))
    }

    /// Shortcut for loading a texture
    #[allow(dead_code)]
    fn load_texture(path: &str, assets: &AssetServer) -> Handle<Image> {
        assets.load(format!("textures/block/{path}"))
    }
}
