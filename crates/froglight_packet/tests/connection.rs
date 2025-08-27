//! TODO

extern crate facet_core as facet;

use facet_macros::Facet;
use facet_minecraft::AssertProtocol;
use froglight_packet::network::{
    ConnConfig,
    protocol::{receive_type, send_type},
};
use futures_lite::{AsyncReadExt, AsyncWriteExt, future::block_on, io::Cursor};

#[test]
fn connection_read() {
    const EXAMPLES: &[u8] = &[2, 0, 0, 2, 4, 4, 3, 128, 1, 8];
    let mut cursor = Cursor::new(EXAMPLES);

    let mut buffer = Vec::new();
    let mut scratch = Vec::new();
    let config = ConnConfig::new();

    assert_eq!(
        PlayerStatisticType { statistic_id: 0, kind: PlayerStatisticKind::Mined },
        read(&mut buffer, &mut scratch, &config, &mut cursor)
    );
    assert_eq!(
        PlayerStatisticType { statistic_id: 4, kind: PlayerStatisticKind::PickedUp },
        read(&mut buffer, &mut scratch, &config, &mut cursor)
    );
    assert_eq!(
        PlayerStatisticType { statistic_id: 128, kind: PlayerStatisticKind::Custom },
        read(&mut buffer, &mut scratch, &config, &mut cursor)
    );
}

#[test]
fn connection_write() {
    const EXAMPLES: &[PlayerStatisticType] = &[
        PlayerStatisticType { statistic_id: 0, kind: PlayerStatisticKind::Mined },
        PlayerStatisticType { statistic_id: 4, kind: PlayerStatisticKind::PickedUp },
        PlayerStatisticType { statistic_id: 128, kind: PlayerStatisticKind::Custom },
    ];

    let mut cursor = Cursor::new(Vec::new());

    let mut buffer = Vec::new();
    let mut scratch = Vec::new();
    let config = ConnConfig::new();

    for example in EXAMPLES {
        write(example, &mut buffer, &mut scratch, &config, &mut cursor);
    }

    assert_eq!(cursor.into_inner(), [2, 0, 0, 2, 4, 4, 3, 128, 1, 8]);
}

// -------------------------------------------------------------------------------------------------

fn read<'facet, T: AssertProtocol<'facet>, R: AsyncReadExt + Unpin + ?Sized>(
    buffer: &mut Vec<u8>,
    scratch: &mut Vec<u8>,
    config: &ConnConfig,
    reader: &mut R,
) -> T {
    block_on(receive_type::<T, R>(buffer, scratch, config, reader)).unwrap()
}

fn write<'facet, T: AssertProtocol<'facet>, W: AsyncWriteExt + Unpin + ?Sized>(
    value: &T,
    buffer: &mut Vec<u8>,
    scratch: &mut Vec<u8>,
    config: &ConnConfig,
    writer: &mut W,
) {
    block_on(send_type::<T, W>(value, buffer, scratch, config, writer)).unwrap();
}

#[derive(Debug, PartialEq, Eq, Facet)]
struct PlayerStatisticType {
    #[facet(var)]
    statistic_id: u32,
    kind: PlayerStatisticKind,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Facet)]
#[expect(dead_code, reason = "Not every variant is used in tests")]
enum PlayerStatisticKind {
    Mined,
    Crafted,
    Used,
    Broken,
    PickedUp,
    Dropped,
    Killed,
    KilledBy,
    Custom,
}
