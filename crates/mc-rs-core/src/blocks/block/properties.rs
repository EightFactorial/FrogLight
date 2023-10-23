#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockProperties {
    pub hardness: f32,
    pub resistance: f32,
    pub friction: f32,
    pub velocity_multiplier: f32,
    pub jump_velocity_multiplier: f32,
    pub random_ticks: bool,
    pub burnable: bool,
    pub collidable: bool,
    pub opaque: bool,
    pub is_air: bool,
    pub is_fluid: bool,
}

impl Default for BlockProperties {
    fn default() -> Self {
        Self {
            hardness: 0.0,
            resistance: 0.0,
            friction: 0.6,
            velocity_multiplier: 1.0,
            jump_velocity_multiplier: 1.0,
            random_ticks: false,
            burnable: false,
            collidable: true,
            opaque: true,
            is_air: false,
            is_fluid: false,
        }
    }
}
