use std::{
    any::type_name,
    io::{Cursor, Write},
};

use froglight_common::Direction;

use crate::protocol::{FrogRead, FrogVarRead, FrogVarWrite, FrogWrite, ReadError, WriteError};

impl FrogRead for Direction {
    fn fg_read(buf: &mut Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        match u32::fg_var_read(buf)? {
            0 => Ok(Direction::Up),
            1 => Ok(Direction::Down),
            2 => Ok(Direction::North),
            3 => Ok(Direction::South),
            4 => Ok(Direction::East),
            5 => Ok(Direction::West),
            #[allow(clippy::cast_possible_wrap)]
            unk => Err(ReadError::InvalidEnum(unk as i32, type_name::<Self>())),
        }
    }
}

impl FrogWrite for Direction {
    fn fg_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
        match self {
            Direction::Up => 0u32.fg_var_write(buf),
            Direction::Down => 1u32.fg_var_write(buf),
            Direction::North => 2u32.fg_var_write(buf),
            Direction::South => 3u32.fg_var_write(buf),
            Direction::East => 4u32.fg_var_write(buf),
            Direction::West => 5u32.fg_var_write(buf),
        }
    }
}
