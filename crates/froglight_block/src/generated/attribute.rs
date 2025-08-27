//! @generated attribute types
#![expect(missing_docs, non_camel_case_types, reason = "Automatically @generated")]

use crate::attribute::BlockAttribute;

generate! {
    @attrs
    AgeInt0To1 { _0: "0" _1: "1" }
    AgeInt0To2 { _0: "0" _1: "1" _2: "2" }
    AgeInt0To3 { _0: "0" _1: "1" _2: "2" _3: "3" }
    AgeInt0To4 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" }
    AgeInt0To5 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" }
    AgeInt0To7 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" _6: "6" _7: "7" }
    AgeInt0To15 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" _6: "6" _7: "7" _8: "8" _9: "9" _10: "10" _11: "11" _12: "12" _13: "13" _14: "14" _15: "15" }
    AgeInt0To25 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" _6: "6" _7: "7" _8: "8" _9: "9" _10: "10" _11: "11" _12: "12" _13: "13" _14: "14" _15: "15" _16: "16" _17: "17" _18: "18" _19: "19" _20: "20" _21: "21" _22: "22" _23: "23" _24: "24" _25: "25" }
    AttachedBool(bool)
    AttachmentEnum { Floor: "floor" Ceiling: "ceiling" SingleWall: "single_wall" DoubleWall: "double_wall" }
    AxisEnum_X_Y_Z { X: "x" Y: "y" Z: "z" }
    AxisEnum_X_Z { X: "x" Z: "z" }
    BerriesBool(bool)
    BitesInt0To6 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" _6: "6" }
    BloomBool(bool)
    BottomBool(bool)
    CanSummonBool(bool)
    CandlesInt1To4 { _1: "1" _2: "2" _3: "3" _4: "4" }
    ChargesInt0To4 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" }
    ConditionalBool(bool)
    CrackedBool(bool)
    CraftingBool(bool)
    CreakingHeartStateEnum { Uprooted: "uprooted" Dormant: "dormant" Awake: "awake" }
    DelayInt1To4 { _1: "1" _2: "2" _3: "3" _4: "4" }
    DisarmedBool(bool)
    DistanceInt0To7 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" _6: "6" _7: "7" }
    DistanceInt1To7 { _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" _6: "6" _7: "7" }
    DownBool(bool)
    DragBool(bool)
    DustedInt0To3 { _0: "0" _1: "1" _2: "2" _3: "3" }
    EastEnum_None_Low_Tall { None: "none" Low: "low" Tall: "tall" }
    EastBool(bool)
    EastEnum_Up_Side_None { Up: "up" Side: "side" None: "none" }
    EggsInt1To4 { _1: "1" _2: "2" _3: "3" _4: "4" }
    EnabledBool(bool)
    ExtendedBool(bool)
    EyeBool(bool)
    FaceEnum { Floor: "floor" Wall: "wall" Ceiling: "ceiling" }
    FacingEnum_Down_North_South_West_East { Down: "down" North: "north" South: "south" West: "west" East: "east" }
    FacingEnum_North_East_South_West_Up_Down { North: "north" East: "east" South: "south" West: "west" Up: "up" Down: "down" }
    FacingEnum_North_South_West_East { North: "north" South: "south" West: "west" East: "east" }
    FlowerAmountInt1To4 { _1: "1" _2: "2" _3: "3" _4: "4" }
    HalfEnum_Top_Bottom { Top: "top" Bottom: "bottom" }
    HalfEnum_Upper_Lower { Upper: "upper" Lower: "lower" }
    HangingBool(bool)
    HasBookBool(bool)
    HasBottle0Bool(bool)
    HasBottle1Bool(bool)
    HasBottle2Bool(bool)
    HasRecordBool(bool)
    HatchInt0To2 { _0: "0" _1: "1" _2: "2" }
    HingeEnum { Left: "left" Right: "right" }
    HoneyLevelInt0To5 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" }
    HydrationInt0To3 { _0: "0" _1: "1" _2: "2" _3: "3" }
    InWallBool(bool)
    InstrumentEnum { Harp: "harp" Basedrum: "basedrum" Snare: "snare" Hat: "hat" Bass: "bass" Flute: "flute" Bell: "bell" Guitar: "guitar" Chime: "chime" Xylophone: "xylophone" IronXylophone: "iron_xylophone" CowBell: "cow_bell" Didgeridoo: "didgeridoo" Bit: "bit" Banjo: "banjo" Pling: "pling" Zombie: "zombie" Skeleton: "skeleton" Creeper: "creeper" Dragon: "dragon" WitherSkeleton: "wither_skeleton" Piglin: "piglin" CustomHead: "custom_head" }
    InvertedBool(bool)
    LayersInt1To8 { _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" _6: "6" _7: "7" _8: "8" }
    LeavesEnum { None: "none" Small: "small" Large: "large" }
    LevelInt0To8 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" _6: "6" _7: "7" _8: "8" }
    LevelInt0To15 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" _6: "6" _7: "7" _8: "8" _9: "9" _10: "10" _11: "11" _12: "12" _13: "13" _14: "14" _15: "15" }
    LevelInt1To3 { _1: "1" _2: "2" _3: "3" }
    LitBool(bool)
    LockedBool(bool)
    ModeEnum_Compare_Subtract { Compare: "compare" Subtract: "subtract" }
    ModeEnum_Save_Load_Corner_Data { Save: "save" Load: "load" Corner: "corner" Data: "data" }
    ModeEnum_Start_Log_Fail_Accept { Start: "start" Log: "log" Fail: "fail" Accept: "accept" }
    MoistureInt0To7 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" _6: "6" _7: "7" }
    NaturalBool(bool)
    NorthEnum_None_Low_Tall { None: "none" Low: "low" Tall: "tall" }
    NorthBool(bool)
    NorthEnum_Up_Side_None { Up: "up" Side: "side" None: "none" }
    NoteInt0To24 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" _6: "6" _7: "7" _8: "8" _9: "9" _10: "10" _11: "11" _12: "12" _13: "13" _14: "14" _15: "15" _16: "16" _17: "17" _18: "18" _19: "19" _20: "20" _21: "21" _22: "22" _23: "23" _24: "24" }
    OccupiedBool(bool)
    OminousBool(bool)
    OpenBool(bool)
    OrientationEnum { DownEast: "down_east" DownNorth: "down_north" DownSouth: "down_south" DownWest: "down_west" UpEast: "up_east" UpNorth: "up_north" UpSouth: "up_south" UpWest: "up_west" WestUp: "west_up" EastUp: "east_up" NorthUp: "north_up" SouthUp: "south_up" }
    PartEnum { Head: "head" Foot: "foot" }
    PersistentBool(bool)
    PicklesInt1To4 { _1: "1" _2: "2" _3: "3" _4: "4" }
    PowerInt0To15 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" _6: "6" _7: "7" _8: "8" _9: "9" _10: "10" _11: "11" _12: "12" _13: "13" _14: "14" _15: "15" }
    PoweredBool(bool)
    RotationInt0To15 { _0: "0" _1: "1" _2: "2" _3: "3" _4: "4" _5: "5" _6: "6" _7: "7" _8: "8" _9: "9" _10: "10" _11: "11" _12: "12" _13: "13" _14: "14" _15: "15" }
    SculkSensorPhaseEnum { Inactive: "inactive" Active: "active" Cooldown: "cooldown" }
    SegmentAmountInt1To4 { _1: "1" _2: "2" _3: "3" _4: "4" }
    ShapeEnum_NorthSouth_EastWest_AscendingEast_AscendingWest_AscendingNorth_AscendingSouth { NorthSouth: "north_south" EastWest: "east_west" AscendingEast: "ascending_east" AscendingWest: "ascending_west" AscendingNorth: "ascending_north" AscendingSouth: "ascending_south" }
    ShapeEnum_NorthSouth_EastWest_AscendingEast_AscendingWest_AscendingNorth_AscendingSouth_SouthEast_SouthWest_NorthWest_NorthEast { NorthSouth: "north_south" EastWest: "east_west" AscendingEast: "ascending_east" AscendingWest: "ascending_west" AscendingNorth: "ascending_north" AscendingSouth: "ascending_south" SouthEast: "south_east" SouthWest: "south_west" NorthWest: "north_west" NorthEast: "north_east" }
    ShapeEnum_Straight_InnerLeft_InnerRight_OuterLeft_OuterRight { Straight: "straight" InnerLeft: "inner_left" InnerRight: "inner_right" OuterLeft: "outer_left" OuterRight: "outer_right" }
    ShortBool(bool)
    ShriekingBool(bool)
    SignalFireBool(bool)
    Slot0OccupiedBool(bool)
    Slot1OccupiedBool(bool)
    Slot2OccupiedBool(bool)
    Slot3OccupiedBool(bool)
    Slot4OccupiedBool(bool)
    Slot5OccupiedBool(bool)
    SnowyBool(bool)
    SouthEnum_None_Low_Tall { None: "none" Low: "low" Tall: "tall" }
    SouthBool(bool)
    SouthEnum_Up_Side_None { Up: "up" Side: "side" None: "none" }
    StageInt0To1 { _0: "0" _1: "1" }
    ThicknessEnum { TipMerge: "tip_merge" Tip: "tip" Frustum: "frustum" Middle: "middle" Base: "base" }
    TiltEnum { None: "none" Unstable: "unstable" Partial: "partial" Full: "full" }
    TipBool(bool)
    TrialSpawnerStateEnum { Inactive: "inactive" WaitingForPlayers: "waiting_for_players" Active: "active" WaitingForRewardEjection: "waiting_for_reward_ejection" EjectingReward: "ejecting_reward" Cooldown: "cooldown" }
    TriggeredBool(bool)
    TypeEnum_Normal_Sticky { Normal: "normal" Sticky: "sticky" }
    TypeEnum_Single_Left_Right { Single: "single" Left: "left" Right: "right" }
    TypeEnum_Top_Bottom_Double { Top: "top" Bottom: "bottom" Double: "double" }
    UnstableBool(bool)
    UpBool(bool)
    VaultStateEnum { Inactive: "inactive" Active: "active" Unlocking: "unlocking" Ejecting: "ejecting" }
    VerticalDirectionEnum { Up: "up" Down: "down" }
    WaterloggedBool(bool)
    WestEnum_None_Low_Tall { None: "none" Low: "low" Tall: "tall" }
    WestBool(bool)
    WestEnum_Up_Side_None { Up: "up" Side: "side" None: "none" }
}
