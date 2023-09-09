use bevy::prelude::*;
use itertools::Itertools;
use mc_rs_proto::types::enums::Direction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockTextures {
    pub pattern: BlockTexturePattern,
    pub textures: Vec<Handle<Image>>,
}

impl Default for BlockTextures {
    fn default() -> Self {
        Self {
            pattern: Default::default(),
            textures: vec![Handle::<Image>::default()],
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BlockTexturePattern {
    None,
    #[default]
    Single,
    TopBottom,
    TopBottomSides,
    All,
    Custom,
}

impl BlockTextures {
    pub const NONE: Self = Self {
        pattern: BlockTexturePattern::None,
        textures: vec![],
    };

    /// Creates a new `BlockTextures` from a list of texture paths with .
    pub fn new(paths: &[&str], asset_server: &AssetServer) -> Self {
        let textures = Self::load_paths(paths, asset_server);

        match textures.len() {
            0 => Self {
                pattern: BlockTexturePattern::None,
                textures,
            },
            1 => Self {
                pattern: BlockTexturePattern::Single,
                textures,
            },
            2 => Self {
                pattern: BlockTexturePattern::TopBottom,
                textures,
            },
            3 => Self {
                pattern: BlockTexturePattern::TopBottomSides,
                textures,
            },
            6 => Self {
                pattern: BlockTexturePattern::All,
                textures,
            },
            _ => panic!("Invalid number of textures"),
        }
    }

    /// Creates a new `BlockTextures` from a list of paths with a custom pattern.
    #[allow(dead_code)]
    pub fn new_custom(paths: &[&str], asset_server: &AssetServer) -> Self {
        Self {
            pattern: BlockTexturePattern::Custom,
            textures: Self::load_paths(paths, asset_server),
        }
    }

    pub fn get_texture(&self, direction: &Direction) -> Option<Handle<Image>> {
        match self.pattern {
            BlockTexturePattern::None | BlockTexturePattern::Custom => None,
            BlockTexturePattern::Single => self.textures.get(0).cloned(),
            BlockTexturePattern::TopBottom => match direction {
                Direction::Up | Direction::Down => self.textures.get(0).cloned(),
                _ => self.textures.get(1).cloned(),
            },
            BlockTexturePattern::TopBottomSides => match direction {
                Direction::Up => self.textures.get(0).cloned(),
                Direction::Down => self.textures.get(1).cloned(),
                _ => self.textures.get(2).cloned(),
            },
            BlockTexturePattern::All => match direction {
                Direction::Up => self.textures.get(0).cloned(),
                Direction::Down => self.textures.get(1).cloned(),
                Direction::North => self.textures.get(2).cloned(),
                Direction::South => self.textures.get(3).cloned(),
                Direction::West => self.textures.get(4).cloned(),
                Direction::East => self.textures.get(5).cloned(),
            },
        }
    }

    fn load_paths(paths: &[&str], asset_server: &AssetServer) -> Vec<Handle<Image>> {
        paths
            .iter()
            .map(|&path| asset_server.load(format!("test/textures/block/{path}")))
            .collect_vec()
    }
}
