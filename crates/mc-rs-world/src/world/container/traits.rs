use std::{future::Future, io::Cursor};

use crate::world::{palette::Palette, tasks::ChunkDecodeError};

use super::Container;

pub trait ContainerType: Sized {
    fn decode_container(
        cursor: &mut Cursor<&[u8]>,
    ) -> impl Future<Output = Result<Container<Self>, ChunkDecodeError>>;

    fn palette_type(bits: &u8) -> Palette;
}
