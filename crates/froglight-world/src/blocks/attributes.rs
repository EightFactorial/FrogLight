//! All block attributes
//!
//! These are set on blocks to define their state and behavior.
#![allow(missing_docs)]

use bevy_app::App;
use bevy_reflect::Reflect;

#[doc(hidden)]
#[allow(clippy::too_many_lines)]
pub(super) fn register(app: &mut App) {
    app.register_type::<WaterloggedAttribute>()
        .register_type::<ThicknessAttribute>()
        .register_type::<Slot2OccupiedAttribute>()
        .register_type::<AttachmentAttribute>()
        .register_type::<PoweredAttribute>()
        .register_type::<TriggeredAttribute>()
        .register_type::<BitesAttribute>()
        .register_type::<ExtendedAttribute>()
        .register_type::<ConditionalAttribute>()
        .register_type::<DisarmedAttribute>()
        .register_type::<DoubleBlockHalfAttribute>()
        .register_type::<Age1Attribute>()
        .register_type::<LockedAttribute>()
        .register_type::<MoistureAttribute>()
        .register_type::<TiltAttribute>()
        .register_type::<Slot3OccupiedAttribute>()
        .register_type::<Level18Attribute>()
        .register_type::<CrackedAttribute>()
        .register_type::<BerriesAttribute>()
        .register_type::<ComparatorModeAttribute>()
        .register_type::<Age2Attribute>()
        .register_type::<HoneyLevelAttribute>()
        .register_type::<BlockHalfAttribute>()
        .register_type::<Age4Attribute>()
        .register_type::<Age25Attribute>()
        .register_type::<ShriekingAttribute>()
        .register_type::<CandlesAttribute>()
        .register_type::<LitAttribute>()
        .register_type::<Distance07Attribute>()
        .register_type::<SouthWireConnectionAttribute>()
        .register_type::<CanSummonAttribute>()
        .register_type::<ChargesAttribute>()
        .register_type::<Slot4OccupiedAttribute>()
        .register_type::<PistonTypeAttribute>()
        .register_type::<BambooLeavesAttribute>()
        .register_type::<ShortAttribute>()
        .register_type::<OccupiedAttribute>()
        .register_type::<SouthAttribute>()
        .register_type::<WestAttribute>()
        .register_type::<PersistentAttribute>()
        .register_type::<HorizontalFacingAttribute>()
        .register_type::<WestWallShapeAttribute>()
        .register_type::<SignalFireAttribute>()
        .register_type::<AxisAttribute>()
        .register_type::<Age3Attribute>()
        .register_type::<MoistureAttribute>()
        .register_type::<VerticalDirectionAttribute>()
        .register_type::<OrientationAttribute>()
        .register_type::<UpAttribute>()
        .register_type::<DragAttribute>()
        .register_type::<ChestTypeAttribute>()
        .register_type::<StairShapeAttribute>()
        .register_type::<HasBottle2Attribute>()
        .register_type::<HopperFacingAttribute>()
        .register_type::<NorthWallShapeAttribute>()
        .register_type::<StraightRailShapeAttribute>()
        .register_type::<HatchAttribute>()
        .register_type::<SouthWallShapeAttribute>()
        .register_type::<AttachedAttribute>()
        .register_type::<OpenAttribute>()
        .register_type::<SnowyAttribute>()
        .register_type::<InvertedAttribute>()
        .register_type::<Level3Attribute>()
        .register_type::<HasBottle0Attribute>()
        .register_type::<DoorHingeAttribute>()
        .register_type::<EnabledAttribute>()
        .register_type::<NorthAttribute>()
        .register_type::<PicklesAttribute>()
        .register_type::<StageAttribute>()
        .register_type::<PowerAttribute>()
        .register_type::<FlowerAmountAttribute>()
        .register_type::<Slot0OccupiedAttribute>()
        .register_type::<Age5Attribute>()
        .register_type::<Age7Attribute>()
        .register_type::<DelayAttribute>()
        .register_type::<Level15Attribute>()
        .register_type::<LayersAttribute>()
        .register_type::<NorthWireConnectionAttribute>()
        .register_type::<BlockFaceAttribute>()
        .register_type::<FallingAttribute>()
        .register_type::<HorizontalAxisAttribute>()
        .register_type::<EastAttribute>()
        .register_type::<Slot1OccupiedAttribute>()
        .register_type::<BedPartAttribute>()
        .register_type::<BloomAttribute>()
        .register_type::<FacingAttribute>()
        .register_type::<SlabTypeAttribute>()
        .register_type::<SculkSensorPhaseAttribute>()
        .register_type::<RotationAttribute>()
        .register_type::<Slot5OccupiedAttribute>()
        .register_type::<HangingAttribute>()
        .register_type::<WestWireConnectionAttribute>()
        .register_type::<DownAttribute>()
        .register_type::<NoteAttribute>()
        .register_type::<InstrumentAttribute>()
        .register_type::<EastWireConnectionAttribute>()
        .register_type::<InWallAttribute>()
        .register_type::<StructureBlockModeAttribute>()
        .register_type::<UnstableAttribute>()
        .register_type::<RailShapeAttribute>()
        .register_type::<BottomAttribute>()
        .register_type::<EggsAttribute>()
        .register_type::<DustedAttribute>()
        .register_type::<EyeAttribute>()
        .register_type::<HasBookAttribute>()
        .register_type::<EastWallShapeAttribute>()
        .register_type::<Age15Attribute>()
        .register_type::<HasRecordAttribute>()
        .register_type::<Level8Attribute>()
        .register_type::<Distance17Attribute>()
        .register_type::<HasBottle1Attribute>();
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct WaterloggedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ThicknessAttribute {
    #[default]
    TipMerge,
    Tip,
    Frustum,
    Middle,
    Base,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Slot2OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum AttachmentAttribute {
    #[default]
    Floor,
    Ceiling,
    SingleWall,
    DoubleWall,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct PoweredAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct TriggeredAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum BitesAttribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct ExtendedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct ConditionalAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct DisarmedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DoubleBlockHalfAttribute {
    #[default]
    Upper,
    Lower,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Age1Attribute {
    #[default]
    _0,
    _1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct LockedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum MoistureAttribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum TiltAttribute {
    #[default]
    None,
    Unstable,
    Partial,
    Full,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Slot3OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Level18Attribute {
    #[default]
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct CrackedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BerriesAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ComparatorModeAttribute {
    #[default]
    Compare,
    Subtract,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Age2Attribute {
    #[default]
    _0,
    _1,
    _2,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum HoneyLevelAttribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum BlockHalfAttribute {
    #[default]
    Top,
    Bottom,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Age4Attribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Age25Attribute {
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
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct ShriekingAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum CandlesAttribute {
    #[default]
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct LitAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Distance07Attribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum SouthWireConnectionAttribute {
    #[default]
    Up,
    Side,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct CanSummonAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ChargesAttribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Slot4OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum PistonTypeAttribute {
    #[default]
    Default,
    Sticky,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum BambooLeavesAttribute {
    #[default]
    None,
    Small,
    Large,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct ShortAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct SouthAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct WestAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct PersistentAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum HorizontalFacingAttribute {
    #[default]
    North,
    East,
    South,
    West,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum WestWallShapeAttribute {
    #[default]
    None,
    Low,
    Tall,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct SignalFireAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum AxisAttribute {
    #[default]
    X,
    Y,
    Z,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Age3Attribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum VerticalDirectionAttribute {
    #[default]
    Up,
    Down,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum OrientationAttribute {
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
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct UpAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct DragAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ChestTypeAttribute {
    #[default]
    Single,
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum StairShapeAttribute {
    #[default]
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct HasBottle2Attribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum HopperFacingAttribute {
    #[default]
    North,
    East,
    South,
    West,
    Down,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum NorthWallShapeAttribute {
    #[default]
    None,
    Low,
    Tall,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum StraightRailShapeAttribute {
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
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum HatchAttribute {
    #[default]
    _0,
    _1,
    _2,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum SouthWallShapeAttribute {
    #[default]
    None,
    Low,
    Tall,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct AttachedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct OpenAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct SnowyAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct InvertedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Level3Attribute {
    #[default]
    _1,
    _2,
    _3,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct HasBottle0Attribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DoorHingeAttribute {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct EnabledAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct NorthAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum PicklesAttribute {
    #[default]
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum StageAttribute {
    #[default]
    _0,
    _1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum PowerAttribute {
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
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum FlowerAmountAttribute {
    #[default]
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Slot0OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Age5Attribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Age7Attribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DelayAttribute {
    #[default]
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Level15Attribute {
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
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum LayersAttribute {
    #[default]
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum NorthWireConnectionAttribute {
    #[default]
    Up,
    Side,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum BlockFaceAttribute {
    #[default]
    Floor,
    Wall,
    Ceiling,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct FallingAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum HorizontalAxisAttribute {
    #[default]
    X,
    Y,
    Z,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct EastAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Slot1OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum BedPartAttribute {
    #[default]
    Head,
    Foot,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BloomAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum FacingAttribute {
    #[default]
    North,
    East,
    South,
    West,
    Up,
    Down,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum SlabTypeAttribute {
    #[default]
    Top,
    Bottom,
    Double,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum SculkSensorPhaseAttribute {
    #[default]
    Inactive,
    Active,
    Cooldown,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum RotationAttribute {
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
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Slot5OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct HangingAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum WestWireConnectionAttribute {
    #[default]
    Up,
    Side,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct DownAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum NoteAttribute {
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
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum InstrumentAttribute {
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
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum EastWireConnectionAttribute {
    #[default]
    Up,
    Side,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct InWallAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum StructureBlockModeAttribute {
    #[default]
    Save,
    Load,
    Corner,
    Data,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct UnstableAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum RailShapeAttribute {
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
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BottomAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum EggsAttribute {
    #[default]
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DustedAttribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct EyeAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct HasBookAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum EastWallShapeAttribute {
    #[default]
    None,
    Low,
    Tall,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Age15Attribute {
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
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct HasRecordAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Level8Attribute {
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
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Distance17Attribute {
    #[default]
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct HasBottle1Attribute(pub bool);
