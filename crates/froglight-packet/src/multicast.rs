//! [`LanBroadcast`]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
#[cfg(feature = "io")]
use froglight_io::prelude::*;
use smol_str::SmolStr;

/// A LAN broadcast message that advertises a server on the local network.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
pub struct LanBroadcast {
    /// The MOTD of the LAN server.
    pub motd: SmolStr,
    /// The port on which the LAN server is running.
    pub port: u16,
}

/// An error that can occur while reading a [`LanBroadcast`].
#[derive(Debug, thiserror::Error)]
pub enum LanBroadcastError {
    /// Missing the `MOTD` tag in the LAN broadcast message.
    #[error("Missing \"MOTD\" tag in LanBroadcast")]
    MissingMotd,
    /// Missing the `AD` tag in the LAN broadcast message.
    #[error("Missing \"AD\" tag in LanBroadcast")]
    MissingPort,
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl FrogRead for LanBroadcast {
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        let content = String::frog_read(buffer)?;

        // Parse the `MOTD` tag
        let Some((_, motd)) = content.split_once("[MOTD]") else {
            return Err(ReadError::Other(Box::new(LanBroadcastError::MissingMotd)));
        };
        let Some((motd, _)) = motd.split_once("[/MOTD]") else {
            return Err(ReadError::Other(Box::new(LanBroadcastError::MissingMotd)));
        };

        // Parse the `AD` tag
        let Some((_, port)) = content.split_once("[AD]") else {
            return Err(ReadError::Other(Box::new(LanBroadcastError::MissingPort)));
        };
        let Some((port, _)) = port.split_once("[/AD]") else {
            return Err(ReadError::Other(Box::new(LanBroadcastError::MissingPort)));
        };

        Ok(LanBroadcast {
            motd: SmolStr::new(motd),
            port: port.parse::<u16>().map_err(|err| ReadError::Other(Box::new(err)))?,
        })
    }
}

#[cfg(feature = "io")]
impl FrogWrite for LanBroadcast {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        format!("[MOTD]{}[/MOTD][AD]{}[/AD]", self.motd, self.port).frog_write(buffer)
    }

    fn frog_len(&self) -> usize {
        format!("[MOTD]{}[/MOTD][AD]{}[/AD]", self.motd, self.port).frog_len()
    }
}
