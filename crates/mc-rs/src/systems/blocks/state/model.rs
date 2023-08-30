use bevy::{
    math::Vec3A,
    prelude::{Handle, Mesh},
    render::primitives::Aabb,
};

#[derive(Debug, Clone)]
pub enum BlockModel {
    None,
    Standard,
    Simple { min: Vec3A, max: Vec3A },
    Custom { collision: Aabb, mesh: Handle<Mesh> },
}
