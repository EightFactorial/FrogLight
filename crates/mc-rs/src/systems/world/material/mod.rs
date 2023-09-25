use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::ShaderType,
};
use bytemuck::{Pod, Zeroable};

mod impl_bind;
pub use impl_bind::*;

mod impl_mat;
pub use impl_mat::*;
use itertools::{EitherOrBoth, Itertools};

pub(super) fn setup(app: &mut App) { app.add_plugins(MaterialPlugin::<BlockMaterial>::default()); }

pub const MAX_TEXTURE_COUNT: usize = 48;
pub const MAX_ANIMATION_COUNT: usize = 32;

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

    // :(
    // pub frame_order: [u32; MAX_ANIMATION_COUNT],
    pub frame_1: u32,
    pub frame_2: u32,
    pub frame_3: u32,
    pub frame_4: u32,
    pub frame_5: u32,
    pub frame_6: u32,
    pub frame_7: u32,
    pub frame_8: u32,
    pub frame_9: u32,
    pub frame_10: u32,
    pub frame_11: u32,
    pub frame_12: u32,
    pub frame_13: u32,
    pub frame_14: u32,
    pub frame_15: u32,
    pub frame_16: u32,
    pub frame_17: u32,
    pub frame_18: u32,
    pub frame_19: u32,
    pub frame_20: u32,
    pub frame_21: u32,
    pub frame_22: u32,
    pub frame_23: u32,
    pub frame_24: u32,
    pub frame_25: u32,
    pub frame_26: u32,
    pub frame_27: u32,
    pub frame_28: u32,
    pub frame_29: u32,
    pub frame_30: u32,
    pub frame_31: u32,
    pub frame_32: u32,

    _padding_0: u32,
    _padding_1: u32,
    _padding_2: u32,
}

impl StateAnimation {
    pub fn new(frame_time: f32, frame_order: impl IntoIterator<Item = u32>) -> Self {
        #[cfg(feature = "debug")]
        StateAnimation::assert_uniform_compat();

        let mut frames = [0u32; MAX_ANIMATION_COUNT];
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

        Self {
            frame_time,
            frame_1: frames[0],
            frame_2: frames[1],
            frame_3: frames[2],
            frame_4: frames[3],
            frame_5: frames[4],
            frame_6: frames[5],
            frame_7: frames[6],
            frame_8: frames[7],
            frame_9: frames[8],
            frame_10: frames[9],
            frame_11: frames[10],
            frame_12: frames[11],
            frame_13: frames[12],
            frame_14: frames[13],
            frame_15: frames[14],
            frame_16: frames[15],
            frame_17: frames[16],
            frame_18: frames[17],
            frame_19: frames[18],
            frame_20: frames[19],
            frame_21: frames[20],
            frame_22: frames[21],
            frame_23: frames[22],
            frame_24: frames[23],
            frame_25: frames[24],
            frame_26: frames[25],
            frame_27: frames[26],
            frame_28: frames[27],
            frame_29: frames[28],
            frame_30: frames[29],
            frame_31: frames[30],
            frame_32: frames[31],
            _padding_0: 0,
            _padding_1: 0,
            _padding_2: 0,
        }
    }
}
