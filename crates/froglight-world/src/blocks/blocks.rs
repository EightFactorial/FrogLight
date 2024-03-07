#![allow(missing_docs)]

use bevy_reflect::Reflect;

use super::attributes::SnowyAttribute;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockAir;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockStone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockGranite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockPolishedGranite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockDiorite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockPolishedDiorite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockAndesite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockPolishedAndesite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockGrassBlock {
    pub snowy: SnowyAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockDirt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockCoarseDirt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockPodzol {
    pub snowy: SnowyAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BlockCobblestone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakPlanks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSprucePlanks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchPlanks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJunglePlanks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaPlanks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryPlanks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakPlanks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangrovePlanks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooPlanks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooMosaic;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakSapling {
//     pub stage: StageAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceSapling {
//     pub stage: StageAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchSapling {
//     pub stage: StageAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleSapling {
//     pub stage: StageAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaSapling {
//     pub stage: StageAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherrySapling {
//     pub stage: StageAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakSapling {
//     pub stage: StageAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangrovePropagule {
//     pub age_4: Age4Attribute,
//     pub hanging: HangingAttribute,
//     pub stage: StageAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBedrock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWater {
//     pub level_15: Level15Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLava {
//     pub level_15: Level15Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSand;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSuspiciousSand {
//     pub dusted: DustedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedSand;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGravel;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSuspiciousGravel {
//     pub dusted: DustedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGoldOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateGoldOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockIronOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateIronOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCoalOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateCoalOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNetherGoldOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveRoots {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMuddyMangroveRoots {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooBlock {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedSpruceLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedBirchLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedJungleLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedAcaciaLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedCherryLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedDarkOakLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedOakLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedMangroveLog {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedBambooBlock {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedOakWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedSpruceWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedBirchWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedJungleWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedAcaciaWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedCherryWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedDarkOakWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedMangroveWood {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakLeaves {
//     pub distance_1_7: Distance17Attribute,
//     pub persistent: PersistentAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceLeaves {
//     pub distance_1_7: Distance17Attribute,
//     pub persistent: PersistentAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchLeaves {
//     pub distance_1_7: Distance17Attribute,
//     pub persistent: PersistentAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleLeaves {
//     pub distance_1_7: Distance17Attribute,
//     pub persistent: PersistentAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaLeaves {
//     pub distance_1_7: Distance17Attribute,
//     pub persistent: PersistentAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryLeaves {
//     pub distance_1_7: Distance17Attribute,
//     pub persistent: PersistentAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakLeaves {
//     pub distance_1_7: Distance17Attribute,
//     pub persistent: PersistentAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveLeaves {
//     pub distance_1_7: Distance17Attribute,
//     pub persistent: PersistentAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAzaleaLeaves {
//     pub distance_1_7: Distance17Attribute,
//     pub persistent: PersistentAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFloweringAzaleaLeaves {
//     pub distance_1_7: Distance17Attribute,
//     pub persistent: PersistentAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSponge;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWetSponge;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLapisOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateLapisOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLapisBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDispenser {
//     pub facing: FacingAttribute,
//     pub triggered: TriggeredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSandstone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChiseledSandstone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCutSandstone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNoteBlock {
//     pub instrument: InstrumentAttribute,
//     pub note: NoteAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackBed {
//     pub bed_part: BedPartAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub occupied: OccupiedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPoweredRail {
//     pub powered: PoweredAttribute,
//     pub straight_rail_shape: StraightRailShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDetectorRail {
//     pub powered: PoweredAttribute,
//     pub straight_rail_shape: StraightRailShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStickyPiston {
//     pub extended: ExtendedAttribute,
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCobweb;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFern;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadBush;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSeagrass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTallSeagrass {
//     pub double_block_half: DoubleBlockHalfAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPiston {
//     pub extended: ExtendedAttribute,
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPistonHead {
//     pub facing: FacingAttribute,
//     pub piston_type: PistonTypeAttribute,
//     pub short: ShortAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackWool;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMovingPiston {
//     pub piston_type: PistonTypeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDandelion;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTorchflower;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPoppy;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueOrchid;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAllium;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAzureBluet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedTulip;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeTulip;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteTulip;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkTulip;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOxeyeDaisy;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCornflower;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWitherRose;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLilyOfTheValley;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownMushroom;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedMushroom;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGoldBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockIronBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTnt {
//     pub unstable: UnstableAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBookshelf;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChiseledBookshelf;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMossyCobblestone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockObsidian;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTorch;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWallTorch {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFire {
//     pub age_15: Age15Attribute,
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub up: UpAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSoulFire;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpawner;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChest {
//     pub chest_type: ChestTypeAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedstoneWire {
//     pub east_wire_connection: EastWireConnectionAttribute,
//     pub north_wire_connection: NorthWireConnectionAttribute,
//     pub power: PowerAttribute,
//     pub south_wire_connection: SouthWireConnectionAttribute,
//     pub west_wire_connection: WestWireConnectionAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDiamondOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateDiamondOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDiamondBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCraftingTable;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWheat {
//     pub age_7: Age7Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFarmland {
//     pub moisture: MoistureAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFurnace {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakSign {
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceSign {
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchSign {
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaSign {
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherrySign {
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleSign {
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakSign {
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveSign {
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooSign {
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakDoor {
//     pub door_hinge: DoorHingeAttribute,
//     pub double_block_half: DoubleBlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLadder {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRail {
//     pub rail_shape: RailShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCobblestoneStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakWallSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceWallSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchWallSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaWallSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryWallSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleWallSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakWallSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveWallSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooWallSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakHangingSign {
//     pub attached: AttachedAttribute,
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceHangingSign {
//     pub attached: AttachedAttribute,
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchHangingSign {
//     pub attached: AttachedAttribute,
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaHangingSign {
//     pub attached: AttachedAttribute,
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryHangingSign {
//     pub attached: AttachedAttribute,
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleHangingSign {
//     pub attached: AttachedAttribute,
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakHangingSign {
//     pub attached: AttachedAttribute,
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonHangingSign {
//     pub attached: AttachedAttribute,
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedHangingSign {
//     pub attached: AttachedAttribute,
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveHangingSign {
//     pub attached: AttachedAttribute,
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooHangingSign {
//     pub attached: AttachedAttribute,
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakWallHangingSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceWallHangingSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchWallHangingSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaWallHangingSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryWallHangingSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleWallHangingSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakWallHangingSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveWallHangingSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonWallHangingSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedWallHangingSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooWallHangingSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLever {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStonePressurePlate {
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockIronDoor {
//     pub door_hinge: DoorHingeAttribute,
//     pub double_block_half: DoubleBlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakPressurePlate {
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSprucePressurePlate {
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchPressurePlate {
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJunglePressurePlate {
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaPressurePlate {
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryPressurePlate {
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakPressurePlate {
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangrovePressurePlate {
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooPressurePlate {
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedstoneOre {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateRedstoneOre {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedstoneTorch {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedstoneWallTorch {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStoneButton {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSnow {
//     pub layers: LayersAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockIce;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSnowBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCactus {
//     pub age_15: Age15Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockClay;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSugarCane {
//     pub age_15: Age15Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJukebox {
//     pub has_record: HasRecordAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakFence {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPumpkin;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNetherrack;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSoulSand;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSoulSoil;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBasalt {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedBasalt {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSoulTorch;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSoulWallTorch {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGlowstone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNetherPortal {
//     pub horizontal_axis: HorizontalAxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCarvedPumpkin {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJackOLantern {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCake {
//     pub bites: BitesAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRepeater {
//     pub delay: DelayAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub locked: LockedAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackStainedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakTrapdoor {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceTrapdoor {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchTrapdoor {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleTrapdoor {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaTrapdoor {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryTrapdoor {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakTrapdoor {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveTrapdoor {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooTrapdoor {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStoneBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMossyStoneBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrackedStoneBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChiseledStoneBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPackedMud;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMudBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockInfestedStone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockInfestedCobblestone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockInfestedStoneBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockInfestedMossyStoneBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockInfestedCrackedStoneBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockInfestedChiseledStoneBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownMushroomBlock {
//     pub down: DownAttribute,
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub up: UpAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedMushroomBlock {
//     pub down: DownAttribute,
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub up: UpAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMushroomStem {
//     pub down: DownAttribute,
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub up: UpAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockIronBars {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChain {
//     pub axis: AxisAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMelon;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAttachedPumpkinStem {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAttachedMelonStem {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPumpkinStem {
//     pub age_7: Age7Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMelonStem {
//     pub age_7: Age7Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockVine {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub up: UpAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGlowLichen {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakFenceGate {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub in_wall: InWallAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrickStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStoneBrickStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMudBrickStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMycelium {
//     pub snowy: SnowyAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLilyPad;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNetherBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNetherBrickFence {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNetherBrickStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNetherWart {
//     pub age_3: Age3Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockEnchantingTable;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrewingStand;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCauldron;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaterCauldron {
//     pub level_3: Level3Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLavaCauldron;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPowderSnowCauldron {
//     pub level_3: Level3Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockEndPortal;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockEndPortalFrame {
//     pub eye: EyeAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockEndStone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDragonEgg;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedstoneLamp {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCocoa {
//     pub age_2: Age2Attribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSandstoneStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockEmeraldOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateEmeraldOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockEnderChest {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTripwireHook {
//     pub attached: AttachedAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTripwire {
//     pub attached: AttachedAttribute,
//     pub disarmed: DisarmedAttribute,
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub powered: PoweredAttribute,
//     pub south: SouthAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockEmeraldBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCommandBlock {
//     pub conditional: ConditionalAttribute,
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBeacon;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCobblestoneWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMossyCobblestoneWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFlowerPot;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedTorchflower;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedOakSapling;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedSpruceSapling;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedBirchSapling;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedJungleSapling;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedAcaciaSapling;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedCherrySapling;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedDarkOakSapling;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedMangrovePropagule;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedFern;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedDandelion;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedPoppy;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedBlueOrchid;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedAllium;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedAzureBluet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedRedTulip;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedOrangeTulip;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedWhiteTulip;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedPinkTulip;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedOxeyeDaisy;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedCornflower;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedLilyOfTheValley;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedWitherRose;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedRedMushroom;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedBrownMushroom;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedDeadBush;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedCactus;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCarrots {
//     pub age_7: Age7Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPotatoes {
//     pub age_7: Age7Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakButton {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceButton {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchButton {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleButton {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaButton {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryButton {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakButton {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveButton {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooButton {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSkeletonSkull {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSkeletonWallSkull {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWitherSkeletonSkull {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWitherSkeletonWallSkull {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockZombieHead {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockZombieWallHead {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPlayerHead {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPlayerWallHead {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCreeperHead {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCreeperWallHead {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDragonHead {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDragonWallHead {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPiglinHead {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPiglinWallHead {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAnvil {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChippedAnvil {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDamagedAnvil {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTrappedChest {
//     pub chest_type: ChestTypeAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightWeightedPressurePlate {
//     pub power: PowerAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockHeavyWeightedPressurePlate {
//     pub power: PowerAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockComparator {
//     pub comparator_mode: ComparatorModeAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDaylightDetector {
//     pub inverted: InvertedAttribute,
//     pub power: PowerAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedstoneBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNetherQuartzOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockHopper {
//     pub enabled: EnabledAttribute,
//     pub hopper_facing: HopperFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockQuartzBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChiseledQuartzBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockQuartzPillar {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockQuartzStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockActivatorRail {
//     pub powered: PoweredAttribute,
//     pub straight_rail_shape: StraightRailShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDropper {
//     pub facing: FacingAttribute,
//     pub triggered: TriggeredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackStainedGlassPane {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooMosaicStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSlimeBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBarrier;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLight {
//     pub level_15: Level15Attribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockIronTrapdoor {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPrismarine;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPrismarineBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkPrismarine;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPrismarineStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPrismarineBrickStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkPrismarineStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPrismarineSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPrismarineBrickSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkPrismarineSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSeaLantern;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockHayBlock {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTerracotta;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCoalBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPackedIce;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSunflower {
//     pub double_block_half: DoubleBlockHalfAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLilac {
//     pub double_block_half: DoubleBlockHalfAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRoseBush {
//     pub double_block_half: DoubleBlockHalfAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPeony {
//     pub double_block_half: DoubleBlockHalfAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTallGrass {
//     pub double_block_half: DoubleBlockHalfAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLargeFern {
//     pub double_block_half: DoubleBlockHalfAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackBanner {
//     pub rotation: RotationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackWallBanner {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedSandstone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChiseledRedSandstone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCutRedSandstone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedSandstoneStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOakSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherrySlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooMosaicSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStoneSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmoothStoneSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSandstoneSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCutSandstoneSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPetrifiedOakSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCobblestoneSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrickSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStoneBrickSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMudBrickSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNetherBrickSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockQuartzSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedSandstoneSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCutRedSandstoneSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpurSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmoothStone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmoothSandstone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmoothQuartz;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmoothRedSandstone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceFenceGate {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub in_wall: InWallAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchFenceGate {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub in_wall: InWallAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleFenceGate {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub in_wall: InWallAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaFenceGate {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub in_wall: InWallAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryFenceGate {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub in_wall: InWallAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakFenceGate {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub in_wall: InWallAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveFenceGate {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub in_wall: InWallAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooFenceGate {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub in_wall: InWallAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceFence {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchFence {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleFence {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaFence {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryFence {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakFence {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveFence {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooFence {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSpruceDoor {
//     pub door_hinge: DoorHingeAttribute,
//     pub double_block_half: DoubleBlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBirchDoor {
//     pub door_hinge: DoorHingeAttribute,
//     pub double_block_half: DoubleBlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJungleDoor {
//     pub door_hinge: DoorHingeAttribute,
//     pub double_block_half: DoubleBlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAcaciaDoor {
//     pub door_hinge: DoorHingeAttribute,
//     pub double_block_half: DoubleBlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCherryDoor {
//     pub door_hinge: DoorHingeAttribute,
//     pub double_block_half: DoubleBlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDarkOakDoor {
//     pub door_hinge: DoorHingeAttribute,
//     pub double_block_half: DoubleBlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMangroveDoor {
//     pub door_hinge: DoorHingeAttribute,
//     pub double_block_half: DoubleBlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooDoor {
//     pub door_hinge: DoorHingeAttribute,
//     pub double_block_half: DoubleBlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockEndRod {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChorusPlant {
//     pub down: DownAttribute,
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub up: UpAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChorusFlower {
//     pub age_5: Age5Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpurBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpurPillar {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpurStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockEndStoneBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTorchflowerCrop {
//     pub age_1: Age1Attribute,
//     pub age_7: Age7Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPitcherCrop {
//     pub age_4: Age4Attribute,
//     pub double_block_half: DoubleBlockHalfAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPitcherPlant {
//     pub double_block_half: DoubleBlockHalfAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBeetroots {
//     pub age_3: Age3Attribute,
//     pub age_7: Age7Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDirtPath;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockEndGateway;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRepeatingCommandBlock {
//     pub conditional: ConditionalAttribute,
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChainCommandBlock {
//     pub conditional: ConditionalAttribute,
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFrostedIce {
//     pub age_3: Age3Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagmaBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNetherWartBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedNetherBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBoneBlock {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStructureVoid;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockObserver {
//     pub facing: FacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackShulkerBox {
//     pub facing: FacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackGlazedTerracotta {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackConcrete;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackConcretePowder;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockKelp {
//     pub age_25: Age25Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockKelpPlant;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDriedKelpBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTurtleEgg {
//     pub eggs: EggsAttribute,
//     pub hatch: HatchAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSnifferEgg {
//     pub hatch: HatchAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadTubeCoralBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadBrainCoralBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadBubbleCoralBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadFireCoralBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadHornCoralBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTubeCoralBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrainCoralBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBubbleCoralBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFireCoralBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockHornCoralBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadTubeCoral {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadBrainCoral {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadBubbleCoral {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadFireCoral {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadHornCoral {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTubeCoral {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrainCoral {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBubbleCoral {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFireCoral {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockHornCoral {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadTubeCoralFan {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadBrainCoralFan {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadBubbleCoralFan {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadFireCoralFan {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadHornCoralFan {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTubeCoralFan {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrainCoralFan {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBubbleCoralFan {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFireCoralFan {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockHornCoralFan {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadTubeCoralWallFan {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadBrainCoralWallFan {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadBubbleCoralWallFan {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadFireCoralWallFan {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeadHornCoralWallFan {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTubeCoralWallFan {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrainCoralWallFan {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBubbleCoralWallFan {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFireCoralWallFan {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockHornCoralWallFan {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSeaPickle {
//     pub pickles: PicklesAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueIce;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockConduit {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBambooSapling;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBamboo {
//     pub age_1: Age1Attribute,
//     pub bamboo_leaves: BambooLeavesAttribute,
//     pub stage: StageAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedBamboo;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockVoidAir;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCaveAir;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBubbleColumn {
//     pub drag: DragAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedGraniteStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmoothRedSandstoneStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMossyStoneBrickStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedDioriteStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMossyCobblestoneStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockEndStoneBrickStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStoneStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmoothSandstoneStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmoothQuartzStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGraniteStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAndesiteStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedNetherBrickStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedAndesiteStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDioriteStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedGraniteSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmoothRedSandstoneSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMossyStoneBrickSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedDioriteSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMossyCobblestoneSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockEndStoneBrickSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmoothSandstoneSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmoothQuartzSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGraniteSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAndesiteSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedNetherBrickSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedAndesiteSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDioriteSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrickWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPrismarineWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedSandstoneWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMossyStoneBrickWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGraniteWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStoneBrickWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMudBrickWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNetherBrickWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAndesiteWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedNetherBrickWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSandstoneWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockEndStoneBrickWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDioriteWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockScaffolding {
//     pub bottom: BottomAttribute,
//     pub distance_0_7: Distance07Attribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLoom {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBarrel {
//     pub facing: FacingAttribute,
//     pub open: OpenAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmoker {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlastFurnace {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCartographyTable;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFletchingTable;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrindstone {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLectern {
//     pub has_book: HasBookAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmithingTable;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStonecutter {
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBell {
//     pub attachment: AttachmentAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLantern {
//     pub hanging: HangingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSoulLantern {
//     pub hanging: HangingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCampfire {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub lit: LitAttribute,
//     pub signal_fire: SignalFireAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSoulCampfire {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub lit: LitAttribute,
//     pub signal_fire: SignalFireAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSweetBerryBush {
//     pub age_3: Age3Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedStem;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedWarpedStem;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedHyphae {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedWarpedHyphae {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedNylium;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedFungus;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedWartBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedRoots;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNetherSprouts;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonStem;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedCrimsonStem;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonHyphae {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStrippedCrimsonHyphae {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonNylium;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonFungus;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockShroomlight;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWeepingVines {
//     pub age_25: Age25Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWeepingVinesPlant;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTwistingVines {
//     pub age_25: Age25Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTwistingVinesPlant;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonRoots;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonPlanks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedPlanks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonPressurePlate {
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedPressurePlate {
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonFence {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedFence {
//     pub east: EastAttribute,
//     pub north: NorthAttribute,
//     pub south: SouthAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west: WestAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonTrapdoor {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedTrapdoor {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonFenceGate {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub in_wall: InWallAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedFenceGate {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub in_wall: InWallAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonButton {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedButton {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonDoor {
//     pub door_hinge: DoorHingeAttribute,
//     pub double_block_half: DoubleBlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedDoor {
//     pub door_hinge: DoorHingeAttribute,
//     pub double_block_half: DoubleBlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub open: OpenAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonSign {
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedSign {
//     pub rotation: RotationAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrimsonWallSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWarpedWallSign {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockStructureBlock {
//     pub structure_block_mode: StructureBlockModeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockJigsaw {
//     pub orientation: OrientationAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockComposter {
//     pub level_8: Level8Attribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTarget {
//     pub power: PowerAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBeeNest {
//     pub honey_level: HoneyLevelAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBeehive {
//     pub honey_level: HoneyLevelAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockHoneyBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockHoneycombBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockNetheriteBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAncientDebris;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCryingObsidian;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRespawnAnchor {
//     pub charges: ChargesAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedCrimsonFungus;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedWarpedFungus;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedCrimsonRoots;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedWarpedRoots;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLodestone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackstone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackstoneStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackstoneWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackstoneSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedBlackstone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedBlackstoneBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrackedPolishedBlackstoneBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChiseledPolishedBlackstone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedBlackstoneBrickSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedBlackstoneBrickStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedBlackstoneBrickWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGildedBlackstone;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedBlackstoneStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedBlackstoneSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedBlackstonePressurePlate {
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedBlackstoneButton {
//     pub block_face: BlockFaceAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub powered: PoweredAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedBlackstoneWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChiseledNetherBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrackedNetherBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockQuartzBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackCandle {
//     pub candles: CandlesAttribute,
//     pub lit: LitAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWhiteCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOrangeCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMagentaCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightBlueCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockYellowCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLimeCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGrayCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightGrayCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCyanCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPurpleCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlueCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBrownCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockGreenCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRedCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBlackCandleCake {
//     pub lit: LitAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAmethystBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBuddingAmethyst;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAmethystCluster {
//     pub facing: FacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLargeAmethystBud {
//     pub facing: FacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMediumAmethystBud {
//     pub facing: FacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmallAmethystBud {
//     pub facing: FacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTuff;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCalcite;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockTintedGlass;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPowderSnow;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSculkSensor {
//     pub power: PowerAttribute,
//     pub sculk_sensor_phase: SculkSensorPhaseAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCalibratedSculkSensor {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub power: PowerAttribute,
//     pub sculk_sensor_phase: SculkSensorPhaseAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSculk;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSculkVein {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSculkCatalyst {
//     pub bloom: BloomAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSculkShrieker {
//     pub can_summon: CanSummonAttribute,
//     pub shrieking: ShriekingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOxidizedCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWeatheredCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockExposedCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCopperBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCopperOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateCopperOre;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOxidizedCutCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWeatheredCutCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockExposedCutCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCutCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOxidizedCutCopperStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWeatheredCutCopperStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockExposedCutCopperStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCutCopperStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOxidizedCutCopperSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWeatheredCutCopperSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockExposedCutCopperSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCutCopperSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedCopperBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedWeatheredCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedExposedCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedOxidizedCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedOxidizedCutCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedWeatheredCutCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedExposedCutCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedCutCopper;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedOxidizedCutCopperStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedWeatheredCutCopperStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedExposedCutCopperStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedCutCopperStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedOxidizedCutCopperSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedWeatheredCutCopperSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedExposedCutCopperSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockWaxedCutCopperSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockLightningRod {
//     pub facing: FacingAttribute,
//     pub powered: PoweredAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPointedDripstone {
//     pub thickness: ThicknessAttribute,
//     pub vertical_direction: VerticalDirectionAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDripstoneBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCaveVines {
//     pub age_25: Age25Attribute,
//     pub berries: BerriesAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCaveVinesPlant {
//     pub berries: BerriesAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSporeBlossom;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockAzalea;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFloweringAzalea;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMossCarpet;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPinkPetals {
//     pub flower_amount: FlowerAmountAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMossBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBigDripleaf {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub tilt: TiltAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockBigDripleafStem {
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmallDripleaf {
//     pub double_block_half: DoubleBlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockHangingRoots {
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRootedDirt;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockMud;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslate {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCobbledDeepslate;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCobbledDeepslateStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCobbledDeepslateSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCobbledDeepslateWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedDeepslate;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedDeepslateStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedDeepslateSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPolishedDeepslateWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateTiles;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateTileStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateTileSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateTileWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateBrickStairs {
//     pub block_half: BlockHalfAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub stair_shape: StairShapeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateBrickSlab {
//     pub slab_type: SlabTypeAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDeepslateBrickWall {
//     pub east_wall_shape: EastWallShapeAttribute,
//     pub north_wall_shape: NorthWallShapeAttribute,
//     pub south_wall_shape: SouthWallShapeAttribute,
//     pub up: UpAttribute,
//     pub waterlogged: WaterloggedAttribute,
//     pub west_wall_shape: WestWallShapeAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockChiseledDeepslate;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrackedDeepslateBricks;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockCrackedDeepslateTiles;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockInfestedDeepslate;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockSmoothBasalt;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRawIronBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRawCopperBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockRawGoldBlock;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedAzaleaBush;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPottedFloweringAzaleaBush;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockOchreFroglight {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockVerdantFroglight {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockPearlescentFroglight {
//     pub axis: AxisAttribute,
// }

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockFrogspawn;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockReinforcedDeepslate;

// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
// // pub struct BlockDecoratedPot {
//     pub cracked: CrackedAttribute,
//     pub horizontal_facing: HorizontalFacingAttribute,
//     pub waterlogged: WaterloggedAttribute,
// }
