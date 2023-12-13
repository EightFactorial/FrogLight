use super::attributes::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockError;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAir;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGranite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedGranite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDiorite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedDiorite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAndesite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedAndesite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrassBlock {
    pub snowy: SnowyAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDirt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCoarseDirt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPodzol {
    pub snowy: SnowyAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCobblestone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakPlanks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSprucePlanks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchPlanks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJunglePlanks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaPlanks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryPlanks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakPlanks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangrovePlanks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooPlanks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooMosaic;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakSapling {
    pub stage: StageAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceSapling {
    pub stage: StageAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchSapling {
    pub stage: StageAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleSapling {
    pub stage: StageAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaSapling {
    pub stage: StageAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherrySapling {
    pub stage: StageAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakSapling {
    pub stage: StageAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangrovePropagule {
    pub age_4: Age4Attribute,
    pub hanging: HangingAttribute,
    pub stage: StageAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBedrock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWater {
    pub level_15: Level15Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLava {
    pub level_15: Level15Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSand;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSuspiciousSand {
    pub dusted: DustedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedSand;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGravel;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSuspiciousGravel {
    pub dusted: DustedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGoldOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateGoldOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockIronOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateIronOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCoalOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateCoalOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNetherGoldOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveRoots {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMuddyMangroveRoots {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooBlock {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedSpruceLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedBirchLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedJungleLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedAcaciaLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedCherryLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedDarkOakLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedOakLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedMangroveLog {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedBambooBlock {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedOakWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedSpruceWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedBirchWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedJungleWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedAcaciaWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedCherryWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedDarkOakWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedMangroveWood {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakLeaves {
    pub distance_1_7: Distance17Attribute,
    pub persistent: PersistentAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceLeaves {
    pub distance_1_7: Distance17Attribute,
    pub persistent: PersistentAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchLeaves {
    pub distance_1_7: Distance17Attribute,
    pub persistent: PersistentAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleLeaves {
    pub distance_1_7: Distance17Attribute,
    pub persistent: PersistentAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaLeaves {
    pub distance_1_7: Distance17Attribute,
    pub persistent: PersistentAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryLeaves {
    pub distance_1_7: Distance17Attribute,
    pub persistent: PersistentAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakLeaves {
    pub distance_1_7: Distance17Attribute,
    pub persistent: PersistentAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveLeaves {
    pub distance_1_7: Distance17Attribute,
    pub persistent: PersistentAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAzaleaLeaves {
    pub distance_1_7: Distance17Attribute,
    pub persistent: PersistentAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFloweringAzaleaLeaves {
    pub distance_1_7: Distance17Attribute,
    pub persistent: PersistentAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSponge;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWetSponge;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLapisOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateLapisOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLapisBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDispenser {
    pub facing: FacingAttribute,
    pub triggered: TriggeredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSandstone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChiseledSandstone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCutSandstone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNoteBlock {
    pub instrument: InstrumentAttribute,
    pub note: NoteAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackBed {
    pub bed_part: BedPartAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub occupied: OccupiedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPoweredRail {
    pub powered: PoweredAttribute,
    pub straight_rail_shape: StraightRailShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDetectorRail {
    pub powered: PoweredAttribute,
    pub straight_rail_shape: StraightRailShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStickyPiston {
    pub extended: ExtendedAttribute,
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCobweb;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFern;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadBush;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSeagrass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTallSeagrass {
    pub double_block_half: DoubleBlockHalfAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPiston {
    pub extended: ExtendedAttribute,
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPistonHead {
    pub facing: FacingAttribute,
    pub piston_type: PistonTypeAttribute,
    pub short: ShortAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackWool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMovingPiston {
    pub piston_type: PistonTypeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDandelion;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTorchflower;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPoppy;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueOrchid;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAllium;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAzureBluet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedTulip;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeTulip;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteTulip;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkTulip;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOxeyeDaisy;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCornflower;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWitherRose;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLilyOfTheValley;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownMushroom;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedMushroom;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGoldBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockIronBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTnt {
    pub unstable: UnstableAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBookshelf;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChiseledBookshelf;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMossyCobblestone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockObsidian;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTorch;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWallTorch {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFire {
    pub age_15: Age15Attribute,
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub up: UpAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSoulFire;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpawner;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChest {
    pub chest_type: ChestTypeAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedstoneWire {
    pub east_wire_connection: EastWireConnectionAttribute,
    pub north_wire_connection: NorthWireConnectionAttribute,
    pub power: PowerAttribute,
    pub south_wire_connection: SouthWireConnectionAttribute,
    pub west_wire_connection: WestWireConnectionAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDiamondOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateDiamondOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDiamondBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCraftingTable;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWheat {
    pub age_7: Age7Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFarmland {
    pub moisture: MoistureAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFurnace {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakSign {
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceSign {
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchSign {
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaSign {
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherrySign {
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleSign {
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakSign {
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveSign {
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooSign {
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakDoor {
    pub door_hinge: DoorHingeAttribute,
    pub double_block_half: DoubleBlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLadder {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRail {
    pub rail_shape: RailShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCobblestoneStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakWallSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceWallSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchWallSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaWallSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryWallSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleWallSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakWallSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveWallSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooWallSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakHangingSign {
    pub attached: AttachedAttribute,
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceHangingSign {
    pub attached: AttachedAttribute,
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchHangingSign {
    pub attached: AttachedAttribute,
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaHangingSign {
    pub attached: AttachedAttribute,
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryHangingSign {
    pub attached: AttachedAttribute,
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleHangingSign {
    pub attached: AttachedAttribute,
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakHangingSign {
    pub attached: AttachedAttribute,
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonHangingSign {
    pub attached: AttachedAttribute,
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedHangingSign {
    pub attached: AttachedAttribute,
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveHangingSign {
    pub attached: AttachedAttribute,
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooHangingSign {
    pub attached: AttachedAttribute,
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakWallHangingSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceWallHangingSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchWallHangingSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaWallHangingSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryWallHangingSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleWallHangingSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakWallHangingSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveWallHangingSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonWallHangingSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedWallHangingSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooWallHangingSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLever {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStonePressurePlate {
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockIronDoor {
    pub door_hinge: DoorHingeAttribute,
    pub double_block_half: DoubleBlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakPressurePlate {
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSprucePressurePlate {
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchPressurePlate {
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJunglePressurePlate {
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaPressurePlate {
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryPressurePlate {
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakPressurePlate {
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangrovePressurePlate {
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooPressurePlate {
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedstoneOre {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateRedstoneOre {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedstoneTorch {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedstoneWallTorch {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStoneButton {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSnow {
    pub layers: LayersAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockIce;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSnowBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCactus {
    pub age_15: Age15Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockClay;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSugarCane {
    pub age_15: Age15Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJukebox {
    pub has_record: HasRecordAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakFence {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPumpkin;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNetherrack;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSoulSand;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSoulSoil;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBasalt {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedBasalt {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSoulTorch;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSoulWallTorch {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGlowstone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNetherPortal {
    pub horizontal_axis: HorizontalAxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCarvedPumpkin {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJackOLantern {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCake {
    pub bites: BitesAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRepeater {
    pub delay: DelayAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub locked: LockedAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackStainedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakTrapdoor {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceTrapdoor {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchTrapdoor {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleTrapdoor {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaTrapdoor {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryTrapdoor {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakTrapdoor {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveTrapdoor {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooTrapdoor {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStoneBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMossyStoneBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrackedStoneBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChiseledStoneBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPackedMud;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMudBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockInfestedStone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockInfestedCobblestone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockInfestedStoneBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockInfestedMossyStoneBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockInfestedCrackedStoneBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockInfestedChiseledStoneBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownMushroomBlock {
    pub down: DownAttribute,
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub up: UpAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedMushroomBlock {
    pub down: DownAttribute,
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub up: UpAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMushroomStem {
    pub down: DownAttribute,
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub up: UpAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockIronBars {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChain {
    pub axis: AxisAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMelon;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAttachedPumpkinStem {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAttachedMelonStem {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPumpkinStem {
    pub age_7: Age7Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMelonStem {
    pub age_7: Age7Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockVine {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub up: UpAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGlowLichen {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakFenceGate {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub in_wall: InWallAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrickStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStoneBrickStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMudBrickStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMycelium {
    pub snowy: SnowyAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLilyPad;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNetherBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNetherBrickFence {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNetherBrickStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNetherWart {
    pub age_3: Age3Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockEnchantingTable;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrewingStand;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCauldron;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaterCauldron {
    pub level_3: Level3Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLavaCauldron;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPowderSnowCauldron {
    pub level_3: Level3Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockEndPortal;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockEndPortalFrame {
    pub eye: EyeAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockEndStone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDragonEgg;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedstoneLamp {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCocoa {
    pub age_2: Age2Attribute,
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSandstoneStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockEmeraldOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateEmeraldOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockEnderChest {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTripwireHook {
    pub attached: AttachedAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTripwire {
    pub attached: AttachedAttribute,
    pub disarmed: DisarmedAttribute,
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub powered: PoweredAttribute,
    pub south: SouthAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockEmeraldBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCommandBlock {
    pub conditional: ConditionalAttribute,
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBeacon;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCobblestoneWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMossyCobblestoneWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFlowerPot;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedTorchflower;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedOakSapling;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedSpruceSapling;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedBirchSapling;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedJungleSapling;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedAcaciaSapling;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedCherrySapling;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedDarkOakSapling;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedMangrovePropagule;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedFern;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedDandelion;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedPoppy;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedBlueOrchid;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedAllium;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedAzureBluet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedRedTulip;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedOrangeTulip;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedWhiteTulip;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedPinkTulip;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedOxeyeDaisy;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedCornflower;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedLilyOfTheValley;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedWitherRose;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedRedMushroom;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedBrownMushroom;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedDeadBush;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedCactus;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCarrots {
    pub age_7: Age7Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPotatoes {
    pub age_7: Age7Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakButton {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceButton {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchButton {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleButton {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaButton {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryButton {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakButton {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveButton {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooButton {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSkeletonSkull {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSkeletonWallSkull {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWitherSkeletonSkull {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWitherSkeletonWallSkull {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockZombieHead {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockZombieWallHead {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPlayerHead {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPlayerWallHead {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCreeperHead {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCreeperWallHead {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDragonHead {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDragonWallHead {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPiglinHead {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPiglinWallHead {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAnvil {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChippedAnvil {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDamagedAnvil {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTrappedChest {
    pub chest_type: ChestTypeAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightWeightedPressurePlate {
    pub power: PowerAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockHeavyWeightedPressurePlate {
    pub power: PowerAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockComparator {
    pub comparator_mode: ComparatorModeAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDaylightDetector {
    pub inverted: InvertedAttribute,
    pub power: PowerAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedstoneBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNetherQuartzOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockHopper {
    pub enabled: EnabledAttribute,
    pub hopper_facing: HopperFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockQuartzBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChiseledQuartzBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockQuartzPillar {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockQuartzStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockActivatorRail {
    pub powered: PoweredAttribute,
    pub straight_rail_shape: StraightRailShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDropper {
    pub facing: FacingAttribute,
    pub triggered: TriggeredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackStainedGlassPane {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooMosaicStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSlimeBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBarrier;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLight {
    pub level_15: Level15Attribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockIronTrapdoor {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPrismarine;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPrismarineBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkPrismarine;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPrismarineStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPrismarineBrickStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkPrismarineStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPrismarineSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPrismarineBrickSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkPrismarineSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSeaLantern;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockHayBlock {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTerracotta;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCoalBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPackedIce;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSunflower {
    pub double_block_half: DoubleBlockHalfAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLilac {
    pub double_block_half: DoubleBlockHalfAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRoseBush {
    pub double_block_half: DoubleBlockHalfAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPeony {
    pub double_block_half: DoubleBlockHalfAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTallGrass {
    pub double_block_half: DoubleBlockHalfAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLargeFern {
    pub double_block_half: DoubleBlockHalfAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackBanner {
    pub rotation: RotationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackWallBanner {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedSandstone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChiseledRedSandstone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCutRedSandstone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedSandstoneStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOakSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherrySlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooMosaicSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStoneSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmoothStoneSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSandstoneSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCutSandstoneSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPetrifiedOakSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCobblestoneSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrickSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStoneBrickSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMudBrickSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNetherBrickSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockQuartzSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedSandstoneSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCutRedSandstoneSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpurSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmoothStone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmoothSandstone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmoothQuartz;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmoothRedSandstone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceFenceGate {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub in_wall: InWallAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchFenceGate {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub in_wall: InWallAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleFenceGate {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub in_wall: InWallAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaFenceGate {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub in_wall: InWallAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryFenceGate {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub in_wall: InWallAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakFenceGate {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub in_wall: InWallAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveFenceGate {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub in_wall: InWallAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooFenceGate {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub in_wall: InWallAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceFence {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchFence {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleFence {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaFence {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryFence {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakFence {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveFence {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooFence {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSpruceDoor {
    pub door_hinge: DoorHingeAttribute,
    pub double_block_half: DoubleBlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBirchDoor {
    pub door_hinge: DoorHingeAttribute,
    pub double_block_half: DoubleBlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJungleDoor {
    pub door_hinge: DoorHingeAttribute,
    pub double_block_half: DoubleBlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAcaciaDoor {
    pub door_hinge: DoorHingeAttribute,
    pub double_block_half: DoubleBlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCherryDoor {
    pub door_hinge: DoorHingeAttribute,
    pub double_block_half: DoubleBlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDarkOakDoor {
    pub door_hinge: DoorHingeAttribute,
    pub double_block_half: DoubleBlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMangroveDoor {
    pub door_hinge: DoorHingeAttribute,
    pub double_block_half: DoubleBlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooDoor {
    pub door_hinge: DoorHingeAttribute,
    pub double_block_half: DoubleBlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockEndRod {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChorusPlant {
    pub down: DownAttribute,
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub up: UpAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChorusFlower {
    pub age_5: Age5Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpurBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpurPillar {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpurStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockEndStoneBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTorchflowerCrop {
    pub age_1: Age1Attribute,
    pub age_7: Age7Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPitcherCrop {
    pub age_4: Age4Attribute,
    pub double_block_half: DoubleBlockHalfAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPitcherPlant {
    pub double_block_half: DoubleBlockHalfAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBeetroots {
    pub age_3: Age3Attribute,
    pub age_7: Age7Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDirtPath;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockEndGateway;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRepeatingCommandBlock {
    pub conditional: ConditionalAttribute,
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChainCommandBlock {
    pub conditional: ConditionalAttribute,
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFrostedIce {
    pub age_3: Age3Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagmaBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNetherWartBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedNetherBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBoneBlock {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStructureVoid;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockObserver {
    pub facing: FacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackShulkerBox {
    pub facing: FacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackGlazedTerracotta {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackConcrete;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackConcretePowder;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockKelp {
    pub age_25: Age25Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockKelpPlant;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDriedKelpBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTurtleEgg {
    pub eggs: EggsAttribute,
    pub hatch: HatchAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSnifferEgg {
    pub hatch: HatchAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadTubeCoralBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadBrainCoralBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadBubbleCoralBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadFireCoralBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadHornCoralBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTubeCoralBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrainCoralBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBubbleCoralBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFireCoralBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockHornCoralBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadTubeCoral {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadBrainCoral {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadBubbleCoral {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadFireCoral {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadHornCoral {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTubeCoral {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrainCoral {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBubbleCoral {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFireCoral {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockHornCoral {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadTubeCoralFan {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadBrainCoralFan {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadBubbleCoralFan {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadFireCoralFan {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadHornCoralFan {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTubeCoralFan {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrainCoralFan {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBubbleCoralFan {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFireCoralFan {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockHornCoralFan {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadTubeCoralWallFan {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadBrainCoralWallFan {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadBubbleCoralWallFan {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadFireCoralWallFan {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeadHornCoralWallFan {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTubeCoralWallFan {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrainCoralWallFan {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBubbleCoralWallFan {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFireCoralWallFan {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockHornCoralWallFan {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSeaPickle {
    pub pickles: PicklesAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueIce;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockConduit {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBambooSapling;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBamboo {
    pub age_1: Age1Attribute,
    pub bamboo_leaves: BambooLeavesAttribute,
    pub stage: StageAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedBamboo;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockVoidAir;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCaveAir;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBubbleColumn {
    pub drag: DragAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedGraniteStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmoothRedSandstoneStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMossyStoneBrickStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedDioriteStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMossyCobblestoneStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockEndStoneBrickStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStoneStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmoothSandstoneStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmoothQuartzStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGraniteStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAndesiteStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedNetherBrickStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedAndesiteStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDioriteStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedGraniteSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmoothRedSandstoneSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMossyStoneBrickSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedDioriteSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMossyCobblestoneSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockEndStoneBrickSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmoothSandstoneSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmoothQuartzSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGraniteSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAndesiteSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedNetherBrickSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedAndesiteSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDioriteSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrickWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPrismarineWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedSandstoneWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMossyStoneBrickWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGraniteWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStoneBrickWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMudBrickWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNetherBrickWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAndesiteWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedNetherBrickWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSandstoneWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockEndStoneBrickWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDioriteWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockScaffolding {
    pub bottom: BottomAttribute,
    pub distance_0_7: Distance07Attribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLoom {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBarrel {
    pub facing: FacingAttribute,
    pub open: OpenAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmoker {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlastFurnace {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCartographyTable;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFletchingTable;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrindstone {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLectern {
    pub has_book: HasBookAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmithingTable;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStonecutter {
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBell {
    pub attachment: AttachmentAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLantern {
    pub hanging: HangingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSoulLantern {
    pub hanging: HangingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCampfire {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub lit: LitAttribute,
    pub signal_fire: SignalFireAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSoulCampfire {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub lit: LitAttribute,
    pub signal_fire: SignalFireAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSweetBerryBush {
    pub age_3: Age3Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedStem;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedWarpedStem;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedHyphae {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedWarpedHyphae {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedNylium;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedFungus;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedWartBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedRoots;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNetherSprouts;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonStem;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedCrimsonStem;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonHyphae {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStrippedCrimsonHyphae {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonNylium;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonFungus;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockShroomlight;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWeepingVines {
    pub age_25: Age25Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWeepingVinesPlant;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTwistingVines {
    pub age_25: Age25Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTwistingVinesPlant;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonRoots;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonPlanks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedPlanks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonPressurePlate {
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedPressurePlate {
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonFence {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedFence {
    pub east: EastAttribute,
    pub north: NorthAttribute,
    pub south: SouthAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west: WestAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonTrapdoor {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedTrapdoor {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonFenceGate {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub in_wall: InWallAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedFenceGate {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub in_wall: InWallAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonButton {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedButton {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonDoor {
    pub door_hinge: DoorHingeAttribute,
    pub double_block_half: DoubleBlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedDoor {
    pub door_hinge: DoorHingeAttribute,
    pub double_block_half: DoubleBlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub open: OpenAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonSign {
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedSign {
    pub rotation: RotationAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrimsonWallSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWarpedWallSign {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStructureBlock {
    pub structure_block_mode: StructureBlockModeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockJigsaw {
    pub orientation: OrientationAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockComposter {
    pub level_8: Level8Attribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTarget {
    pub power: PowerAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBeeNest {
    pub honey_level: HoneyLevelAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBeehive {
    pub honey_level: HoneyLevelAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockHoneyBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockHoneycombBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockNetheriteBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAncientDebris;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCryingObsidian;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRespawnAnchor {
    pub charges: ChargesAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedCrimsonFungus;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedWarpedFungus;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedCrimsonRoots;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedWarpedRoots;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLodestone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackstone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackstoneStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackstoneWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackstoneSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedBlackstone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedBlackstoneBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrackedPolishedBlackstoneBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChiseledPolishedBlackstone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedBlackstoneBrickSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedBlackstoneBrickStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedBlackstoneBrickWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGildedBlackstone;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedBlackstoneStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedBlackstoneSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedBlackstonePressurePlate {
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedBlackstoneButton {
    pub block_face: BlockFaceAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub powered: PoweredAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedBlackstoneWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChiseledNetherBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrackedNetherBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockQuartzBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackCandle {
    pub candles: CandlesAttribute,
    pub lit: LitAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWhiteCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOrangeCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMagentaCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightBlueCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockYellowCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLimeCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGrayCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightGrayCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCyanCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPurpleCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlueCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBrownCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockGreenCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBlackCandleCake {
    pub lit: LitAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAmethystBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBuddingAmethyst;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAmethystCluster {
    pub facing: FacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLargeAmethystBud {
    pub facing: FacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMediumAmethystBud {
    pub facing: FacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmallAmethystBud {
    pub facing: FacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTuff;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCalcite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockTintedGlass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPowderSnow;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSculkSensor {
    pub power: PowerAttribute,
    pub sculk_sensor_phase: SculkSensorPhaseAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCalibratedSculkSensor {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub power: PowerAttribute,
    pub sculk_sensor_phase: SculkSensorPhaseAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSculk;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSculkVein {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSculkCatalyst {
    pub bloom: BloomAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSculkShrieker {
    pub can_summon: CanSummonAttribute,
    pub shrieking: ShriekingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOxidizedCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWeatheredCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockExposedCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCopperBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCopperOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateCopperOre;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOxidizedCutCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWeatheredCutCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockExposedCutCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCutCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOxidizedCutCopperStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWeatheredCutCopperStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockExposedCutCopperStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCutCopperStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOxidizedCutCopperSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWeatheredCutCopperSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockExposedCutCopperSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCutCopperSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedCopperBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedWeatheredCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedExposedCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedOxidizedCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedOxidizedCutCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedWeatheredCutCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedExposedCutCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedCutCopper;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedOxidizedCutCopperStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedWeatheredCutCopperStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedExposedCutCopperStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedCutCopperStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedOxidizedCutCopperSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedWeatheredCutCopperSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedExposedCutCopperSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockWaxedCutCopperSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockLightningRod {
    pub facing: FacingAttribute,
    pub powered: PoweredAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPointedDripstone {
    pub thickness: ThicknessAttribute,
    pub vertical_direction: VerticalDirectionAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDripstoneBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCaveVines {
    pub age_25: Age25Attribute,
    pub berries: BerriesAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCaveVinesPlant {
    pub berries: BerriesAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSporeBlossom;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAzalea;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFloweringAzalea;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMossCarpet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPinkPetals {
    pub flower_amount: FlowerAmountAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMossBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBigDripleaf {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub tilt: TiltAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockBigDripleafStem {
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmallDripleaf {
    pub double_block_half: DoubleBlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockHangingRoots {
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRootedDirt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMud;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslate {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCobbledDeepslate;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCobbledDeepslateStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCobbledDeepslateSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCobbledDeepslateWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedDeepslate;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedDeepslateStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedDeepslateSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPolishedDeepslateWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateTiles;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateTileStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateTileSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateTileWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateBrickStairs {
    pub block_half: BlockHalfAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub stair_shape: StairShapeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateBrickSlab {
    pub slab_type: SlabTypeAttribute,
    pub waterlogged: WaterloggedAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeepslateBrickWall {
    pub east_wall_shape: EastWallShapeAttribute,
    pub north_wall_shape: NorthWallShapeAttribute,
    pub south_wall_shape: SouthWallShapeAttribute,
    pub up: UpAttribute,
    pub waterlogged: WaterloggedAttribute,
    pub west_wall_shape: WestWallShapeAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockChiseledDeepslate;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrackedDeepslateBricks;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockCrackedDeepslateTiles;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockInfestedDeepslate;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockSmoothBasalt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRawIronBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRawCopperBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockRawGoldBlock;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedAzaleaBush;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPottedFloweringAzaleaBush;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockOchreFroglight {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockVerdantFroglight {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockPearlescentFroglight {
    pub axis: AxisAttribute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockFrogspawn;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockReinforcedDeepslate;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockDecoratedPot {
    pub cracked: CrackedAttribute,
    pub horizontal_facing: HorizontalFacingAttribute,
    pub waterlogged: WaterloggedAttribute,
}
