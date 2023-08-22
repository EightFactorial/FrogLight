use bevy::prelude::{AssetServer, Handle, Image};
use mc_rs_proto::types::enums::Direction;

/// A voxel texture
///
/// This is used to determine what texture(s) a voxel should use and on which sides
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoxelTexture {
    /// No texture
    None,
    /// The same texture on all sides
    Single(Vec<Handle<Image>>),
    /// The same texture on the top and bottom, and another texture on all sides
    TopBottom(Vec<Handle<Image>>),
    /// A different texture on the top, the bottom, and the sides
    TopBottomSides(Vec<Handle<Image>>),
    /// A different texture on the top, bottom, and sides
    AllSides(Vec<Handle<Image>>),
}

impl VoxelTexture {
    /// Get all of the voxel textures
    pub fn get_textures(&self) -> Option<&[Handle<Image>]> {
        match self {
            VoxelTexture::None => None,
            VoxelTexture::Single(textures)
            | VoxelTexture::TopBottom(textures)
            | VoxelTexture::TopBottomSides(textures)
            | VoxelTexture::AllSides(textures) => Some(textures),
        }
    }

    /// Get the texture index for the given direction
    pub fn get_texture_index(&self, direction: Direction) -> Option<usize> {
        match self {
            VoxelTexture::None => None,
            VoxelTexture::Single(_) => Some(0),
            VoxelTexture::TopBottom(_) => match direction {
                Direction::Up | Direction::Down => Some(0),
                _ => Some(1),
            },
            VoxelTexture::TopBottomSides(_) => match direction {
                Direction::Up => Some(0),
                Direction::Down => Some(1),
                _ => Some(2),
            },
            VoxelTexture::AllSides(_) => match direction {
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
    pub fn get_texture(&self, direction: Direction) -> Option<&Handle<Image>> {
        self.get_textures().and_then(|textures| {
            self.get_texture_index(direction)
                .map(|index| &textures[index])
        })
    }

    /// Creates a new voxel texture from a list of textures
    pub fn from_textures(textures: Vec<Handle<Image>>) -> Option<Self> {
        match textures.len() {
            0 => Some(Self::None),
            1 => Some(Self::Single(textures)),
            2 => Some(Self::TopBottom(textures)),
            3 => Some(Self::TopBottomSides(textures)),
            6 => Some(Self::AllSides(textures)),
            _ => None,
        }
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
