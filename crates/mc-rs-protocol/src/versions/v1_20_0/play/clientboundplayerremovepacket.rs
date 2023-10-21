use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ClientboundPlayerRemovePacket(Vec<Uuid>);
