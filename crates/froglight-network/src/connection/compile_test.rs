use froglight_protocol::{
    states::{Handshaking, Login, Play, Status},
    versions::v1_20_0::V1_20_0,
};

use super::{Clientbound, Connection};

#[test]
fn v1_20_0_clientbound() {
    Connection::<V1_20_0, Handshaking, Clientbound>::nothing();
    Connection::<V1_20_0, Status, Clientbound>::nothing();
    Connection::<V1_20_0, Login, Clientbound>::nothing();
    Connection::<V1_20_0, Play, Clientbound>::nothing();
}
#[test]
fn v1_20_0_serverbound() {
    Connection::<V1_20_0, Handshaking>::nothing();
    Connection::<V1_20_0, Status>::nothing();
    Connection::<V1_20_0, Login>::nothing();
    Connection::<V1_20_0, Play>::nothing();
}

// #[test]
// fn v1_20_2_clientbound() {
//     Connection::<V1_20_2, Handshaking, Clientbound>::nothing();
//     Connection::<V1_20_2, Status, Clientbound>::nothing();
//     Connection::<V1_20_2, Login, Clientbound>::nothing();
//     Connection::<V1_20_2, Play, Clientbound>::nothing();
// }

// #[test]
// fn v1_20_2_serverbound() {
//     Connection::<V1_20_2, Handshaking>::nothing();
//     Connection::<V1_20_2, Status>::nothing();
//     Connection::<V1_20_2, Login>::nothing();
//     Connection::<V1_20_2, Play>::nothing();
// }

// #[test]
// fn v1_20_3_clientbound() {
//     Connection::<V1_20_3, Handshaking, Clientbound>::nothing();
//     Connection::<V1_20_3, Status, Clientbound>::nothing();
//     Connection::<V1_20_3, Login, Clientbound>::nothing();
//     Connection::<V1_20_3, Play, Clientbound>::nothing();
// }

// #[test]
// fn v1_20_3_serverbound() {
//     Connection::<V1_20_3, Handshaking>::nothing();
//     Connection::<V1_20_3, Status>::nothing();
//     Connection::<V1_20_3, Login>::nothing();
//     Connection::<V1_20_3, Play>::nothing();
// }
