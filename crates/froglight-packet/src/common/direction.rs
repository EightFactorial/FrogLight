//! TODO: Fix enum variant ordering
#![expect(dead_code, unreachable_pub)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    North,
    South,
    West,
    East,
}

impl Direction {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerticalDirection {
    Up,
    Down,
}

impl VerticalDirection {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HorizontalDirection {
    North,
    South,
    West,
    East,
}

impl HorizontalDirection {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignedAxis {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl SignedAxis {}
