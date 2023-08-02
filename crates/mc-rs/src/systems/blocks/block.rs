#![allow(dead_code)]

use bevy::prelude::{AssetServer, Handle, Image};
use convert_case::{Case, Casing};
use mc_rs_proto::types::{enums::Direction, ResourceLocation};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub id: u32,
    pub name: String,
    pub key: ResourceLocation,
    pub textures: BlockTexture,
}

impl Block {
    pub(crate) fn new(id: u32, name: &str, textures: BlockTexture) -> Self {
        Self {
            id,
            name: name.to_case(Case::Title),
            key: ResourceLocation::new(name.to_case(Case::Snake)),
            textures,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockTexture {
    None,
    /// The same texture on all sides
    Single(Handle<Image>),
    /// The same texture on the top and bottom, and another texture on all sides
    TopBottom(Handle<Image>, Handle<Image>),
    /// A different texture on the top, the bottom, and the sides
    TopBottomSides(Handle<Image>, Handle<Image>, Handle<Image>),
    /// A different texture on the top, bottom, and sides
    AllSides(
        Handle<Image>,
        Handle<Image>,
        Handle<Image>,
        Handle<Image>,
        Handle<Image>,
        Handle<Image>,
    ),
}

impl BlockTexture {
    pub fn get_texture(&self, direction: Direction) -> Option<&Handle<Image>> {
        match self {
            BlockTexture::None => None,
            BlockTexture::Single(texture) => Some(texture),
            BlockTexture::TopBottom(top_bot, side) => match direction {
                Direction::Up | Direction::Down => Some(top_bot),
                _ => Some(side),
            },
            BlockTexture::TopBottomSides(top, bot, side) => match direction {
                Direction::Up => Some(top),
                Direction::Down => Some(bot),
                _ => Some(side),
            },
            BlockTexture::AllSides(top, bot, north, south, west, east) => match direction {
                Direction::Up => Some(top),
                Direction::Down => Some(bot),
                Direction::North => Some(north),
                Direction::South => Some(south),
                Direction::West => Some(west),
                Direction::East => Some(east),
            },
        }
    }

    pub fn get_textures(&self) -> Option<Vec<&Handle<Image>>> {
        match self {
            BlockTexture::None => None,
            BlockTexture::Single(texture) => Some(vec![texture]),
            BlockTexture::TopBottom(top_bot, side) => Some(vec![top_bot, side]),
            BlockTexture::TopBottomSides(top, bot, side) => Some(vec![top, bot, side]),
            BlockTexture::AllSides(top, bot, north, south, west, east) => {
                Some(vec![top, bot, north, south, west, east])
            }
        }
    }

    pub(crate) fn new_single(texture: &str, assets: &AssetServer) -> Self {
        Self::Single(Self::load_image(texture, assets))
    }

    pub(crate) fn new_top_bottom(top_bottom: &str, sides: &str, assets: &AssetServer) -> Self {
        Self::TopBottom(
            Self::load_image(top_bottom, assets),
            Self::load_image(sides, assets),
        )
    }

    pub(crate) fn new_top_bottom_sides(
        top: &str,
        bottom: &str,
        sides: &str,
        assets: &AssetServer,
    ) -> Self {
        Self::TopBottomSides(
            Self::load_image(top, assets),
            Self::load_image(bottom, assets),
            Self::load_image(sides, assets),
        )
    }

    pub(crate) fn new_all_sides(
        top: &str,
        bottom: &str,
        north: &str,
        south: &str,
        west: &str,
        east: &str,
        assets: &AssetServer,
    ) -> Self {
        Self::AllSides(
            Self::load_image(top, assets),
            Self::load_image(bottom, assets),
            Self::load_image(north, assets),
            Self::load_image(south, assets),
            Self::load_image(west, assets),
            Self::load_image(east, assets),
        )
    }

    fn load_image(path: &str, assets: &AssetServer) -> Handle<Image> {
        assets.load(format!("test/textures/block/{path}"))
    }
}
