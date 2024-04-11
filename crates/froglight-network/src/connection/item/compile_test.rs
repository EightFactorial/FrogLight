use froglight_protocol::{
    states::{Handshaking, Login, Play, Status},
    versions::v1_20_0::V1_20_0,
};

use super::{Clientbound, Connection};

#[test]
fn v1_20_1_serverbound() {
    Connection::<V1_20_0, Handshaking>::nothing();
    Connection::<V1_20_0, Status>::nothing();
    Connection::<V1_20_0, Login>::nothing();
    Connection::<V1_20_0, Play>::nothing();
}

#[test]
fn v1_20_1_clientbound() {
    Connection::<V1_20_0, Handshaking, Clientbound>::nothing();
    Connection::<V1_20_0, Status, Clientbound>::nothing();
    Connection::<V1_20_0, Login, Clientbound>::nothing();
    Connection::<V1_20_0, Play, Clientbound>::nothing();
}
