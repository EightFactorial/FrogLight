use compact_str::CompactString;
use derive_more::{Deref, DerefMut, From, Into};
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, From, Into, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [4, 74, 65, 73, 74])]
pub struct ServerboundRenameItemPacket(CompactString);
