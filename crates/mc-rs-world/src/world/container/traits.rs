use std::{fmt::Debug, io::Cursor};

use bitvec::vec::BitVec;
use futures_lite::Future;
use mc_rs_protocol::buffer::Decode;

use crate::world::{palette::Palette, tasks::ChunkDecodeError};

use super::Container;

/// A trait for [`Container`] types.
pub trait ContainerType: Sized + Debug + Default {
    fn decode_container(
        cursor: &mut Cursor<&[u8]>,
    ) -> impl Future<Output = Result<Container<Self>, ChunkDecodeError>> {
        async {
            // Get the bits per block
            let bits =
                usize::from(u8::decode(cursor).map_err(|_| ChunkDecodeError::InvalidContainer)?);

            Ok(Container::<Self> {
                // Decode the palette
                palette: Palette::decode_palette::<Self>(bits, cursor).await?,
                // Decode the data
                data: BitVec::try_from_vec(
                    Vec::<u64>::decode(cursor).map_err(|_| ChunkDecodeError::InvalidContainer)?,
                )
                .map_err(|_| ChunkDecodeError::InvalidContainer)?,
                // Construct the container
                bits,
                _phantom: std::marker::PhantomData,
            })
        }
    }

    /// Get the [`Palette`] type for the given bits per block.
    fn palette_type(bits: usize) -> Palette;
}
