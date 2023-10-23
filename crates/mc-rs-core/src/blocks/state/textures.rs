use bevy::prelude::*;
use itertools::Itertools;
use mc_rs_protocol::types::enums::Direction;

use crate::world::material::StateAnimation;

#[derive(Debug, Default, Clone)]
pub struct BlockTextures {
    pub pattern: BlockTexturePattern,
    pub textures: Vec<BlockTexture>,
}

#[derive(Debug, Default, Clone)]
pub struct BlockTexture {
    pub image: Handle<Image>,
    pub animation: Option<StateAnimation>,
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

    /// Creates a new `BlockTextures` from a list of texture paths with animations.
    #[allow(dead_code)]
    pub fn new_with_animations(
        data: &[(&str, StateAnimation)],
        asset_server: &AssetServer,
    ) -> Self {
        let mut textures =
            Self::load_paths(&data.iter().map(|(p, _)| *p).collect_vec(), asset_server);

        textures
            .iter_mut()
            .zip_eq(data)
            .for_each(|(t, (_, a))| t.animation = Some(*a));

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

    fn get_texture_object(&self, direction: &Direction) -> Option<&BlockTexture> {
        match self.pattern {
            BlockTexturePattern::None | BlockTexturePattern::Custom => None,
            BlockTexturePattern::Single => self.textures.first(),
            BlockTexturePattern::TopBottom => match direction {
                Direction::Up | Direction::Down => self.textures.first(),
                _ => self.textures.get(1),
            },
            BlockTexturePattern::TopBottomSides => match direction {
                Direction::Up => self.textures.first(),
                Direction::Down => self.textures.get(1),
                _ => self.textures.get(2),
            },
            BlockTexturePattern::All => match direction {
                Direction::Up => self.textures.first(),
                Direction::Down => self.textures.get(1),
                Direction::North => self.textures.get(2),
                Direction::South => self.textures.get(3),
                Direction::West => self.textures.get(4),
                Direction::East => self.textures.get(5),
            },
        }
    }

    /// Returns the texture for the given direction.
    pub fn get_texture(&self, direction: &Direction) -> Option<&Handle<Image>> {
        self.get_texture_object(direction).map(|t| &t.image)
    }

    /// Returns the animation for the given direction.
    pub fn get_animation(&self, direction: &Direction) -> Option<&StateAnimation> {
        self.get_texture_object(direction)
            .and_then(|t| t.animation.as_ref())
    }

    fn load_paths(paths: &[&str], asset_server: &AssetServer) -> Vec<BlockTexture> {
        paths
            .iter()
            .map(|&path| BlockTexture {
                image: asset_server
                    .load(format!("minecraft/assets/minecraft/textures/block/{path}")),
                animation: None,
            })
            .collect_vec()
    }
}
