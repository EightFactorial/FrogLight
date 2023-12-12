#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WaterloggedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ThicknessAttribute {
    #[default]
    TipMerge,
    Tip,
    Frustum,
    Middle,
    Base,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Slot2OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AttachmentAttribute {
    #[default]
    Floor,
    Ceiling,
    SingleWall,
    DoubleWall,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PoweredAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TriggeredAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ExtendedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ConditionalAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DisarmedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DoubleBlockHalfAttribute {
    #[default]
    Upper,
    Lower,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Age1Attribute {
    #[default]
    _0,
    _1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LockedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TiltAttribute {
    #[default]
    None,
    Unstable,
    Partial,
    Full,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Slot3OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CrackedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BerriesAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ComparatorModeAttribute {
    #[default]
    Compare,
    Subtract,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Age2Attribute {
    #[default]
    _0,
    _1,
    _2,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HoneyLevelAttribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BlockHalfAttribute {
    #[default]
    Top,
    Bottom,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Age4Attribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ShriekingAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CandlesAttribute {
    #[default]
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LitAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SouthWireConnectionAttribute {
    #[default]
    Up,
    Side,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CanSummonAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ChargesAttribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Slot4OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PistonTypeAttribute {
    #[default]
    Default,
    Sticky,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BambooLeavesAttribute {
    #[default]
    None,
    Small,
    Large,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ShortAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SouthAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WestAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PersistentAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalFacingAttribute {
    #[default]
    North,
    East,
    South,
    West,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum WestWallShapeAttribute {
    #[default]
    None,
    Low,
    Tall,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SignalFireAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AxisAttribute {
    #[default]
    X,
    Y,
    Z,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Age3Attribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum VerticalDirectionAttribute {
    #[default]
    Up,
    Down,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UpAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DragAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ChestTypeAttribute {
    #[default]
    Single,
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum StairShapeAttribute {
    #[default]
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HasBottle2Attribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HopperFacingAttribute {
    #[default]
    North,
    East,
    South,
    West,
    Down,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NorthWallShapeAttribute {
    #[default]
    None,
    Low,
    Tall,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HatchAttribute {
    #[default]
    _0,
    _1,
    _2,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SouthWallShapeAttribute {
    #[default]
    None,
    Low,
    Tall,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AttachedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OpenAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SnowyAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InvertedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Level3Attribute {
    #[default]
    _1,
    _2,
    _3,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HasBottle0Attribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DoorHingeAttribute {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EnabledAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NorthAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PicklesAttribute {
    #[default]
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum StageAttribute {
    #[default]
    _0,
    _1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FlowerAmountAttribute {
    #[default]
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Slot0OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Age5Attribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DelayAttribute {
    #[default]
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NorthWireConnectionAttribute {
    #[default]
    Up,
    Side,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BlockFaceAttribute {
    #[default]
    Floor,
    Wall,
    Ceiling,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FallingAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAxisAttribute {
    #[default]
    X,
    Y,
    Z,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EastAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Slot1OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BedPartAttribute {
    #[default]
    Head,
    Foot,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BloomAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FacingAttribute {
    #[default]
    North,
    East,
    South,
    West,
    Up,
    Down,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SlabTypeAttribute {
    #[default]
    Top,
    Bottom,
    Double,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SculkSensorPhaseAttribute {
    #[default]
    Inactive,
    Active,
    Cooldown,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Slot5OccupiedAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HangingAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum WestWireConnectionAttribute {
    #[default]
    Up,
    Side,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DownAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EastWireConnectionAttribute {
    #[default]
    Up,
    Side,
    None,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InWallAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum StructureBlockModeAttribute {
    #[default]
    Save,
    Load,
    Corner,
    Data,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UnstableAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BottomAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EggsAttribute {
    #[default]
    _1,
    _2,
    _3,
    _4,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DustedAttribute {
    #[default]
    _0,
    _1,
    _2,
    _3,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EyeAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HasBookAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EastWallShapeAttribute {
    #[default]
    None,
    Low,
    Tall,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HasRecordAttribute(pub bool);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HasBottle1Attribute(pub bool);

