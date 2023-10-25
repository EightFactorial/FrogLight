use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::ShaderType,
};
use bytemuck::{Pod, Zeroable};
use itertools::{EitherOrBoth, Itertools};

pub mod impl_bind;
pub mod impl_mat;

pub(super) fn setup(app: &mut App) { app.add_plugins(MaterialPlugin::<BlockMaterial>::default()); }

pub const MAX_TEXTURE_COUNT: usize = 48;

pub const MAX_ANIMATION_FRAMES: usize = 32;
pub const MAX_ANIMATION_COUNT: usize = 16;

#[derive(Debug, Default, Clone, TypePath, TypeUuid)]
#[uuid = "0059fd0b-5b43-46cc-bd77-c89130562e75"]
pub struct BlockMaterial {
    pub textures: Vec<Handle<Image>>,
    pub animations: Vec<StateAnimation>,
    pub alpha_mode: AlphaMode,
}

impl BlockMaterial {
    pub fn new_opaque(textures: Vec<Handle<Image>>, animations: Vec<StateAnimation>) -> Self {
        Self {
            textures,
            animations,
            alpha_mode: AlphaMode::Opaque,
        }
    }

    pub fn new_blended(textures: Vec<Handle<Image>>, animations: Vec<StateAnimation>) -> Self {
        Self {
            textures,
            animations,
            alpha_mode: AlphaMode::Blend,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, ShaderType, Pod, Zeroable, Reflect)]
#[repr(C)]
pub struct StateAnimation {
    pub frame_time: f32,

    // Animation frame indices
    pub frame_data_0: u32,
    pub frame_data_1: u32,
    pub frame_data_2: u32,
    pub frame_data_3: u32,
    pub frame_data_4: u32,
    pub frame_data_5: u32,
    pub frame_data_6: u32,
    pub frame_data_7: u32,

    _padding_0: u32,
    _padding_1: u32,
    _padding_2: u32,
}

impl StateAnimation {
    pub fn new(frame_time: f32, frame_order: impl IntoIterator<Item = u8>) -> Self {
        #[cfg(feature = "debug")]
        StateAnimation::assert_uniform_compat();

        // Get the order of the frames
        let mut frames = [0u8; MAX_ANIMATION_FRAMES];
        for state in frame_order.into_iter().zip_longest(frames.iter_mut()) {
            match state {
                EitherOrBoth::Both(state, frame) => *frame = state,
                _ => {
                    if state.is_left() {
                        warn!("Animation frame order is longer than the maximum allowed frames");
                    }
                    break;
                }
            }
        }

        // Compress the frame data into 32-bit integers
        let mut frame_data = [0u32; MAX_ANIMATION_FRAMES / 4];
        for (i, frame) in frames.into_iter().enumerate() {
            let shift = (i % 4) * 8;
            frame_data[i / 4] |= u32::from(frame) << shift;
        }

        Self {
            frame_time,

            frame_data_0: frame_data[0],
            frame_data_1: frame_data[1],
            frame_data_2: frame_data[2],
            frame_data_3: frame_data[3],
            frame_data_4: frame_data[4],
            frame_data_5: frame_data[5],
            frame_data_6: frame_data[6],
            frame_data_7: frame_data[7],

            _padding_0: 0,
            _padding_1: 0,
            _padding_2: 0,
        }
    }
}
