#![allow(dead_code)]

use bevy::prelude::{AssetServer, Handle, Image};
use convert_case::{Case, Casing};
use mc_rs_proto::types::{enums::Direction, ResourceLocation};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub id: u32,
    pub name: String,
    pub key: ResourceLocation,
    pub texture: BlockTexture,
}

impl Block {
    /// Create a new block with the given id, name, and texture path(s)
    pub fn new(id: u32, name: &str, paths: &[&str], assets: &AssetServer) -> Option<Self> {
        Some(Self {
            id,
            name: name.to_case(Case::Title),
            key: ResourceLocation::new(name.to_case(Case::Snake)),
            texture: BlockTexture::from_paths(paths, assets)?,
        })
    }

    /// Create a new block with the given id, name, and texture
    pub fn new_with(id: u32, name: &str, texture: BlockTexture) -> Self {
        Self {
            id,
            name: name.to_case(Case::Title),
            key: ResourceLocation::new(name.to_case(Case::Snake)),
            texture,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockTexture {
    None,
    /// The same texture on all sides
    Single(Handle<Image>),
    /// The same texture on the top and bottom, and another texture on all sides
    TopBottom(Vec<Handle<Image>>),
    /// A different texture on the top, the bottom, and the sides
    TopBottomSides(Vec<Handle<Image>>),
    /// A different texture on the top, bottom, and sides
    AllSides(Vec<Handle<Image>>),
}

impl BlockTexture {
    /// Get the texture for the given direction
    pub fn get_texture(&self, direction: Direction) -> Option<&Handle<Image>> {
        match self {
            BlockTexture::None => None,
            BlockTexture::Single(texture) => Some(texture),
            BlockTexture::TopBottom(textures) => match direction {
                Direction::Up | Direction::Down => Some(&textures[0]),
                _ => Some(&textures[1]),
            },
            BlockTexture::TopBottomSides(textures) => match direction {
                Direction::Up => Some(&textures[0]),
                Direction::Down => Some(&textures[1]),
                _ => Some(&textures[2]),
            },
            BlockTexture::AllSides(textures) => match direction {
                Direction::Up => Some(&textures[0]),
                Direction::Down => Some(&textures[1]),
                Direction::North => Some(&textures[2]),
                Direction::South => Some(&textures[3]),
                Direction::West => Some(&textures[4]),
                Direction::East => Some(&textures[5]),
            },
        }
    }

    /// Get all of the block textures
    pub fn get_textures(&self) -> Option<&[Handle<Image>]> {
        match self {
            BlockTexture::None => None,
            BlockTexture::Single(texture) => Some(std::slice::from_ref(texture)),
            BlockTexture::TopBottom(textures) => Some(textures),
            BlockTexture::TopBottomSides(textures) => Some(textures),
            BlockTexture::AllSides(textures) => Some(textures),
        }
    }

    /// Creates a new block texture from a list of textures
    pub fn from_paths(paths: &[&str], assets: &AssetServer) -> Option<Self> {
        Self::from_textures(
            paths
                .iter()
                .map(|&path| Self::load_test_texture(path, assets))
                .collect(),
        )
    }

    /// Creates a new block texture from a list of textures
    pub fn from_textures(textures: Vec<Handle<Image>>) -> Option<Self> {
        match textures.len() {
            0 => Some(Self::None),
            1 => Some(Self::Single(
                textures
                    .into_iter()
                    .next()
                    .expect("Texture list with length 1 but no texture?"),
            )),
            2 => Some(Self::TopBottom(textures)),
            3 => Some(Self::TopBottomSides(textures)),
            6 => Some(Self::AllSides(textures)),
            _ => None,
        }
    }

    /// Creates a new block texture from a single texture
    pub fn new_single(texture: &str, assets: &AssetServer) -> Self {
        Self::Single(Self::load_test_texture(texture, assets))
    }

    /// Creates a new block texture from a top/bottom texture and a side texture
    pub fn new_top_bottom(top_bottom: &str, sides: &str, assets: &AssetServer) -> Self {
        Self::TopBottom(vec![
            Self::load_test_texture(top_bottom, assets),
            Self::load_test_texture(sides, assets),
        ])
    }

    /// Creates a new block texture from a top texture, bottom texture, and a side texture
    pub fn new_top_bottom_sides(
        top: &str,
        bottom: &str,
        sides: &str,
        assets: &AssetServer,
    ) -> Self {
        Self::TopBottomSides(vec![
            Self::load_test_texture(top, assets),
            Self::load_test_texture(bottom, assets),
            Self::load_test_texture(sides, assets),
        ])
    }

    /// Creates a new block texture from a texture for each side
    pub fn new_all_sides(
        top: &str,
        bottom: &str,
        north: &str,
        south: &str,
        west: &str,
        east: &str,
        assets: &AssetServer,
    ) -> Self {
        Self::AllSides(vec![
            Self::load_test_texture(top, assets),
            Self::load_test_texture(bottom, assets),
            Self::load_test_texture(north, assets),
            Self::load_test_texture(south, assets),
            Self::load_test_texture(west, assets),
            Self::load_test_texture(east, assets),
        ])
    }

    /// Shortcut for loading a test texture
    fn load_test_texture(path: &str, assets: &AssetServer) -> Handle<Image> {
        assets.load(format!("test/textures/block/{path}"))
    }

    /// Shortcut for loading a texture
    fn load_texture(path: &str, assets: &AssetServer) -> Handle<Image> {
        assets.load(format!("textures/block/{path}"))
    }
}
