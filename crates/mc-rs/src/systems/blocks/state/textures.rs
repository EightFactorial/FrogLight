use bevy::prelude::*;
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BlockTexturePattern {
    None,
    #[default]
    Single,
    TopBottom,
    TopBottomSides,
    All,
}

impl BlockTextures {
    pub fn get_texture(&self, direction: &Direction) -> Option<Handle<Image>> {
        match self.pattern {
            BlockTexturePattern::None => None,
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
}
