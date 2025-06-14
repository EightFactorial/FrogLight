use froglight_io::prelude::*;

use crate::{player::profile::ProfileResponse, prelude::PlayerProfile};

impl FrogRead for PlayerProfile {
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        Self::try_from(ProfileResponse::frog_read(buffer)?).map_err(ReadError::Json)
    }
}
impl FrogWrite for PlayerProfile {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        ProfileResponse::try_from(self).map_err(WriteError::Json)?.frog_write(buffer)
    }

    fn frog_len(&self) -> usize { ProfileResponse::try_from(self).unwrap().frog_len() }
}
