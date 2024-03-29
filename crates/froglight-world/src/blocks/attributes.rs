//! All block attributes
//!
//! These are set on blocks to define their state and behavior.
#![allow(missing_docs)]

use bevy_app::App;
use froglight_macros::frog_block_attributes;

frog_block_attributes! {
    Waterlogged(pub bool),
    Thickness {
        #[default]
        TipMerge,
        Tip,
        Frustum,
        Middle,
        Base,
    },
    Slot2Occupied(pub bool),
    Attachment {
        #[default]
        Floor,
        Ceiling,
        SingleWall,
        DoubleWall,
    },
    Powered(pub bool),
    Triggered(pub bool),
    Bites {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
    },
    Extended(pub bool),
    Conditional(pub bool),
    Disarmed(pub bool),
    DoubleBlockHalf {
        #[default]
        Upper,
        Lower,
    },
    Age1 {
        #[default]
        _0,
        _1,
    },
    Locked(pub bool),
    Moisture {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
    },
    Tilt {
        #[default]
        None,
        Unstable,
        Partial,
        Full,
    },
    Slot3Occupied(pub bool),
    Level18 {
        #[default]
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
    },
    Cracked(pub bool),
    Berries(pub bool),
    ComparatorMode {
        #[default]
        Compare,
        Subtract,
    },
    Age2 {
        #[default]
        _0,
        _1,
        _2,
    },
    HoneyLevel {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
    },
    BlockHalf {
        #[default]
        Top,
        Bottom,
    },
    Age4 {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
    },
    Age25 {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
        _9,
        _10,
        _11,
        _12,
        _13,
        _14,
        _15,
        _16,
        _17,
        _18,
        _19,
        _20,
        _21,
        _22,
        _23,
        _24,
        _25,
    },
    Shrieking(pub bool),
    Candles {
        #[default]
        _1,
        _2,
        _3,
        _4,
    },
    Lit(pub bool),
    Distance07 {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
    },
    SouthWireConnection {
        #[default]
        Up,
        Side,
        None,
    },
    CanSummon(pub bool),
    Charges {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
    },
    Slot4Occupied(pub bool),
    PistonType {
        #[default]
        Default,
        Sticky,
    },
    BambooLeaves {
        #[default]
        None,
        Small,
        Large,
    },
    Short(pub bool),
    Occupied(pub bool),
    South(pub bool),
    West(pub bool),
    Persistent(pub bool),
    HorizontalFacing {
        #[default]
        North,
        East,
        South,
        West,
    },
    WestWallShape {
        #[default]
        None,
        Low,
        Tall,
    },
    SignalFire(pub bool),
    Axis {
        #[default]
        X,
        Y,
        Z,
    },
    Age3 {
        #[default]
        _0,
        _1,
        _2,
        _3,
    },
    VerticalDirection {
        #[default]
        Up,
        Down,
    },
    Orientation {
        #[default]
        DownEast,
        DownNorth,
        DownSouth,
        DownWest,
        UpEast,
        UpNorth,
        UpSouth,
        UpWest,
        WestUp,
        EastUp,
        NorthUp,
        SouthUp,
    },
    Up(pub bool),
    Drag(pub bool),
    ChestType {
        #[default]
        Single,
        Left,
        Right,
    },
    StairShape {
        #[default]
        Straight,
        InnerLeft,
        InnerRight,
        OuterLeft,
        OuterRight,
    },
    HasBottle2(pub bool),
    HopperFacing {
        #[default]
        North,
        East,
        South,
        West,
        Down,
    },
    NorthWallShape {
        #[default]
        None,
        Low,
        Tall,
    },
    StraightRailShape {
        #[default]
        NorthSouth,
        EastWest,
        AscendingEast,
        AscendingWest,
        AscendingNorth,
        AscendingSouth,
        SouthEast,
        SouthWest,
        NorthWest,
        NorthEast,
    },
    Hatch {
        #[default]
        _0,
        _1,
        _2,
    },
    SouthWallShape {
        #[default]
        None,
        Low,
        Tall,
    },
    Attached(pub bool),
    Open(pub bool),
    Snowy(pub bool),
    Inverted(pub bool),
    Level3 {
        #[default]
        _1,
        _2,
        _3,
    },
    HasBottle0(pub bool),
    DoorHinge {
        #[default]
        Left,
        Right,
    },
    Enabled(pub bool),
    North(pub bool),
    Pickles {
        #[default]
        _1,
        _2,
        _3,
        _4,
    },
    Stage {
        #[default]
        _0,
        _1,
    },
    Power {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
        _9,
        _10,
        _11,
        _12,
        _13,
        _14,
        _15,
    },
    FlowerAmount {
        #[default]
        _1,
        _2,
        _3,
        _4,
    },
    Slot0Occupied(pub bool),
    Age5 {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
    },
    Age7 {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
    },
    Delay {
        #[default]
        _1,
        _2,
        _3,
        _4,
    },
    Level15 {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
        _9,
        _10,
        _11,
        _12,
        _13,
        _14,
        _15,
    },
    Layers {
        #[default]
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
    },
    NorthWireConnection {
        #[default]
        Up,
        Side,
        None,
    },
    BlockFace {
        #[default]
        Floor,
        Wall,
        Ceiling,
    },
    Falling(pub bool),
    HorizontalAxis {
        #[default]
        X,
        Y,
        Z,
    },
    East(pub bool),
    Slot1Occupied(pub bool),
    BedPart {
        #[default]
        Head,
        Foot,
    },
    Bloom(pub bool),
    Facing {
        #[default]
        North,
        East,
        South,
        West,
        Up,
        Down,
    },
    SlabType {
        #[default]
        Top,
        Bottom,
        Double,
    },
    SculkSensorPhase {
        #[default]
        Inactive,
        Active,
        Cooldown,
    },
    Rotation {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
        _9,
        _10,
        _11,
        _12,
        _13,
        _14,
        _15,
    },
    Slot5Occupied(pub bool),
    Hanging(pub bool),
    WestWireConnection {
        #[default]
        Up,
        Side,
        None,
    },
    Down(pub bool),
    Note {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
        _9,
        _10,
        _11,
        _12,
        _13,
        _14,
        _15,
        _16,
        _17,
        _18,
        _19,
        _20,
        _21,
        _22,
        _23,
        _24,
    },
    Instrument {
        #[default]
        Harp,
        Basedrum,
        Snare,
        Hat,
        Bass,
        Flute,
        Bell,
        Guitar,
        Chime,
        Xylophone,
        IronXylophone,
        CowBell,
        Didgeridoo,
        Bit,
        Banjo,
        Pling,
        Zombie,
        Skeleton,
        Creeper,
        Dragon,
        WitherSkeleton,
        Piglin,
        CustomHead,
    },
    EastWireConnection {
        #[default]
        Up,
        Side,
        None,
    },
    InWall(pub bool),
    StructureBlockMode {
        #[default]
        Save,
        Load,
        Corner,
        Data,
    },
    Unstable(pub bool),
    RailShape {
        #[default]
        NorthSouth,
        EastWest,
        AscendingEast,
        AscendingWest,
        AscendingNorth,
        AscendingSouth,
        SouthEast,
        SouthWest,
        NorthWest,
        NorthEast,
    },
    Bottom(pub bool),
    Eggs {
        #[default]
        _1,
        _2,
        _3,
        _4,
    },
    Dusted {
        #[default]
        _0,
        _1,
        _2,
        _3,
    },
    Eye(pub bool),
    HasBook(pub bool),
    EastWallShape {
        #[default]
        None,
        Low,
        Tall,
    },
    Age15 {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
        _9,
        _10,
        _11,
        _12,
        _13,
        _14,
        _15,
    },
    HasRecord(pub bool),
    Level8 {
        #[default]
        _0,
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
        _8,
    },
    Distance17 {
        #[default]
        _1,
        _2,
        _3,
        _4,
        _5,
        _6,
        _7,
    },
    HasBottle1(pub bool),
}

#[test]
fn attribute_state_count() {
    use crate::blocks::traits::BlockAttribute;

    assert_eq!(<SnowyAttribute as BlockAttribute>::STATES, 2);
    assert_eq!(<WaterloggedAttribute as BlockAttribute>::STATES, 2);
    assert_eq!(<HasRecordAttribute as BlockAttribute>::STATES, 2);

    assert_eq!(<ThicknessAttribute as BlockAttribute>::STATES, 5);

    assert_eq!(<Age25Attribute as BlockAttribute>::STATES, 26);
    assert_eq!(<Age15Attribute as BlockAttribute>::STATES, 16);
    assert_eq!(<Age7Attribute as BlockAttribute>::STATES, 8);
    assert_eq!(<Age5Attribute as BlockAttribute>::STATES, 6);
    assert_eq!(<Age4Attribute as BlockAttribute>::STATES, 5);
    assert_eq!(<Age3Attribute as BlockAttribute>::STATES, 4);
    assert_eq!(<Age2Attribute as BlockAttribute>::STATES, 3);
    assert_eq!(<Age1Attribute as BlockAttribute>::STATES, 2);
}
