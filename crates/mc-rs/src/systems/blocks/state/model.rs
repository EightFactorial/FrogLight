use bevy::{
    prelude::{Handle, Mesh, Vec3},
    render::primitives::Aabb,
};
use mc_rs_proto::types::enums::Direction;

use super::fluid_model::*;

#[derive(Debug, Clone)]
pub enum BlockModel {
    /// A block without a model.
    None,
    /// A block that is a standard cube.
    Standard,
    /// A rectangular block with the same shape and collision box.
    Simple(Aabb),
    /// A block whose shape and collision box are defined by a list of vertices.
    Complex(Vec<[f32; 3]>),
    /// A block with a custom model and collision box.
    Custom {
        collision: Option<Aabb>,
        mesh: Handle<Mesh>,
    },
}

impl BlockModel {
    /// Modifies the mesh positions to match the block collision box.
    pub fn mod_mesh_positions(&self, direction: &Direction, pos: &mut [[f32; 3]; 4]) {
        match self {
            Self::Simple(collision)
            | Self::Custom {
                collision: Some(collision),
                ..
            } => {
                // Offset the positions with the sides of the shape.
                Self::mod_bounding_box(direction, *collision, pos);
            }
            Self::Complex(vert_pos) => {
                // Get the index of the side of the shape.
                let side_index = match direction {
                    Direction::Up => 0,
                    Direction::Down => 1,
                    Direction::North => 2,
                    Direction::South => 3,
                    Direction::East => 4,
                    Direction::West => 5,
                };

                // Reset vertex positions by subtracting with the cube positions.
                // Offset the positions with the sides of the shape.
                for (i, (pos, cpos)) in pos.iter_mut().zip(&Self::CUBE[side_index]).enumerate() {
                    pos.iter_mut()
                        .zip(cpos)
                        .zip(&vert_pos[side_index * 4 + i])
                        .for_each(|((p, c), v)| *p -= c - v);
                }
            }
            _ => {}
        }
    }

    /// Modifies the vertex positions to match the block collision box.
    fn mod_bounding_box(direction: &Direction, bounding_box: Aabb, pos: &mut [[f32; 3]; 4]) {
        let [min_x, min_y, min_z] = bounding_box.min().to_array();
        let [max_x, max_y, max_z] = bounding_box.max().to_array();

        match direction {
            Direction::Up | Direction::Down | Direction::East | Direction::West => {
                pos[0][0] += min_x;
                pos[2][0] += min_x;
                pos[1][0] -= 1. - max_x;
                pos[3][0] -= 1. - max_x;
            }
            Direction::North | Direction::South => {
                pos[0][0] += min_x;
                pos[1][0] += min_x;
                pos[2][0] += min_x;
                pos[3][0] += min_x;
            }
        }

        match direction {
            Direction::Up => {
                pos[0][1] -= 1. - max_y;
                pos[1][1] -= 1. - max_y;
                pos[2][1] -= 1. - max_y;
                pos[3][1] -= 1. - max_y;
            }
            Direction::Down => {
                pos[0][1] += min_y;
                pos[1][1] += min_y;
                pos[2][1] += min_y;
                pos[3][1] += min_y;
            }
            Direction::North | Direction::South | Direction::East | Direction::West => {
                pos[0][1] += min_y;
                pos[1][1] += min_y;
                pos[2][1] -= 1. - max_y;
                pos[3][1] -= 1. - max_y;
            }
        }

        match direction {
            Direction::Up | Direction::Down | Direction::North | Direction::South => {
                pos[0][2] += min_z;
                pos[1][2] += min_z;
                pos[2][2] -= 1. - max_z;
                pos[3][2] -= 1. - max_z;
            }
            Direction::East => {
                pos[0][2] += min_z;
                pos[1][2] += min_z;
                pos[2][2] += min_z;
                pos[3][2] += min_z;
            }
            Direction::West => {
                pos[0][2] -= 1. - max_z;
                pos[1][2] -= 1. - max_z;
                pos[2][2] -= 1. - max_z;
                pos[3][2] -= 1. - max_z;
            }
        }
    }

    /// The model of a the top slab of a block.
    pub fn slab_upper() -> BlockModel {
        BlockModel::Simple(Aabb::from_min_max(
            Vec3::new(0.0, 0.5, 0.0),
            Vec3::new(1.0, 1.0, 1.0),
        ))
    }

    /// The model of a the bottom slab of a block.
    pub fn slab_lower() -> BlockModel {
        BlockModel::Simple(Aabb::from_min_max(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.5, 1.0),
        ))
    }

    /// The model of a fluid block.
    pub fn fluid_shape(fluid_level: u8) -> BlockModel {
        BlockModel::Complex(
            match fluid_level {
                15 => FLUID_LEVEL_15,
                14 => FLUID_LEVEL_14,
                13 => FLUID_LEVEL_13,
                12 => FLUID_LEVEL_12,
                11 => FLUID_LEVEL_11,
                10 => FLUID_LEVEL_10,
                9 => FLUID_LEVEL_09,
                8 => FLUID_LEVEL_08,
                7 => FLUID_LEVEL_07,
                6 => FLUID_LEVEL_06,
                5 => FLUID_LEVEL_05,
                4 => FLUID_LEVEL_04,
                3 => FLUID_LEVEL_03,
                2 => FLUID_LEVEL_02,
                1 => FLUID_LEVEL_01,
                0 => FLUID_LEVEL_00,
                _ => panic!("Invalid fluid level: {}", fluid_level),
            }
            .into_iter()
            .flatten()
            .collect(),
        )
    }

    /// The vertices of a cube.
    const CUBE: [[[f32; 3]; 4]; 6] = [
        // Up
        [
            [0.0, 1.0, 0.0],
            [1.0, 1.0, 0.0],
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 1.0],
        ],
        // Down
        [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
        // North
        [
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 1.0],
        ],
        // South
        [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
        // East
        [
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, 0.0],
        ],
        // West
        [
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 1.0],
            [0.0, 1.0, 0.0],
        ],
    ];
}
