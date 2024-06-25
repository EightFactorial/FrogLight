use froglight_protocol::{
    states::{Handshake, Login, Play, Status},
    versions::v1_21_0::V1_21_0,
};

use super::{Clientbound, Connection};

#[test]
fn v1_21_0_clientbound() {
    Connection::<V1_21_0, Handshake, Clientbound>::nothing();
    Connection::<V1_21_0, Status, Clientbound>::nothing();
    Connection::<V1_21_0, Login, Clientbound>::nothing();
    Connection::<V1_21_0, Play, Clientbound>::nothing();
}
#[test]
fn v1_21_0_serverbound() {
    Connection::<V1_21_0, Handshake>::nothing();
    Connection::<V1_21_0, Status>::nothing();
    Connection::<V1_21_0, Login>::nothing();
    Connection::<V1_21_0, Play>::nothing();
}
