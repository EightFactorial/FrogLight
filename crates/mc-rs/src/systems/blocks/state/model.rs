use bevy::{
    prelude::{Handle, Mesh},
    render::primitives::Aabb,
};
use mc_rs_proto::types::enums::Direction;

#[derive(Debug, Clone)]
pub enum BlockModel {
    /// A block without a model.
    None,
    /// A block that is a cube.
    Standard,
    /// A block whose shape and collision box are the same.
    Simple(Aabb),
    /// A block with a custom model and collision.
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
                Self::mod_pos(direction, *collision, pos);
            }
            _ => {}
        }
    }

    fn mod_pos(direction: &Direction, bounding_box: Aabb, pos: &mut [[f32; 3]; 4]) {
        let [min_x, min_y, min_z] = (bounding_box.min() / 16.0).to_array();
        let [max_x, max_y, max_z] = (bounding_box.max() / 16.0).to_array();

        match direction {
            Direction::Up => {
                pos[0][0] += min_x;
                pos[2][0] += min_x;
                pos[1][0] -= 1. - max_x;
                pos[3][0] -= 1. - max_x;

                pos[0][1] -= 1. - max_y;
                pos[1][1] -= 1. - max_y;
                pos[2][1] -= 1. - max_y;
                pos[3][1] -= 1. - max_y;

                pos[0][2] += min_z;
                pos[1][2] += min_z;
                pos[2][2] -= 1. - max_z;
                pos[3][2] -= 1. - max_z;
            }
            Direction::Down => {
                pos[0][0] += min_x;
                pos[2][0] += min_x;
                pos[1][0] -= 1. - max_x;
                pos[3][0] -= 1. - max_x;

                pos[0][1] += min_y;
                pos[1][1] += min_y;
                pos[2][1] += min_y;
                pos[3][1] += min_y;

                pos[0][2] += min_z;
                pos[1][2] += min_z;
                pos[2][2] -= 1. - max_z;
                pos[3][2] -= 1. - max_z;
            }
            Direction::North => {
                pos[0][0] += min_x;
                pos[1][0] += min_x;
                pos[2][0] += min_x;
                pos[3][0] += min_x;

                pos[0][1] += min_y;
                pos[1][1] += min_y;
                pos[2][1] -= 1. - max_y;
                pos[3][1] -= 1. - max_y;

                pos[0][2] += min_z;
                pos[3][2] += min_z;
                pos[1][2] -= 1. - max_z;
                pos[2][2] -= 1. - max_z;
            }
            Direction::South => {
                pos[0][0] += min_x;
                pos[1][0] += min_x;
                pos[2][0] += min_x;
                pos[3][0] += min_x;

                pos[0][1] += min_y;
                pos[1][1] += min_y;
                pos[2][1] -= 1. - max_y;
                pos[3][1] -= 1. - max_y;

                pos[0][2] -= 1. - max_z;
                pos[3][2] -= 1. - max_z;
                pos[1][2] += min_z;
                pos[2][2] += min_z;
            }
            Direction::East => {
                pos[0][0] += min_x;
                pos[2][0] += min_x;
                pos[1][0] -= 1. - max_x;
                pos[3][0] -= 1. - max_x;

                pos[0][1] += min_y;
                pos[1][1] += min_y;
                pos[2][1] -= 1. - max_y;
                pos[3][1] -= 1. - max_y;

                pos[0][2] += min_z;
                pos[1][2] += min_z;
                pos[2][2] += min_z;
                pos[3][2] += min_z;
            }
            Direction::West => {
                pos[0][0] += min_x;
                pos[2][0] += min_x;
                pos[1][0] -= 1. - max_x;
                pos[3][0] -= 1. - max_x;

                pos[0][1] += min_y;
                pos[1][1] += min_y;
                pos[2][1] -= 1. - max_y;
                pos[3][1] -= 1. - max_y;

                pos[0][2] -= 1. - max_z;
                pos[1][2] -= 1. - max_z;
                pos[2][2] -= 1. - max_z;
                pos[3][2] -= 1. - max_z;
            }
        }
    }
}
