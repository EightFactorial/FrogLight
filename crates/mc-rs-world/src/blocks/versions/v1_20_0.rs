use crate::blocks::{
    structs::*,
    traits::{BlockTrait, BlocksTrait},
    Blocks,
};
use mc_rs_protocol::versions::v1_20_0::V1_20_0;

impl BlocksTrait<V1_20_0> for Blocks {
    fn resource_location(&self) -> &'static str {
        match self {
            Self::Error(b) => b.resource_location(),
            Self::Air(b) => b.resource_location(),
            Self::Stone(b) => b.resource_location(),
            Self::Granite(b) => b.resource_location(),
            Self::PolishedGranite(b) => b.resource_location(),
            Self::Diorite(b) => b.resource_location(),
            Self::PolishedDiorite(b) => b.resource_location(),
            Self::Andesite(b) => b.resource_location(),
            Self::PolishedAndesite(b) => b.resource_location(),
            Self::GrassBlock(b) => b.resource_location(),
            Self::Dirt(b) => b.resource_location(),
            Self::CoarseDirt(b) => b.resource_location(),
            Self::Podzol(b) => b.resource_location(),
            Self::Cobblestone(b) => b.resource_location(),
            Self::OakPlanks(b) => b.resource_location(),
            Self::SprucePlanks(b) => b.resource_location(),
            Self::BirchPlanks(b) => b.resource_location(),
            Self::JunglePlanks(b) => b.resource_location(),
            Self::AcaciaPlanks(b) => b.resource_location(),
            Self::CherryPlanks(b) => b.resource_location(),
            Self::DarkOakPlanks(b) => b.resource_location(),
            Self::MangrovePlanks(b) => b.resource_location(),
            Self::BambooPlanks(b) => b.resource_location(),
            Self::BambooMosaic(b) => b.resource_location(),
            Self::OakSapling(b) => b.resource_location(),
            Self::SpruceSapling(b) => b.resource_location(),
            Self::BirchSapling(b) => b.resource_location(),
            Self::JungleSapling(b) => b.resource_location(),
            Self::AcaciaSapling(b) => b.resource_location(),
            Self::CherrySapling(b) => b.resource_location(),
            Self::DarkOakSapling(b) => b.resource_location(),
            Self::MangrovePropagule(b) => b.resource_location(),
            Self::Bedrock(b) => b.resource_location(),
            Self::Water(b) => b.resource_location(),
            Self::Lava(b) => b.resource_location(),
            Self::Sand(b) => b.resource_location(),
            Self::SuspiciousSand(b) => b.resource_location(),
            Self::RedSand(b) => b.resource_location(),
            Self::Gravel(b) => b.resource_location(),
            Self::SuspiciousGravel(b) => b.resource_location(),
            Self::GoldOre(b) => b.resource_location(),
            Self::DeepslateGoldOre(b) => b.resource_location(),
            Self::IronOre(b) => b.resource_location(),
            Self::DeepslateIronOre(b) => b.resource_location(),
            Self::CoalOre(b) => b.resource_location(),
            Self::DeepslateCoalOre(b) => b.resource_location(),
            Self::NetherGoldOre(b) => b.resource_location(),
            Self::OakLog(b) => b.resource_location(),
            Self::SpruceLog(b) => b.resource_location(),
            Self::BirchLog(b) => b.resource_location(),
            Self::JungleLog(b) => b.resource_location(),
            Self::AcaciaLog(b) => b.resource_location(),
            Self::CherryLog(b) => b.resource_location(),
            Self::DarkOakLog(b) => b.resource_location(),
            Self::MangroveLog(b) => b.resource_location(),
            Self::MangroveRoots(b) => b.resource_location(),
            Self::MuddyMangroveRoots(b) => b.resource_location(),
            Self::BambooBlock(b) => b.resource_location(),
            Self::StrippedSpruceLog(b) => b.resource_location(),
            Self::StrippedBirchLog(b) => b.resource_location(),
            Self::StrippedJungleLog(b) => b.resource_location(),
            Self::StrippedAcaciaLog(b) => b.resource_location(),
            Self::StrippedCherryLog(b) => b.resource_location(),
            Self::StrippedDarkOakLog(b) => b.resource_location(),
            Self::StrippedOakLog(b) => b.resource_location(),
            Self::StrippedMangroveLog(b) => b.resource_location(),
            Self::StrippedBambooBlock(b) => b.resource_location(),
            Self::OakWood(b) => b.resource_location(),
            Self::SpruceWood(b) => b.resource_location(),
            Self::BirchWood(b) => b.resource_location(),
            Self::JungleWood(b) => b.resource_location(),
            Self::AcaciaWood(b) => b.resource_location(),
            Self::CherryWood(b) => b.resource_location(),
            Self::DarkOakWood(b) => b.resource_location(),
            Self::MangroveWood(b) => b.resource_location(),
            Self::StrippedOakWood(b) => b.resource_location(),
            Self::StrippedSpruceWood(b) => b.resource_location(),
            Self::StrippedBirchWood(b) => b.resource_location(),
            Self::StrippedJungleWood(b) => b.resource_location(),
            Self::StrippedAcaciaWood(b) => b.resource_location(),
            Self::StrippedCherryWood(b) => b.resource_location(),
            Self::StrippedDarkOakWood(b) => b.resource_location(),
            Self::StrippedMangroveWood(b) => b.resource_location(),
            Self::OakLeaves(b) => b.resource_location(),
            Self::SpruceLeaves(b) => b.resource_location(),
            Self::BirchLeaves(b) => b.resource_location(),
            Self::JungleLeaves(b) => b.resource_location(),
            Self::AcaciaLeaves(b) => b.resource_location(),
            Self::CherryLeaves(b) => b.resource_location(),
            Self::DarkOakLeaves(b) => b.resource_location(),
            Self::MangroveLeaves(b) => b.resource_location(),
            Self::AzaleaLeaves(b) => b.resource_location(),
            Self::FloweringAzaleaLeaves(b) => b.resource_location(),
            Self::Sponge(b) => b.resource_location(),
            Self::WetSponge(b) => b.resource_location(),
            Self::Glass(b) => b.resource_location(),
            Self::LapisOre(b) => b.resource_location(),
            Self::DeepslateLapisOre(b) => b.resource_location(),
            Self::LapisBlock(b) => b.resource_location(),
            Self::Dispenser(b) => b.resource_location(),
            Self::Sandstone(b) => b.resource_location(),
            Self::ChiseledSandstone(b) => b.resource_location(),
            Self::CutSandstone(b) => b.resource_location(),
            Self::NoteBlock(b) => b.resource_location(),
            Self::WhiteBed(b) => b.resource_location(),
            Self::OrangeBed(b) => b.resource_location(),
            Self::MagentaBed(b) => b.resource_location(),
            Self::LightBlueBed(b) => b.resource_location(),
            Self::YellowBed(b) => b.resource_location(),
            Self::LimeBed(b) => b.resource_location(),
            Self::PinkBed(b) => b.resource_location(),
            Self::GrayBed(b) => b.resource_location(),
            Self::LightGrayBed(b) => b.resource_location(),
            Self::CyanBed(b) => b.resource_location(),
            Self::PurpleBed(b) => b.resource_location(),
            Self::BlueBed(b) => b.resource_location(),
            Self::BrownBed(b) => b.resource_location(),
            Self::GreenBed(b) => b.resource_location(),
            Self::RedBed(b) => b.resource_location(),
            Self::BlackBed(b) => b.resource_location(),
            Self::PoweredRail(b) => b.resource_location(),
            Self::DetectorRail(b) => b.resource_location(),
            Self::StickyPiston(b) => b.resource_location(),
            Self::Cobweb(b) => b.resource_location(),
            Self::Grass(b) => b.resource_location(),
            Self::Fern(b) => b.resource_location(),
            Self::DeadBush(b) => b.resource_location(),
            Self::Seagrass(b) => b.resource_location(),
            Self::TallSeagrass(b) => b.resource_location(),
            Self::Piston(b) => b.resource_location(),
            Self::PistonHead(b) => b.resource_location(),
            Self::WhiteWool(b) => b.resource_location(),
            Self::OrangeWool(b) => b.resource_location(),
            Self::MagentaWool(b) => b.resource_location(),
            Self::LightBlueWool(b) => b.resource_location(),
            Self::YellowWool(b) => b.resource_location(),
            Self::LimeWool(b) => b.resource_location(),
            Self::PinkWool(b) => b.resource_location(),
            Self::GrayWool(b) => b.resource_location(),
            Self::LightGrayWool(b) => b.resource_location(),
            Self::CyanWool(b) => b.resource_location(),
            Self::PurpleWool(b) => b.resource_location(),
            Self::BlueWool(b) => b.resource_location(),
            Self::BrownWool(b) => b.resource_location(),
            Self::GreenWool(b) => b.resource_location(),
            Self::RedWool(b) => b.resource_location(),
            Self::BlackWool(b) => b.resource_location(),
            Self::MovingPiston(b) => b.resource_location(),
            Self::Dandelion(b) => b.resource_location(),
            Self::Torchflower(b) => b.resource_location(),
            Self::Poppy(b) => b.resource_location(),
            Self::BlueOrchid(b) => b.resource_location(),
            Self::Allium(b) => b.resource_location(),
            Self::AzureBluet(b) => b.resource_location(),
            Self::RedTulip(b) => b.resource_location(),
            Self::OrangeTulip(b) => b.resource_location(),
            Self::WhiteTulip(b) => b.resource_location(),
            Self::PinkTulip(b) => b.resource_location(),
            Self::OxeyeDaisy(b) => b.resource_location(),
            Self::Cornflower(b) => b.resource_location(),
            Self::WitherRose(b) => b.resource_location(),
            Self::LilyOfTheValley(b) => b.resource_location(),
            Self::BrownMushroom(b) => b.resource_location(),
            Self::RedMushroom(b) => b.resource_location(),
            Self::GoldBlock(b) => b.resource_location(),
            Self::IronBlock(b) => b.resource_location(),
            Self::Bricks(b) => b.resource_location(),
            Self::Tnt(b) => b.resource_location(),
            Self::Bookshelf(b) => b.resource_location(),
            Self::ChiseledBookshelf(b) => b.resource_location(),
            Self::MossyCobblestone(b) => b.resource_location(),
            Self::Obsidian(b) => b.resource_location(),
            Self::Torch(b) => b.resource_location(),
            Self::WallTorch(b) => b.resource_location(),
            Self::Fire(b) => b.resource_location(),
            Self::SoulFire(b) => b.resource_location(),
            Self::Spawner(b) => b.resource_location(),
            Self::OakStairs(b) => b.resource_location(),
            Self::Chest(b) => b.resource_location(),
            Self::RedstoneWire(b) => b.resource_location(),
            Self::DiamondOre(b) => b.resource_location(),
            Self::DeepslateDiamondOre(b) => b.resource_location(),
            Self::DiamondBlock(b) => b.resource_location(),
            Self::CraftingTable(b) => b.resource_location(),
            Self::Wheat(b) => b.resource_location(),
            Self::Farmland(b) => b.resource_location(),
            Self::Furnace(b) => b.resource_location(),
            Self::OakSign(b) => b.resource_location(),
            Self::SpruceSign(b) => b.resource_location(),
            Self::BirchSign(b) => b.resource_location(),
            Self::AcaciaSign(b) => b.resource_location(),
            Self::CherrySign(b) => b.resource_location(),
            Self::JungleSign(b) => b.resource_location(),
            Self::DarkOakSign(b) => b.resource_location(),
            Self::MangroveSign(b) => b.resource_location(),
            Self::BambooSign(b) => b.resource_location(),
            Self::OakDoor(b) => b.resource_location(),
            Self::Ladder(b) => b.resource_location(),
            Self::Rail(b) => b.resource_location(),
            Self::CobblestoneStairs(b) => b.resource_location(),
            Self::OakWallSign(b) => b.resource_location(),
            Self::SpruceWallSign(b) => b.resource_location(),
            Self::BirchWallSign(b) => b.resource_location(),
            Self::AcaciaWallSign(b) => b.resource_location(),
            Self::CherryWallSign(b) => b.resource_location(),
            Self::JungleWallSign(b) => b.resource_location(),
            Self::DarkOakWallSign(b) => b.resource_location(),
            Self::MangroveWallSign(b) => b.resource_location(),
            Self::BambooWallSign(b) => b.resource_location(),
            Self::OakHangingSign(b) => b.resource_location(),
            Self::SpruceHangingSign(b) => b.resource_location(),
            Self::BirchHangingSign(b) => b.resource_location(),
            Self::AcaciaHangingSign(b) => b.resource_location(),
            Self::CherryHangingSign(b) => b.resource_location(),
            Self::JungleHangingSign(b) => b.resource_location(),
            Self::DarkOakHangingSign(b) => b.resource_location(),
            Self::CrimsonHangingSign(b) => b.resource_location(),
            Self::WarpedHangingSign(b) => b.resource_location(),
            Self::MangroveHangingSign(b) => b.resource_location(),
            Self::BambooHangingSign(b) => b.resource_location(),
            Self::OakWallHangingSign(b) => b.resource_location(),
            Self::SpruceWallHangingSign(b) => b.resource_location(),
            Self::BirchWallHangingSign(b) => b.resource_location(),
            Self::AcaciaWallHangingSign(b) => b.resource_location(),
            Self::CherryWallHangingSign(b) => b.resource_location(),
            Self::JungleWallHangingSign(b) => b.resource_location(),
            Self::DarkOakWallHangingSign(b) => b.resource_location(),
            Self::MangroveWallHangingSign(b) => b.resource_location(),
            Self::CrimsonWallHangingSign(b) => b.resource_location(),
            Self::WarpedWallHangingSign(b) => b.resource_location(),
            Self::BambooWallHangingSign(b) => b.resource_location(),
            Self::Lever(b) => b.resource_location(),
            Self::StonePressurePlate(b) => b.resource_location(),
            Self::IronDoor(b) => b.resource_location(),
            Self::OakPressurePlate(b) => b.resource_location(),
            Self::SprucePressurePlate(b) => b.resource_location(),
            Self::BirchPressurePlate(b) => b.resource_location(),
            Self::JunglePressurePlate(b) => b.resource_location(),
            Self::AcaciaPressurePlate(b) => b.resource_location(),
            Self::CherryPressurePlate(b) => b.resource_location(),
            Self::DarkOakPressurePlate(b) => b.resource_location(),
            Self::MangrovePressurePlate(b) => b.resource_location(),
            Self::BambooPressurePlate(b) => b.resource_location(),
            Self::RedstoneOre(b) => b.resource_location(),
            Self::DeepslateRedstoneOre(b) => b.resource_location(),
            Self::RedstoneTorch(b) => b.resource_location(),
            Self::RedstoneWallTorch(b) => b.resource_location(),
            Self::StoneButton(b) => b.resource_location(),
            Self::Snow(b) => b.resource_location(),
            Self::Ice(b) => b.resource_location(),
            Self::SnowBlock(b) => b.resource_location(),
            Self::Cactus(b) => b.resource_location(),
            Self::Clay(b) => b.resource_location(),
            Self::SugarCane(b) => b.resource_location(),
            Self::Jukebox(b) => b.resource_location(),
            Self::OakFence(b) => b.resource_location(),
            Self::Pumpkin(b) => b.resource_location(),
            Self::Netherrack(b) => b.resource_location(),
            Self::SoulSand(b) => b.resource_location(),
            Self::SoulSoil(b) => b.resource_location(),
            Self::Basalt(b) => b.resource_location(),
            Self::PolishedBasalt(b) => b.resource_location(),
            Self::SoulTorch(b) => b.resource_location(),
            Self::SoulWallTorch(b) => b.resource_location(),
            Self::Glowstone(b) => b.resource_location(),
            Self::NetherPortal(b) => b.resource_location(),
            Self::CarvedPumpkin(b) => b.resource_location(),
            Self::JackOLantern(b) => b.resource_location(),
            Self::Cake(b) => b.resource_location(),
            Self::Repeater(b) => b.resource_location(),
            Self::WhiteStainedGlass(b) => b.resource_location(),
            Self::OrangeStainedGlass(b) => b.resource_location(),
            Self::MagentaStainedGlass(b) => b.resource_location(),
            Self::LightBlueStainedGlass(b) => b.resource_location(),
            Self::YellowStainedGlass(b) => b.resource_location(),
            Self::LimeStainedGlass(b) => b.resource_location(),
            Self::PinkStainedGlass(b) => b.resource_location(),
            Self::GrayStainedGlass(b) => b.resource_location(),
            Self::LightGrayStainedGlass(b) => b.resource_location(),
            Self::CyanStainedGlass(b) => b.resource_location(),
            Self::PurpleStainedGlass(b) => b.resource_location(),
            Self::BlueStainedGlass(b) => b.resource_location(),
            Self::BrownStainedGlass(b) => b.resource_location(),
            Self::GreenStainedGlass(b) => b.resource_location(),
            Self::RedStainedGlass(b) => b.resource_location(),
            Self::BlackStainedGlass(b) => b.resource_location(),
            Self::OakTrapdoor(b) => b.resource_location(),
            Self::SpruceTrapdoor(b) => b.resource_location(),
            Self::BirchTrapdoor(b) => b.resource_location(),
            Self::JungleTrapdoor(b) => b.resource_location(),
            Self::AcaciaTrapdoor(b) => b.resource_location(),
            Self::CherryTrapdoor(b) => b.resource_location(),
            Self::DarkOakTrapdoor(b) => b.resource_location(),
            Self::MangroveTrapdoor(b) => b.resource_location(),
            Self::BambooTrapdoor(b) => b.resource_location(),
            Self::StoneBricks(b) => b.resource_location(),
            Self::MossyStoneBricks(b) => b.resource_location(),
            Self::CrackedStoneBricks(b) => b.resource_location(),
            Self::ChiseledStoneBricks(b) => b.resource_location(),
            Self::PackedMud(b) => b.resource_location(),
            Self::MudBricks(b) => b.resource_location(),
            Self::InfestedStone(b) => b.resource_location(),
            Self::InfestedCobblestone(b) => b.resource_location(),
            Self::InfestedStoneBricks(b) => b.resource_location(),
            Self::InfestedMossyStoneBricks(b) => b.resource_location(),
            Self::InfestedCrackedStoneBricks(b) => b.resource_location(),
            Self::InfestedChiseledStoneBricks(b) => b.resource_location(),
            Self::BrownMushroomBlock(b) => b.resource_location(),
            Self::RedMushroomBlock(b) => b.resource_location(),
            Self::MushroomStem(b) => b.resource_location(),
            Self::IronBars(b) => b.resource_location(),
            Self::Chain(b) => b.resource_location(),
            Self::GlassPane(b) => b.resource_location(),
            Self::Melon(b) => b.resource_location(),
            Self::AttachedPumpkinStem(b) => b.resource_location(),
            Self::AttachedMelonStem(b) => b.resource_location(),
            Self::PumpkinStem(b) => b.resource_location(),
            Self::MelonStem(b) => b.resource_location(),
            Self::Vine(b) => b.resource_location(),
            Self::GlowLichen(b) => b.resource_location(),
            Self::OakFenceGate(b) => b.resource_location(),
            Self::BrickStairs(b) => b.resource_location(),
            Self::StoneBrickStairs(b) => b.resource_location(),
            Self::MudBrickStairs(b) => b.resource_location(),
            Self::Mycelium(b) => b.resource_location(),
            Self::LilyPad(b) => b.resource_location(),
            Self::NetherBricks(b) => b.resource_location(),
            Self::NetherBrickFence(b) => b.resource_location(),
            Self::NetherBrickStairs(b) => b.resource_location(),
            Self::NetherWart(b) => b.resource_location(),
            Self::EnchantingTable(b) => b.resource_location(),
            Self::BrewingStand(b) => b.resource_location(),
            Self::Cauldron(b) => b.resource_location(),
            Self::WaterCauldron(b) => b.resource_location(),
            Self::LavaCauldron(b) => b.resource_location(),
            Self::PowderSnowCauldron(b) => b.resource_location(),
            Self::EndPortal(b) => b.resource_location(),
            Self::EndPortalFrame(b) => b.resource_location(),
            Self::EndStone(b) => b.resource_location(),
            Self::DragonEgg(b) => b.resource_location(),
            Self::RedstoneLamp(b) => b.resource_location(),
            Self::Cocoa(b) => b.resource_location(),
            Self::SandstoneStairs(b) => b.resource_location(),
            Self::EmeraldOre(b) => b.resource_location(),
            Self::DeepslateEmeraldOre(b) => b.resource_location(),
            Self::EnderChest(b) => b.resource_location(),
            Self::TripwireHook(b) => b.resource_location(),
            Self::Tripwire(b) => b.resource_location(),
            Self::EmeraldBlock(b) => b.resource_location(),
            Self::SpruceStairs(b) => b.resource_location(),
            Self::BirchStairs(b) => b.resource_location(),
            Self::JungleStairs(b) => b.resource_location(),
            Self::CommandBlock(b) => b.resource_location(),
            Self::Beacon(b) => b.resource_location(),
            Self::CobblestoneWall(b) => b.resource_location(),
            Self::MossyCobblestoneWall(b) => b.resource_location(),
            Self::FlowerPot(b) => b.resource_location(),
            Self::PottedTorchflower(b) => b.resource_location(),
            Self::PottedOakSapling(b) => b.resource_location(),
            Self::PottedSpruceSapling(b) => b.resource_location(),
            Self::PottedBirchSapling(b) => b.resource_location(),
            Self::PottedJungleSapling(b) => b.resource_location(),
            Self::PottedAcaciaSapling(b) => b.resource_location(),
            Self::PottedCherrySapling(b) => b.resource_location(),
            Self::PottedDarkOakSapling(b) => b.resource_location(),
            Self::PottedMangrovePropagule(b) => b.resource_location(),
            Self::PottedFern(b) => b.resource_location(),
            Self::PottedDandelion(b) => b.resource_location(),
            Self::PottedPoppy(b) => b.resource_location(),
            Self::PottedBlueOrchid(b) => b.resource_location(),
            Self::PottedAllium(b) => b.resource_location(),
            Self::PottedAzureBluet(b) => b.resource_location(),
            Self::PottedRedTulip(b) => b.resource_location(),
            Self::PottedOrangeTulip(b) => b.resource_location(),
            Self::PottedWhiteTulip(b) => b.resource_location(),
            Self::PottedPinkTulip(b) => b.resource_location(),
            Self::PottedOxeyeDaisy(b) => b.resource_location(),
            Self::PottedCornflower(b) => b.resource_location(),
            Self::PottedLilyOfTheValley(b) => b.resource_location(),
            Self::PottedWitherRose(b) => b.resource_location(),
            Self::PottedRedMushroom(b) => b.resource_location(),
            Self::PottedBrownMushroom(b) => b.resource_location(),
            Self::PottedDeadBush(b) => b.resource_location(),
            Self::PottedCactus(b) => b.resource_location(),
            Self::Carrots(b) => b.resource_location(),
            Self::Potatoes(b) => b.resource_location(),
            Self::OakButton(b) => b.resource_location(),
            Self::SpruceButton(b) => b.resource_location(),
            Self::BirchButton(b) => b.resource_location(),
            Self::JungleButton(b) => b.resource_location(),
            Self::AcaciaButton(b) => b.resource_location(),
            Self::CherryButton(b) => b.resource_location(),
            Self::DarkOakButton(b) => b.resource_location(),
            Self::MangroveButton(b) => b.resource_location(),
            Self::BambooButton(b) => b.resource_location(),
            Self::SkeletonSkull(b) => b.resource_location(),
            Self::SkeletonWallSkull(b) => b.resource_location(),
            Self::WitherSkeletonSkull(b) => b.resource_location(),
            Self::WitherSkeletonWallSkull(b) => b.resource_location(),
            Self::ZombieHead(b) => b.resource_location(),
            Self::ZombieWallHead(b) => b.resource_location(),
            Self::PlayerHead(b) => b.resource_location(),
            Self::PlayerWallHead(b) => b.resource_location(),
            Self::CreeperHead(b) => b.resource_location(),
            Self::CreeperWallHead(b) => b.resource_location(),
            Self::DragonHead(b) => b.resource_location(),
            Self::DragonWallHead(b) => b.resource_location(),
            Self::PiglinHead(b) => b.resource_location(),
            Self::PiglinWallHead(b) => b.resource_location(),
            Self::Anvil(b) => b.resource_location(),
            Self::ChippedAnvil(b) => b.resource_location(),
            Self::DamagedAnvil(b) => b.resource_location(),
            Self::TrappedChest(b) => b.resource_location(),
            Self::LightWeightedPressurePlate(b) => b.resource_location(),
            Self::HeavyWeightedPressurePlate(b) => b.resource_location(),
            Self::Comparator(b) => b.resource_location(),
            Self::DaylightDetector(b) => b.resource_location(),
            Self::RedstoneBlock(b) => b.resource_location(),
            Self::NetherQuartzOre(b) => b.resource_location(),
            Self::Hopper(b) => b.resource_location(),
            Self::QuartzBlock(b) => b.resource_location(),
            Self::ChiseledQuartzBlock(b) => b.resource_location(),
            Self::QuartzPillar(b) => b.resource_location(),
            Self::QuartzStairs(b) => b.resource_location(),
            Self::ActivatorRail(b) => b.resource_location(),
            Self::Dropper(b) => b.resource_location(),
            Self::WhiteTerracotta(b) => b.resource_location(),
            Self::OrangeTerracotta(b) => b.resource_location(),
            Self::MagentaTerracotta(b) => b.resource_location(),
            Self::LightBlueTerracotta(b) => b.resource_location(),
            Self::YellowTerracotta(b) => b.resource_location(),
            Self::LimeTerracotta(b) => b.resource_location(),
            Self::PinkTerracotta(b) => b.resource_location(),
            Self::GrayTerracotta(b) => b.resource_location(),
            Self::LightGrayTerracotta(b) => b.resource_location(),
            Self::CyanTerracotta(b) => b.resource_location(),
            Self::PurpleTerracotta(b) => b.resource_location(),
            Self::BlueTerracotta(b) => b.resource_location(),
            Self::BrownTerracotta(b) => b.resource_location(),
            Self::GreenTerracotta(b) => b.resource_location(),
            Self::RedTerracotta(b) => b.resource_location(),
            Self::BlackTerracotta(b) => b.resource_location(),
            Self::WhiteStainedGlassPane(b) => b.resource_location(),
            Self::OrangeStainedGlassPane(b) => b.resource_location(),
            Self::MagentaStainedGlassPane(b) => b.resource_location(),
            Self::LightBlueStainedGlassPane(b) => b.resource_location(),
            Self::YellowStainedGlassPane(b) => b.resource_location(),
            Self::LimeStainedGlassPane(b) => b.resource_location(),
            Self::PinkStainedGlassPane(b) => b.resource_location(),
            Self::GrayStainedGlassPane(b) => b.resource_location(),
            Self::LightGrayStainedGlassPane(b) => b.resource_location(),
            Self::CyanStainedGlassPane(b) => b.resource_location(),
            Self::PurpleStainedGlassPane(b) => b.resource_location(),
            Self::BlueStainedGlassPane(b) => b.resource_location(),
            Self::BrownStainedGlassPane(b) => b.resource_location(),
            Self::GreenStainedGlassPane(b) => b.resource_location(),
            Self::RedStainedGlassPane(b) => b.resource_location(),
            Self::BlackStainedGlassPane(b) => b.resource_location(),
            Self::AcaciaStairs(b) => b.resource_location(),
            Self::CherryStairs(b) => b.resource_location(),
            Self::DarkOakStairs(b) => b.resource_location(),
            Self::MangroveStairs(b) => b.resource_location(),
            Self::BambooStairs(b) => b.resource_location(),
            Self::BambooMosaicStairs(b) => b.resource_location(),
            Self::SlimeBlock(b) => b.resource_location(),
            Self::Barrier(b) => b.resource_location(),
            Self::Light(b) => b.resource_location(),
            Self::IronTrapdoor(b) => b.resource_location(),
            Self::Prismarine(b) => b.resource_location(),
            Self::PrismarineBricks(b) => b.resource_location(),
            Self::DarkPrismarine(b) => b.resource_location(),
            Self::PrismarineStairs(b) => b.resource_location(),
            Self::PrismarineBrickStairs(b) => b.resource_location(),
            Self::DarkPrismarineStairs(b) => b.resource_location(),
            Self::PrismarineSlab(b) => b.resource_location(),
            Self::PrismarineBrickSlab(b) => b.resource_location(),
            Self::DarkPrismarineSlab(b) => b.resource_location(),
            Self::SeaLantern(b) => b.resource_location(),
            Self::HayBlock(b) => b.resource_location(),
            Self::WhiteCarpet(b) => b.resource_location(),
            Self::OrangeCarpet(b) => b.resource_location(),
            Self::MagentaCarpet(b) => b.resource_location(),
            Self::LightBlueCarpet(b) => b.resource_location(),
            Self::YellowCarpet(b) => b.resource_location(),
            Self::LimeCarpet(b) => b.resource_location(),
            Self::PinkCarpet(b) => b.resource_location(),
            Self::GrayCarpet(b) => b.resource_location(),
            Self::LightGrayCarpet(b) => b.resource_location(),
            Self::CyanCarpet(b) => b.resource_location(),
            Self::PurpleCarpet(b) => b.resource_location(),
            Self::BlueCarpet(b) => b.resource_location(),
            Self::BrownCarpet(b) => b.resource_location(),
            Self::GreenCarpet(b) => b.resource_location(),
            Self::RedCarpet(b) => b.resource_location(),
            Self::BlackCarpet(b) => b.resource_location(),
            Self::Terracotta(b) => b.resource_location(),
            Self::CoalBlock(b) => b.resource_location(),
            Self::PackedIce(b) => b.resource_location(),
            Self::Sunflower(b) => b.resource_location(),
            Self::Lilac(b) => b.resource_location(),
            Self::RoseBush(b) => b.resource_location(),
            Self::Peony(b) => b.resource_location(),
            Self::TallGrass(b) => b.resource_location(),
            Self::LargeFern(b) => b.resource_location(),
            Self::WhiteBanner(b) => b.resource_location(),
            Self::OrangeBanner(b) => b.resource_location(),
            Self::MagentaBanner(b) => b.resource_location(),
            Self::LightBlueBanner(b) => b.resource_location(),
            Self::YellowBanner(b) => b.resource_location(),
            Self::LimeBanner(b) => b.resource_location(),
            Self::PinkBanner(b) => b.resource_location(),
            Self::GrayBanner(b) => b.resource_location(),
            Self::LightGrayBanner(b) => b.resource_location(),
            Self::CyanBanner(b) => b.resource_location(),
            Self::PurpleBanner(b) => b.resource_location(),
            Self::BlueBanner(b) => b.resource_location(),
            Self::BrownBanner(b) => b.resource_location(),
            Self::GreenBanner(b) => b.resource_location(),
            Self::RedBanner(b) => b.resource_location(),
            Self::BlackBanner(b) => b.resource_location(),
            Self::WhiteWallBanner(b) => b.resource_location(),
            Self::OrangeWallBanner(b) => b.resource_location(),
            Self::MagentaWallBanner(b) => b.resource_location(),
            Self::LightBlueWallBanner(b) => b.resource_location(),
            Self::YellowWallBanner(b) => b.resource_location(),
            Self::LimeWallBanner(b) => b.resource_location(),
            Self::PinkWallBanner(b) => b.resource_location(),
            Self::GrayWallBanner(b) => b.resource_location(),
            Self::LightGrayWallBanner(b) => b.resource_location(),
            Self::CyanWallBanner(b) => b.resource_location(),
            Self::PurpleWallBanner(b) => b.resource_location(),
            Self::BlueWallBanner(b) => b.resource_location(),
            Self::BrownWallBanner(b) => b.resource_location(),
            Self::GreenWallBanner(b) => b.resource_location(),
            Self::RedWallBanner(b) => b.resource_location(),
            Self::BlackWallBanner(b) => b.resource_location(),
            Self::RedSandstone(b) => b.resource_location(),
            Self::ChiseledRedSandstone(b) => b.resource_location(),
            Self::CutRedSandstone(b) => b.resource_location(),
            Self::RedSandstoneStairs(b) => b.resource_location(),
            Self::OakSlab(b) => b.resource_location(),
            Self::SpruceSlab(b) => b.resource_location(),
            Self::BirchSlab(b) => b.resource_location(),
            Self::JungleSlab(b) => b.resource_location(),
            Self::AcaciaSlab(b) => b.resource_location(),
            Self::CherrySlab(b) => b.resource_location(),
            Self::DarkOakSlab(b) => b.resource_location(),
            Self::MangroveSlab(b) => b.resource_location(),
            Self::BambooSlab(b) => b.resource_location(),
            Self::BambooMosaicSlab(b) => b.resource_location(),
            Self::StoneSlab(b) => b.resource_location(),
            Self::SmoothStoneSlab(b) => b.resource_location(),
            Self::SandstoneSlab(b) => b.resource_location(),
            Self::CutSandstoneSlab(b) => b.resource_location(),
            Self::PetrifiedOakSlab(b) => b.resource_location(),
            Self::CobblestoneSlab(b) => b.resource_location(),
            Self::BrickSlab(b) => b.resource_location(),
            Self::StoneBrickSlab(b) => b.resource_location(),
            Self::MudBrickSlab(b) => b.resource_location(),
            Self::NetherBrickSlab(b) => b.resource_location(),
            Self::QuartzSlab(b) => b.resource_location(),
            Self::RedSandstoneSlab(b) => b.resource_location(),
            Self::CutRedSandstoneSlab(b) => b.resource_location(),
            Self::PurpurSlab(b) => b.resource_location(),
            Self::SmoothStone(b) => b.resource_location(),
            Self::SmoothSandstone(b) => b.resource_location(),
            Self::SmoothQuartz(b) => b.resource_location(),
            Self::SmoothRedSandstone(b) => b.resource_location(),
            Self::SpruceFenceGate(b) => b.resource_location(),
            Self::BirchFenceGate(b) => b.resource_location(),
            Self::JungleFenceGate(b) => b.resource_location(),
            Self::AcaciaFenceGate(b) => b.resource_location(),
            Self::CherryFenceGate(b) => b.resource_location(),
            Self::DarkOakFenceGate(b) => b.resource_location(),
            Self::MangroveFenceGate(b) => b.resource_location(),
            Self::BambooFenceGate(b) => b.resource_location(),
            Self::SpruceFence(b) => b.resource_location(),
            Self::BirchFence(b) => b.resource_location(),
            Self::JungleFence(b) => b.resource_location(),
            Self::AcaciaFence(b) => b.resource_location(),
            Self::CherryFence(b) => b.resource_location(),
            Self::DarkOakFence(b) => b.resource_location(),
            Self::MangroveFence(b) => b.resource_location(),
            Self::BambooFence(b) => b.resource_location(),
            Self::SpruceDoor(b) => b.resource_location(),
            Self::BirchDoor(b) => b.resource_location(),
            Self::JungleDoor(b) => b.resource_location(),
            Self::AcaciaDoor(b) => b.resource_location(),
            Self::CherryDoor(b) => b.resource_location(),
            Self::DarkOakDoor(b) => b.resource_location(),
            Self::MangroveDoor(b) => b.resource_location(),
            Self::BambooDoor(b) => b.resource_location(),
            Self::EndRod(b) => b.resource_location(),
            Self::ChorusPlant(b) => b.resource_location(),
            Self::ChorusFlower(b) => b.resource_location(),
            Self::PurpurBlock(b) => b.resource_location(),
            Self::PurpurPillar(b) => b.resource_location(),
            Self::PurpurStairs(b) => b.resource_location(),
            Self::EndStoneBricks(b) => b.resource_location(),
            Self::TorchflowerCrop(b) => b.resource_location(),
            Self::PitcherCrop(b) => b.resource_location(),
            Self::PitcherPlant(b) => b.resource_location(),
            Self::Beetroots(b) => b.resource_location(),
            Self::DirtPath(b) => b.resource_location(),
            Self::EndGateway(b) => b.resource_location(),
            Self::RepeatingCommandBlock(b) => b.resource_location(),
            Self::ChainCommandBlock(b) => b.resource_location(),
            Self::FrostedIce(b) => b.resource_location(),
            Self::MagmaBlock(b) => b.resource_location(),
            Self::NetherWartBlock(b) => b.resource_location(),
            Self::RedNetherBricks(b) => b.resource_location(),
            Self::BoneBlock(b) => b.resource_location(),
            Self::StructureVoid(b) => b.resource_location(),
            Self::Observer(b) => b.resource_location(),
            Self::ShulkerBox(b) => b.resource_location(),
            Self::WhiteShulkerBox(b) => b.resource_location(),
            Self::OrangeShulkerBox(b) => b.resource_location(),
            Self::MagentaShulkerBox(b) => b.resource_location(),
            Self::LightBlueShulkerBox(b) => b.resource_location(),
            Self::YellowShulkerBox(b) => b.resource_location(),
            Self::LimeShulkerBox(b) => b.resource_location(),
            Self::PinkShulkerBox(b) => b.resource_location(),
            Self::GrayShulkerBox(b) => b.resource_location(),
            Self::LightGrayShulkerBox(b) => b.resource_location(),
            Self::CyanShulkerBox(b) => b.resource_location(),
            Self::PurpleShulkerBox(b) => b.resource_location(),
            Self::BlueShulkerBox(b) => b.resource_location(),
            Self::BrownShulkerBox(b) => b.resource_location(),
            Self::GreenShulkerBox(b) => b.resource_location(),
            Self::RedShulkerBox(b) => b.resource_location(),
            Self::BlackShulkerBox(b) => b.resource_location(),
            Self::WhiteGlazedTerracotta(b) => b.resource_location(),
            Self::OrangeGlazedTerracotta(b) => b.resource_location(),
            Self::MagentaGlazedTerracotta(b) => b.resource_location(),
            Self::LightBlueGlazedTerracotta(b) => b.resource_location(),
            Self::YellowGlazedTerracotta(b) => b.resource_location(),
            Self::LimeGlazedTerracotta(b) => b.resource_location(),
            Self::PinkGlazedTerracotta(b) => b.resource_location(),
            Self::GrayGlazedTerracotta(b) => b.resource_location(),
            Self::LightGrayGlazedTerracotta(b) => b.resource_location(),
            Self::CyanGlazedTerracotta(b) => b.resource_location(),
            Self::PurpleGlazedTerracotta(b) => b.resource_location(),
            Self::BlueGlazedTerracotta(b) => b.resource_location(),
            Self::BrownGlazedTerracotta(b) => b.resource_location(),
            Self::GreenGlazedTerracotta(b) => b.resource_location(),
            Self::RedGlazedTerracotta(b) => b.resource_location(),
            Self::BlackGlazedTerracotta(b) => b.resource_location(),
            Self::WhiteConcrete(b) => b.resource_location(),
            Self::OrangeConcrete(b) => b.resource_location(),
            Self::MagentaConcrete(b) => b.resource_location(),
            Self::LightBlueConcrete(b) => b.resource_location(),
            Self::YellowConcrete(b) => b.resource_location(),
            Self::LimeConcrete(b) => b.resource_location(),
            Self::PinkConcrete(b) => b.resource_location(),
            Self::GrayConcrete(b) => b.resource_location(),
            Self::LightGrayConcrete(b) => b.resource_location(),
            Self::CyanConcrete(b) => b.resource_location(),
            Self::PurpleConcrete(b) => b.resource_location(),
            Self::BlueConcrete(b) => b.resource_location(),
            Self::BrownConcrete(b) => b.resource_location(),
            Self::GreenConcrete(b) => b.resource_location(),
            Self::RedConcrete(b) => b.resource_location(),
            Self::BlackConcrete(b) => b.resource_location(),
            Self::WhiteConcretePowder(b) => b.resource_location(),
            Self::OrangeConcretePowder(b) => b.resource_location(),
            Self::MagentaConcretePowder(b) => b.resource_location(),
            Self::LightBlueConcretePowder(b) => b.resource_location(),
            Self::YellowConcretePowder(b) => b.resource_location(),
            Self::LimeConcretePowder(b) => b.resource_location(),
            Self::PinkConcretePowder(b) => b.resource_location(),
            Self::GrayConcretePowder(b) => b.resource_location(),
            Self::LightGrayConcretePowder(b) => b.resource_location(),
            Self::CyanConcretePowder(b) => b.resource_location(),
            Self::PurpleConcretePowder(b) => b.resource_location(),
            Self::BlueConcretePowder(b) => b.resource_location(),
            Self::BrownConcretePowder(b) => b.resource_location(),
            Self::GreenConcretePowder(b) => b.resource_location(),
            Self::RedConcretePowder(b) => b.resource_location(),
            Self::BlackConcretePowder(b) => b.resource_location(),
            Self::Kelp(b) => b.resource_location(),
            Self::KelpPlant(b) => b.resource_location(),
            Self::DriedKelpBlock(b) => b.resource_location(),
            Self::TurtleEgg(b) => b.resource_location(),
            Self::SnifferEgg(b) => b.resource_location(),
            Self::DeadTubeCoralBlock(b) => b.resource_location(),
            Self::DeadBrainCoralBlock(b) => b.resource_location(),
            Self::DeadBubbleCoralBlock(b) => b.resource_location(),
            Self::DeadFireCoralBlock(b) => b.resource_location(),
            Self::DeadHornCoralBlock(b) => b.resource_location(),
            Self::TubeCoralBlock(b) => b.resource_location(),
            Self::BrainCoralBlock(b) => b.resource_location(),
            Self::BubbleCoralBlock(b) => b.resource_location(),
            Self::FireCoralBlock(b) => b.resource_location(),
            Self::HornCoralBlock(b) => b.resource_location(),
            Self::DeadTubeCoral(b) => b.resource_location(),
            Self::DeadBrainCoral(b) => b.resource_location(),
            Self::DeadBubbleCoral(b) => b.resource_location(),
            Self::DeadFireCoral(b) => b.resource_location(),
            Self::DeadHornCoral(b) => b.resource_location(),
            Self::TubeCoral(b) => b.resource_location(),
            Self::BrainCoral(b) => b.resource_location(),
            Self::BubbleCoral(b) => b.resource_location(),
            Self::FireCoral(b) => b.resource_location(),
            Self::HornCoral(b) => b.resource_location(),
            Self::DeadTubeCoralFan(b) => b.resource_location(),
            Self::DeadBrainCoralFan(b) => b.resource_location(),
            Self::DeadBubbleCoralFan(b) => b.resource_location(),
            Self::DeadFireCoralFan(b) => b.resource_location(),
            Self::DeadHornCoralFan(b) => b.resource_location(),
            Self::TubeCoralFan(b) => b.resource_location(),
            Self::BrainCoralFan(b) => b.resource_location(),
            Self::BubbleCoralFan(b) => b.resource_location(),
            Self::FireCoralFan(b) => b.resource_location(),
            Self::HornCoralFan(b) => b.resource_location(),
            Self::DeadTubeCoralWallFan(b) => b.resource_location(),
            Self::DeadBrainCoralWallFan(b) => b.resource_location(),
            Self::DeadBubbleCoralWallFan(b) => b.resource_location(),
            Self::DeadFireCoralWallFan(b) => b.resource_location(),
            Self::DeadHornCoralWallFan(b) => b.resource_location(),
            Self::TubeCoralWallFan(b) => b.resource_location(),
            Self::BrainCoralWallFan(b) => b.resource_location(),
            Self::BubbleCoralWallFan(b) => b.resource_location(),
            Self::FireCoralWallFan(b) => b.resource_location(),
            Self::HornCoralWallFan(b) => b.resource_location(),
            Self::SeaPickle(b) => b.resource_location(),
            Self::BlueIce(b) => b.resource_location(),
            Self::Conduit(b) => b.resource_location(),
            Self::BambooSapling(b) => b.resource_location(),
            Self::Bamboo(b) => b.resource_location(),
            Self::PottedBamboo(b) => b.resource_location(),
            Self::VoidAir(b) => b.resource_location(),
            Self::CaveAir(b) => b.resource_location(),
            Self::BubbleColumn(b) => b.resource_location(),
            Self::PolishedGraniteStairs(b) => b.resource_location(),
            Self::SmoothRedSandstoneStairs(b) => b.resource_location(),
            Self::MossyStoneBrickStairs(b) => b.resource_location(),
            Self::PolishedDioriteStairs(b) => b.resource_location(),
            Self::MossyCobblestoneStairs(b) => b.resource_location(),
            Self::EndStoneBrickStairs(b) => b.resource_location(),
            Self::StoneStairs(b) => b.resource_location(),
            Self::SmoothSandstoneStairs(b) => b.resource_location(),
            Self::SmoothQuartzStairs(b) => b.resource_location(),
            Self::GraniteStairs(b) => b.resource_location(),
            Self::AndesiteStairs(b) => b.resource_location(),
            Self::RedNetherBrickStairs(b) => b.resource_location(),
            Self::PolishedAndesiteStairs(b) => b.resource_location(),
            Self::DioriteStairs(b) => b.resource_location(),
            Self::PolishedGraniteSlab(b) => b.resource_location(),
            Self::SmoothRedSandstoneSlab(b) => b.resource_location(),
            Self::MossyStoneBrickSlab(b) => b.resource_location(),
            Self::PolishedDioriteSlab(b) => b.resource_location(),
            Self::MossyCobblestoneSlab(b) => b.resource_location(),
            Self::EndStoneBrickSlab(b) => b.resource_location(),
            Self::SmoothSandstoneSlab(b) => b.resource_location(),
            Self::SmoothQuartzSlab(b) => b.resource_location(),
            Self::GraniteSlab(b) => b.resource_location(),
            Self::AndesiteSlab(b) => b.resource_location(),
            Self::RedNetherBrickSlab(b) => b.resource_location(),
            Self::PolishedAndesiteSlab(b) => b.resource_location(),
            Self::DioriteSlab(b) => b.resource_location(),
            Self::BrickWall(b) => b.resource_location(),
            Self::PrismarineWall(b) => b.resource_location(),
            Self::RedSandstoneWall(b) => b.resource_location(),
            Self::MossyStoneBrickWall(b) => b.resource_location(),
            Self::GraniteWall(b) => b.resource_location(),
            Self::StoneBrickWall(b) => b.resource_location(),
            Self::MudBrickWall(b) => b.resource_location(),
            Self::NetherBrickWall(b) => b.resource_location(),
            Self::AndesiteWall(b) => b.resource_location(),
            Self::RedNetherBrickWall(b) => b.resource_location(),
            Self::SandstoneWall(b) => b.resource_location(),
            Self::EndStoneBrickWall(b) => b.resource_location(),
            Self::DioriteWall(b) => b.resource_location(),
            Self::Scaffolding(b) => b.resource_location(),
            Self::Loom(b) => b.resource_location(),
            Self::Barrel(b) => b.resource_location(),
            Self::Smoker(b) => b.resource_location(),
            Self::BlastFurnace(b) => b.resource_location(),
            Self::CartographyTable(b) => b.resource_location(),
            Self::FletchingTable(b) => b.resource_location(),
            Self::Grindstone(b) => b.resource_location(),
            Self::Lectern(b) => b.resource_location(),
            Self::SmithingTable(b) => b.resource_location(),
            Self::Stonecutter(b) => b.resource_location(),
            Self::Bell(b) => b.resource_location(),
            Self::Lantern(b) => b.resource_location(),
            Self::SoulLantern(b) => b.resource_location(),
            Self::Campfire(b) => b.resource_location(),
            Self::SoulCampfire(b) => b.resource_location(),
            Self::SweetBerryBush(b) => b.resource_location(),
            Self::WarpedStem(b) => b.resource_location(),
            Self::StrippedWarpedStem(b) => b.resource_location(),
            Self::WarpedHyphae(b) => b.resource_location(),
            Self::StrippedWarpedHyphae(b) => b.resource_location(),
            Self::WarpedNylium(b) => b.resource_location(),
            Self::WarpedFungus(b) => b.resource_location(),
            Self::WarpedWartBlock(b) => b.resource_location(),
            Self::WarpedRoots(b) => b.resource_location(),
            Self::NetherSprouts(b) => b.resource_location(),
            Self::CrimsonStem(b) => b.resource_location(),
            Self::StrippedCrimsonStem(b) => b.resource_location(),
            Self::CrimsonHyphae(b) => b.resource_location(),
            Self::StrippedCrimsonHyphae(b) => b.resource_location(),
            Self::CrimsonNylium(b) => b.resource_location(),
            Self::CrimsonFungus(b) => b.resource_location(),
            Self::Shroomlight(b) => b.resource_location(),
            Self::WeepingVines(b) => b.resource_location(),
            Self::WeepingVinesPlant(b) => b.resource_location(),
            Self::TwistingVines(b) => b.resource_location(),
            Self::TwistingVinesPlant(b) => b.resource_location(),
            Self::CrimsonRoots(b) => b.resource_location(),
            Self::CrimsonPlanks(b) => b.resource_location(),
            Self::WarpedPlanks(b) => b.resource_location(),
            Self::CrimsonSlab(b) => b.resource_location(),
            Self::WarpedSlab(b) => b.resource_location(),
            Self::CrimsonPressurePlate(b) => b.resource_location(),
            Self::WarpedPressurePlate(b) => b.resource_location(),
            Self::CrimsonFence(b) => b.resource_location(),
            Self::WarpedFence(b) => b.resource_location(),
            Self::CrimsonTrapdoor(b) => b.resource_location(),
            Self::WarpedTrapdoor(b) => b.resource_location(),
            Self::CrimsonFenceGate(b) => b.resource_location(),
            Self::WarpedFenceGate(b) => b.resource_location(),
            Self::CrimsonStairs(b) => b.resource_location(),
            Self::WarpedStairs(b) => b.resource_location(),
            Self::CrimsonButton(b) => b.resource_location(),
            Self::WarpedButton(b) => b.resource_location(),
            Self::CrimsonDoor(b) => b.resource_location(),
            Self::WarpedDoor(b) => b.resource_location(),
            Self::CrimsonSign(b) => b.resource_location(),
            Self::WarpedSign(b) => b.resource_location(),
            Self::CrimsonWallSign(b) => b.resource_location(),
            Self::WarpedWallSign(b) => b.resource_location(),
            Self::StructureBlock(b) => b.resource_location(),
            Self::Jigsaw(b) => b.resource_location(),
            Self::Composter(b) => b.resource_location(),
            Self::Target(b) => b.resource_location(),
            Self::BeeNest(b) => b.resource_location(),
            Self::Beehive(b) => b.resource_location(),
            Self::HoneyBlock(b) => b.resource_location(),
            Self::HoneycombBlock(b) => b.resource_location(),
            Self::NetheriteBlock(b) => b.resource_location(),
            Self::AncientDebris(b) => b.resource_location(),
            Self::CryingObsidian(b) => b.resource_location(),
            Self::RespawnAnchor(b) => b.resource_location(),
            Self::PottedCrimsonFungus(b) => b.resource_location(),
            Self::PottedWarpedFungus(b) => b.resource_location(),
            Self::PottedCrimsonRoots(b) => b.resource_location(),
            Self::PottedWarpedRoots(b) => b.resource_location(),
            Self::Lodestone(b) => b.resource_location(),
            Self::Blackstone(b) => b.resource_location(),
            Self::BlackstoneStairs(b) => b.resource_location(),
            Self::BlackstoneWall(b) => b.resource_location(),
            Self::BlackstoneSlab(b) => b.resource_location(),
            Self::PolishedBlackstone(b) => b.resource_location(),
            Self::PolishedBlackstoneBricks(b) => b.resource_location(),
            Self::CrackedPolishedBlackstoneBricks(b) => b.resource_location(),
            Self::ChiseledPolishedBlackstone(b) => b.resource_location(),
            Self::PolishedBlackstoneBrickSlab(b) => b.resource_location(),
            Self::PolishedBlackstoneBrickStairs(b) => b.resource_location(),
            Self::PolishedBlackstoneBrickWall(b) => b.resource_location(),
            Self::GildedBlackstone(b) => b.resource_location(),
            Self::PolishedBlackstoneStairs(b) => b.resource_location(),
            Self::PolishedBlackstoneSlab(b) => b.resource_location(),
            Self::PolishedBlackstonePressurePlate(b) => b.resource_location(),
            Self::PolishedBlackstoneButton(b) => b.resource_location(),
            Self::PolishedBlackstoneWall(b) => b.resource_location(),
            Self::ChiseledNetherBricks(b) => b.resource_location(),
            Self::CrackedNetherBricks(b) => b.resource_location(),
            Self::QuartzBricks(b) => b.resource_location(),
            Self::Candle(b) => b.resource_location(),
            Self::WhiteCandle(b) => b.resource_location(),
            Self::OrangeCandle(b) => b.resource_location(),
            Self::MagentaCandle(b) => b.resource_location(),
            Self::LightBlueCandle(b) => b.resource_location(),
            Self::YellowCandle(b) => b.resource_location(),
            Self::LimeCandle(b) => b.resource_location(),
            Self::PinkCandle(b) => b.resource_location(),
            Self::GrayCandle(b) => b.resource_location(),
            Self::LightGrayCandle(b) => b.resource_location(),
            Self::CyanCandle(b) => b.resource_location(),
            Self::PurpleCandle(b) => b.resource_location(),
            Self::BlueCandle(b) => b.resource_location(),
            Self::BrownCandle(b) => b.resource_location(),
            Self::GreenCandle(b) => b.resource_location(),
            Self::RedCandle(b) => b.resource_location(),
            Self::BlackCandle(b) => b.resource_location(),
            Self::CandleCake(b) => b.resource_location(),
            Self::WhiteCandleCake(b) => b.resource_location(),
            Self::OrangeCandleCake(b) => b.resource_location(),
            Self::MagentaCandleCake(b) => b.resource_location(),
            Self::LightBlueCandleCake(b) => b.resource_location(),
            Self::YellowCandleCake(b) => b.resource_location(),
            Self::LimeCandleCake(b) => b.resource_location(),
            Self::PinkCandleCake(b) => b.resource_location(),
            Self::GrayCandleCake(b) => b.resource_location(),
            Self::LightGrayCandleCake(b) => b.resource_location(),
            Self::CyanCandleCake(b) => b.resource_location(),
            Self::PurpleCandleCake(b) => b.resource_location(),
            Self::BlueCandleCake(b) => b.resource_location(),
            Self::BrownCandleCake(b) => b.resource_location(),
            Self::GreenCandleCake(b) => b.resource_location(),
            Self::RedCandleCake(b) => b.resource_location(),
            Self::BlackCandleCake(b) => b.resource_location(),
            Self::AmethystBlock(b) => b.resource_location(),
            Self::BuddingAmethyst(b) => b.resource_location(),
            Self::AmethystCluster(b) => b.resource_location(),
            Self::LargeAmethystBud(b) => b.resource_location(),
            Self::MediumAmethystBud(b) => b.resource_location(),
            Self::SmallAmethystBud(b) => b.resource_location(),
            Self::Tuff(b) => b.resource_location(),
            Self::Calcite(b) => b.resource_location(),
            Self::TintedGlass(b) => b.resource_location(),
            Self::PowderSnow(b) => b.resource_location(),
            Self::SculkSensor(b) => b.resource_location(),
            Self::CalibratedSculkSensor(b) => b.resource_location(),
            Self::Sculk(b) => b.resource_location(),
            Self::SculkVein(b) => b.resource_location(),
            Self::SculkCatalyst(b) => b.resource_location(),
            Self::SculkShrieker(b) => b.resource_location(),
            Self::OxidizedCopper(b) => b.resource_location(),
            Self::WeatheredCopper(b) => b.resource_location(),
            Self::ExposedCopper(b) => b.resource_location(),
            Self::CopperBlock(b) => b.resource_location(),
            Self::CopperOre(b) => b.resource_location(),
            Self::DeepslateCopperOre(b) => b.resource_location(),
            Self::OxidizedCutCopper(b) => b.resource_location(),
            Self::WeatheredCutCopper(b) => b.resource_location(),
            Self::ExposedCutCopper(b) => b.resource_location(),
            Self::CutCopper(b) => b.resource_location(),
            Self::OxidizedCutCopperStairs(b) => b.resource_location(),
            Self::WeatheredCutCopperStairs(b) => b.resource_location(),
            Self::ExposedCutCopperStairs(b) => b.resource_location(),
            Self::CutCopperStairs(b) => b.resource_location(),
            Self::OxidizedCutCopperSlab(b) => b.resource_location(),
            Self::WeatheredCutCopperSlab(b) => b.resource_location(),
            Self::ExposedCutCopperSlab(b) => b.resource_location(),
            Self::CutCopperSlab(b) => b.resource_location(),
            Self::WaxedCopperBlock(b) => b.resource_location(),
            Self::WaxedWeatheredCopper(b) => b.resource_location(),
            Self::WaxedExposedCopper(b) => b.resource_location(),
            Self::WaxedOxidizedCopper(b) => b.resource_location(),
            Self::WaxedOxidizedCutCopper(b) => b.resource_location(),
            Self::WaxedWeatheredCutCopper(b) => b.resource_location(),
            Self::WaxedExposedCutCopper(b) => b.resource_location(),
            Self::WaxedCutCopper(b) => b.resource_location(),
            Self::WaxedOxidizedCutCopperStairs(b) => b.resource_location(),
            Self::WaxedWeatheredCutCopperStairs(b) => b.resource_location(),
            Self::WaxedExposedCutCopperStairs(b) => b.resource_location(),
            Self::WaxedCutCopperStairs(b) => b.resource_location(),
            Self::WaxedOxidizedCutCopperSlab(b) => b.resource_location(),
            Self::WaxedWeatheredCutCopperSlab(b) => b.resource_location(),
            Self::WaxedExposedCutCopperSlab(b) => b.resource_location(),
            Self::WaxedCutCopperSlab(b) => b.resource_location(),
            Self::LightningRod(b) => b.resource_location(),
            Self::PointedDripstone(b) => b.resource_location(),
            Self::DripstoneBlock(b) => b.resource_location(),
            Self::CaveVines(b) => b.resource_location(),
            Self::CaveVinesPlant(b) => b.resource_location(),
            Self::SporeBlossom(b) => b.resource_location(),
            Self::Azalea(b) => b.resource_location(),
            Self::FloweringAzalea(b) => b.resource_location(),
            Self::MossCarpet(b) => b.resource_location(),
            Self::PinkPetals(b) => b.resource_location(),
            Self::MossBlock(b) => b.resource_location(),
            Self::BigDripleaf(b) => b.resource_location(),
            Self::BigDripleafStem(b) => b.resource_location(),
            Self::SmallDripleaf(b) => b.resource_location(),
            Self::HangingRoots(b) => b.resource_location(),
            Self::RootedDirt(b) => b.resource_location(),
            Self::Mud(b) => b.resource_location(),
            Self::Deepslate(b) => b.resource_location(),
            Self::CobbledDeepslate(b) => b.resource_location(),
            Self::CobbledDeepslateStairs(b) => b.resource_location(),
            Self::CobbledDeepslateSlab(b) => b.resource_location(),
            Self::CobbledDeepslateWall(b) => b.resource_location(),
            Self::PolishedDeepslate(b) => b.resource_location(),
            Self::PolishedDeepslateStairs(b) => b.resource_location(),
            Self::PolishedDeepslateSlab(b) => b.resource_location(),
            Self::PolishedDeepslateWall(b) => b.resource_location(),
            Self::DeepslateTiles(b) => b.resource_location(),
            Self::DeepslateTileStairs(b) => b.resource_location(),
            Self::DeepslateTileSlab(b) => b.resource_location(),
            Self::DeepslateTileWall(b) => b.resource_location(),
            Self::DeepslateBricks(b) => b.resource_location(),
            Self::DeepslateBrickStairs(b) => b.resource_location(),
            Self::DeepslateBrickSlab(b) => b.resource_location(),
            Self::DeepslateBrickWall(b) => b.resource_location(),
            Self::ChiseledDeepslate(b) => b.resource_location(),
            Self::CrackedDeepslateBricks(b) => b.resource_location(),
            Self::CrackedDeepslateTiles(b) => b.resource_location(),
            Self::InfestedDeepslate(b) => b.resource_location(),
            Self::SmoothBasalt(b) => b.resource_location(),
            Self::RawIronBlock(b) => b.resource_location(),
            Self::RawCopperBlock(b) => b.resource_location(),
            Self::RawGoldBlock(b) => b.resource_location(),
            Self::PottedAzaleaBush(b) => b.resource_location(),
            Self::PottedFloweringAzaleaBush(b) => b.resource_location(),
            Self::OchreFroglight(b) => b.resource_location(),
            Self::VerdantFroglight(b) => b.resource_location(),
            Self::PearlescentFroglight(b) => b.resource_location(),
            Self::Frogspawn(b) => b.resource_location(),
            Self::ReinforcedDeepslate(b) => b.resource_location(),
            Self::DecoratedPot(b) => b.resource_location(),
        }
    }
    fn to_u32(&self) -> u32 {
        match self {
            Self::Error(b) => b.to_u32(),
            Self::Air(b) => b.to_u32(),
            Self::Stone(b) => b.to_u32(),
            Self::Granite(b) => b.to_u32(),
            Self::PolishedGranite(b) => b.to_u32(),
            Self::Diorite(b) => b.to_u32(),
            Self::PolishedDiorite(b) => b.to_u32(),
            Self::Andesite(b) => b.to_u32(),
            Self::PolishedAndesite(b) => b.to_u32(),
            Self::GrassBlock(b) => b.to_u32(),
            Self::Dirt(b) => b.to_u32(),
            Self::CoarseDirt(b) => b.to_u32(),
            Self::Podzol(b) => b.to_u32(),
            Self::Cobblestone(b) => b.to_u32(),
            Self::OakPlanks(b) => b.to_u32(),
            Self::SprucePlanks(b) => b.to_u32(),
            Self::BirchPlanks(b) => b.to_u32(),
            Self::JunglePlanks(b) => b.to_u32(),
            Self::AcaciaPlanks(b) => b.to_u32(),
            Self::CherryPlanks(b) => b.to_u32(),
            Self::DarkOakPlanks(b) => b.to_u32(),
            Self::MangrovePlanks(b) => b.to_u32(),
            Self::BambooPlanks(b) => b.to_u32(),
            Self::BambooMosaic(b) => b.to_u32(),
            Self::OakSapling(b) => b.to_u32(),
            Self::SpruceSapling(b) => b.to_u32(),
            Self::BirchSapling(b) => b.to_u32(),
            Self::JungleSapling(b) => b.to_u32(),
            Self::AcaciaSapling(b) => b.to_u32(),
            Self::CherrySapling(b) => b.to_u32(),
            Self::DarkOakSapling(b) => b.to_u32(),
            Self::MangrovePropagule(b) => b.to_u32(),
            Self::Bedrock(b) => b.to_u32(),
            Self::Water(b) => b.to_u32(),
            Self::Lava(b) => b.to_u32(),
            Self::Sand(b) => b.to_u32(),
            Self::SuspiciousSand(b) => b.to_u32(),
            Self::RedSand(b) => b.to_u32(),
            Self::Gravel(b) => b.to_u32(),
            Self::SuspiciousGravel(b) => b.to_u32(),
            Self::GoldOre(b) => b.to_u32(),
            Self::DeepslateGoldOre(b) => b.to_u32(),
            Self::IronOre(b) => b.to_u32(),
            Self::DeepslateIronOre(b) => b.to_u32(),
            Self::CoalOre(b) => b.to_u32(),
            Self::DeepslateCoalOre(b) => b.to_u32(),
            Self::NetherGoldOre(b) => b.to_u32(),
            Self::OakLog(b) => b.to_u32(),
            Self::SpruceLog(b) => b.to_u32(),
            Self::BirchLog(b) => b.to_u32(),
            Self::JungleLog(b) => b.to_u32(),
            Self::AcaciaLog(b) => b.to_u32(),
            Self::CherryLog(b) => b.to_u32(),
            Self::DarkOakLog(b) => b.to_u32(),
            Self::MangroveLog(b) => b.to_u32(),
            Self::MangroveRoots(b) => b.to_u32(),
            Self::MuddyMangroveRoots(b) => b.to_u32(),
            Self::BambooBlock(b) => b.to_u32(),
            Self::StrippedSpruceLog(b) => b.to_u32(),
            Self::StrippedBirchLog(b) => b.to_u32(),
            Self::StrippedJungleLog(b) => b.to_u32(),
            Self::StrippedAcaciaLog(b) => b.to_u32(),
            Self::StrippedCherryLog(b) => b.to_u32(),
            Self::StrippedDarkOakLog(b) => b.to_u32(),
            Self::StrippedOakLog(b) => b.to_u32(),
            Self::StrippedMangroveLog(b) => b.to_u32(),
            Self::StrippedBambooBlock(b) => b.to_u32(),
            Self::OakWood(b) => b.to_u32(),
            Self::SpruceWood(b) => b.to_u32(),
            Self::BirchWood(b) => b.to_u32(),
            Self::JungleWood(b) => b.to_u32(),
            Self::AcaciaWood(b) => b.to_u32(),
            Self::CherryWood(b) => b.to_u32(),
            Self::DarkOakWood(b) => b.to_u32(),
            Self::MangroveWood(b) => b.to_u32(),
            Self::StrippedOakWood(b) => b.to_u32(),
            Self::StrippedSpruceWood(b) => b.to_u32(),
            Self::StrippedBirchWood(b) => b.to_u32(),
            Self::StrippedJungleWood(b) => b.to_u32(),
            Self::StrippedAcaciaWood(b) => b.to_u32(),
            Self::StrippedCherryWood(b) => b.to_u32(),
            Self::StrippedDarkOakWood(b) => b.to_u32(),
            Self::StrippedMangroveWood(b) => b.to_u32(),
            Self::OakLeaves(b) => b.to_u32(),
            Self::SpruceLeaves(b) => b.to_u32(),
            Self::BirchLeaves(b) => b.to_u32(),
            Self::JungleLeaves(b) => b.to_u32(),
            Self::AcaciaLeaves(b) => b.to_u32(),
            Self::CherryLeaves(b) => b.to_u32(),
            Self::DarkOakLeaves(b) => b.to_u32(),
            Self::MangroveLeaves(b) => b.to_u32(),
            Self::AzaleaLeaves(b) => b.to_u32(),
            Self::FloweringAzaleaLeaves(b) => b.to_u32(),
            Self::Sponge(b) => b.to_u32(),
            Self::WetSponge(b) => b.to_u32(),
            Self::Glass(b) => b.to_u32(),
            Self::LapisOre(b) => b.to_u32(),
            Self::DeepslateLapisOre(b) => b.to_u32(),
            Self::LapisBlock(b) => b.to_u32(),
            Self::Dispenser(b) => b.to_u32(),
            Self::Sandstone(b) => b.to_u32(),
            Self::ChiseledSandstone(b) => b.to_u32(),
            Self::CutSandstone(b) => b.to_u32(),
            Self::NoteBlock(b) => b.to_u32(),
            Self::WhiteBed(b) => b.to_u32(),
            Self::OrangeBed(b) => b.to_u32(),
            Self::MagentaBed(b) => b.to_u32(),
            Self::LightBlueBed(b) => b.to_u32(),
            Self::YellowBed(b) => b.to_u32(),
            Self::LimeBed(b) => b.to_u32(),
            Self::PinkBed(b) => b.to_u32(),
            Self::GrayBed(b) => b.to_u32(),
            Self::LightGrayBed(b) => b.to_u32(),
            Self::CyanBed(b) => b.to_u32(),
            Self::PurpleBed(b) => b.to_u32(),
            Self::BlueBed(b) => b.to_u32(),
            Self::BrownBed(b) => b.to_u32(),
            Self::GreenBed(b) => b.to_u32(),
            Self::RedBed(b) => b.to_u32(),
            Self::BlackBed(b) => b.to_u32(),
            Self::PoweredRail(b) => b.to_u32(),
            Self::DetectorRail(b) => b.to_u32(),
            Self::StickyPiston(b) => b.to_u32(),
            Self::Cobweb(b) => b.to_u32(),
            Self::Grass(b) => b.to_u32(),
            Self::Fern(b) => b.to_u32(),
            Self::DeadBush(b) => b.to_u32(),
            Self::Seagrass(b) => b.to_u32(),
            Self::TallSeagrass(b) => b.to_u32(),
            Self::Piston(b) => b.to_u32(),
            Self::PistonHead(b) => b.to_u32(),
            Self::WhiteWool(b) => b.to_u32(),
            Self::OrangeWool(b) => b.to_u32(),
            Self::MagentaWool(b) => b.to_u32(),
            Self::LightBlueWool(b) => b.to_u32(),
            Self::YellowWool(b) => b.to_u32(),
            Self::LimeWool(b) => b.to_u32(),
            Self::PinkWool(b) => b.to_u32(),
            Self::GrayWool(b) => b.to_u32(),
            Self::LightGrayWool(b) => b.to_u32(),
            Self::CyanWool(b) => b.to_u32(),
            Self::PurpleWool(b) => b.to_u32(),
            Self::BlueWool(b) => b.to_u32(),
            Self::BrownWool(b) => b.to_u32(),
            Self::GreenWool(b) => b.to_u32(),
            Self::RedWool(b) => b.to_u32(),
            Self::BlackWool(b) => b.to_u32(),
            Self::MovingPiston(b) => b.to_u32(),
            Self::Dandelion(b) => b.to_u32(),
            Self::Torchflower(b) => b.to_u32(),
            Self::Poppy(b) => b.to_u32(),
            Self::BlueOrchid(b) => b.to_u32(),
            Self::Allium(b) => b.to_u32(),
            Self::AzureBluet(b) => b.to_u32(),
            Self::RedTulip(b) => b.to_u32(),
            Self::OrangeTulip(b) => b.to_u32(),
            Self::WhiteTulip(b) => b.to_u32(),
            Self::PinkTulip(b) => b.to_u32(),
            Self::OxeyeDaisy(b) => b.to_u32(),
            Self::Cornflower(b) => b.to_u32(),
            Self::WitherRose(b) => b.to_u32(),
            Self::LilyOfTheValley(b) => b.to_u32(),
            Self::BrownMushroom(b) => b.to_u32(),
            Self::RedMushroom(b) => b.to_u32(),
            Self::GoldBlock(b) => b.to_u32(),
            Self::IronBlock(b) => b.to_u32(),
            Self::Bricks(b) => b.to_u32(),
            Self::Tnt(b) => b.to_u32(),
            Self::Bookshelf(b) => b.to_u32(),
            Self::ChiseledBookshelf(b) => b.to_u32(),
            Self::MossyCobblestone(b) => b.to_u32(),
            Self::Obsidian(b) => b.to_u32(),
            Self::Torch(b) => b.to_u32(),
            Self::WallTorch(b) => b.to_u32(),
            Self::Fire(b) => b.to_u32(),
            Self::SoulFire(b) => b.to_u32(),
            Self::Spawner(b) => b.to_u32(),
            Self::OakStairs(b) => b.to_u32(),
            Self::Chest(b) => b.to_u32(),
            Self::RedstoneWire(b) => b.to_u32(),
            Self::DiamondOre(b) => b.to_u32(),
            Self::DeepslateDiamondOre(b) => b.to_u32(),
            Self::DiamondBlock(b) => b.to_u32(),
            Self::CraftingTable(b) => b.to_u32(),
            Self::Wheat(b) => b.to_u32(),
            Self::Farmland(b) => b.to_u32(),
            Self::Furnace(b) => b.to_u32(),
            Self::OakSign(b) => b.to_u32(),
            Self::SpruceSign(b) => b.to_u32(),
            Self::BirchSign(b) => b.to_u32(),
            Self::AcaciaSign(b) => b.to_u32(),
            Self::CherrySign(b) => b.to_u32(),
            Self::JungleSign(b) => b.to_u32(),
            Self::DarkOakSign(b) => b.to_u32(),
            Self::MangroveSign(b) => b.to_u32(),
            Self::BambooSign(b) => b.to_u32(),
            Self::OakDoor(b) => b.to_u32(),
            Self::Ladder(b) => b.to_u32(),
            Self::Rail(b) => b.to_u32(),
            Self::CobblestoneStairs(b) => b.to_u32(),
            Self::OakWallSign(b) => b.to_u32(),
            Self::SpruceWallSign(b) => b.to_u32(),
            Self::BirchWallSign(b) => b.to_u32(),
            Self::AcaciaWallSign(b) => b.to_u32(),
            Self::CherryWallSign(b) => b.to_u32(),
            Self::JungleWallSign(b) => b.to_u32(),
            Self::DarkOakWallSign(b) => b.to_u32(),
            Self::MangroveWallSign(b) => b.to_u32(),
            Self::BambooWallSign(b) => b.to_u32(),
            Self::OakHangingSign(b) => b.to_u32(),
            Self::SpruceHangingSign(b) => b.to_u32(),
            Self::BirchHangingSign(b) => b.to_u32(),
            Self::AcaciaHangingSign(b) => b.to_u32(),
            Self::CherryHangingSign(b) => b.to_u32(),
            Self::JungleHangingSign(b) => b.to_u32(),
            Self::DarkOakHangingSign(b) => b.to_u32(),
            Self::CrimsonHangingSign(b) => b.to_u32(),
            Self::WarpedHangingSign(b) => b.to_u32(),
            Self::MangroveHangingSign(b) => b.to_u32(),
            Self::BambooHangingSign(b) => b.to_u32(),
            Self::OakWallHangingSign(b) => b.to_u32(),
            Self::SpruceWallHangingSign(b) => b.to_u32(),
            Self::BirchWallHangingSign(b) => b.to_u32(),
            Self::AcaciaWallHangingSign(b) => b.to_u32(),
            Self::CherryWallHangingSign(b) => b.to_u32(),
            Self::JungleWallHangingSign(b) => b.to_u32(),
            Self::DarkOakWallHangingSign(b) => b.to_u32(),
            Self::MangroveWallHangingSign(b) => b.to_u32(),
            Self::CrimsonWallHangingSign(b) => b.to_u32(),
            Self::WarpedWallHangingSign(b) => b.to_u32(),
            Self::BambooWallHangingSign(b) => b.to_u32(),
            Self::Lever(b) => b.to_u32(),
            Self::StonePressurePlate(b) => b.to_u32(),
            Self::IronDoor(b) => b.to_u32(),
            Self::OakPressurePlate(b) => b.to_u32(),
            Self::SprucePressurePlate(b) => b.to_u32(),
            Self::BirchPressurePlate(b) => b.to_u32(),
            Self::JunglePressurePlate(b) => b.to_u32(),
            Self::AcaciaPressurePlate(b) => b.to_u32(),
            Self::CherryPressurePlate(b) => b.to_u32(),
            Self::DarkOakPressurePlate(b) => b.to_u32(),
            Self::MangrovePressurePlate(b) => b.to_u32(),
            Self::BambooPressurePlate(b) => b.to_u32(),
            Self::RedstoneOre(b) => b.to_u32(),
            Self::DeepslateRedstoneOre(b) => b.to_u32(),
            Self::RedstoneTorch(b) => b.to_u32(),
            Self::RedstoneWallTorch(b) => b.to_u32(),
            Self::StoneButton(b) => b.to_u32(),
            Self::Snow(b) => b.to_u32(),
            Self::Ice(b) => b.to_u32(),
            Self::SnowBlock(b) => b.to_u32(),
            Self::Cactus(b) => b.to_u32(),
            Self::Clay(b) => b.to_u32(),
            Self::SugarCane(b) => b.to_u32(),
            Self::Jukebox(b) => b.to_u32(),
            Self::OakFence(b) => b.to_u32(),
            Self::Pumpkin(b) => b.to_u32(),
            Self::Netherrack(b) => b.to_u32(),
            Self::SoulSand(b) => b.to_u32(),
            Self::SoulSoil(b) => b.to_u32(),
            Self::Basalt(b) => b.to_u32(),
            Self::PolishedBasalt(b) => b.to_u32(),
            Self::SoulTorch(b) => b.to_u32(),
            Self::SoulWallTorch(b) => b.to_u32(),
            Self::Glowstone(b) => b.to_u32(),
            Self::NetherPortal(b) => b.to_u32(),
            Self::CarvedPumpkin(b) => b.to_u32(),
            Self::JackOLantern(b) => b.to_u32(),
            Self::Cake(b) => b.to_u32(),
            Self::Repeater(b) => b.to_u32(),
            Self::WhiteStainedGlass(b) => b.to_u32(),
            Self::OrangeStainedGlass(b) => b.to_u32(),
            Self::MagentaStainedGlass(b) => b.to_u32(),
            Self::LightBlueStainedGlass(b) => b.to_u32(),
            Self::YellowStainedGlass(b) => b.to_u32(),
            Self::LimeStainedGlass(b) => b.to_u32(),
            Self::PinkStainedGlass(b) => b.to_u32(),
            Self::GrayStainedGlass(b) => b.to_u32(),
            Self::LightGrayStainedGlass(b) => b.to_u32(),
            Self::CyanStainedGlass(b) => b.to_u32(),
            Self::PurpleStainedGlass(b) => b.to_u32(),
            Self::BlueStainedGlass(b) => b.to_u32(),
            Self::BrownStainedGlass(b) => b.to_u32(),
            Self::GreenStainedGlass(b) => b.to_u32(),
            Self::RedStainedGlass(b) => b.to_u32(),
            Self::BlackStainedGlass(b) => b.to_u32(),
            Self::OakTrapdoor(b) => b.to_u32(),
            Self::SpruceTrapdoor(b) => b.to_u32(),
            Self::BirchTrapdoor(b) => b.to_u32(),
            Self::JungleTrapdoor(b) => b.to_u32(),
            Self::AcaciaTrapdoor(b) => b.to_u32(),
            Self::CherryTrapdoor(b) => b.to_u32(),
            Self::DarkOakTrapdoor(b) => b.to_u32(),
            Self::MangroveTrapdoor(b) => b.to_u32(),
            Self::BambooTrapdoor(b) => b.to_u32(),
            Self::StoneBricks(b) => b.to_u32(),
            Self::MossyStoneBricks(b) => b.to_u32(),
            Self::CrackedStoneBricks(b) => b.to_u32(),
            Self::ChiseledStoneBricks(b) => b.to_u32(),
            Self::PackedMud(b) => b.to_u32(),
            Self::MudBricks(b) => b.to_u32(),
            Self::InfestedStone(b) => b.to_u32(),
            Self::InfestedCobblestone(b) => b.to_u32(),
            Self::InfestedStoneBricks(b) => b.to_u32(),
            Self::InfestedMossyStoneBricks(b) => b.to_u32(),
            Self::InfestedCrackedStoneBricks(b) => b.to_u32(),
            Self::InfestedChiseledStoneBricks(b) => b.to_u32(),
            Self::BrownMushroomBlock(b) => b.to_u32(),
            Self::RedMushroomBlock(b) => b.to_u32(),
            Self::MushroomStem(b) => b.to_u32(),
            Self::IronBars(b) => b.to_u32(),
            Self::Chain(b) => b.to_u32(),
            Self::GlassPane(b) => b.to_u32(),
            Self::Melon(b) => b.to_u32(),
            Self::AttachedPumpkinStem(b) => b.to_u32(),
            Self::AttachedMelonStem(b) => b.to_u32(),
            Self::PumpkinStem(b) => b.to_u32(),
            Self::MelonStem(b) => b.to_u32(),
            Self::Vine(b) => b.to_u32(),
            Self::GlowLichen(b) => b.to_u32(),
            Self::OakFenceGate(b) => b.to_u32(),
            Self::BrickStairs(b) => b.to_u32(),
            Self::StoneBrickStairs(b) => b.to_u32(),
            Self::MudBrickStairs(b) => b.to_u32(),
            Self::Mycelium(b) => b.to_u32(),
            Self::LilyPad(b) => b.to_u32(),
            Self::NetherBricks(b) => b.to_u32(),
            Self::NetherBrickFence(b) => b.to_u32(),
            Self::NetherBrickStairs(b) => b.to_u32(),
            Self::NetherWart(b) => b.to_u32(),
            Self::EnchantingTable(b) => b.to_u32(),
            Self::BrewingStand(b) => b.to_u32(),
            Self::Cauldron(b) => b.to_u32(),
            Self::WaterCauldron(b) => b.to_u32(),
            Self::LavaCauldron(b) => b.to_u32(),
            Self::PowderSnowCauldron(b) => b.to_u32(),
            Self::EndPortal(b) => b.to_u32(),
            Self::EndPortalFrame(b) => b.to_u32(),
            Self::EndStone(b) => b.to_u32(),
            Self::DragonEgg(b) => b.to_u32(),
            Self::RedstoneLamp(b) => b.to_u32(),
            Self::Cocoa(b) => b.to_u32(),
            Self::SandstoneStairs(b) => b.to_u32(),
            Self::EmeraldOre(b) => b.to_u32(),
            Self::DeepslateEmeraldOre(b) => b.to_u32(),
            Self::EnderChest(b) => b.to_u32(),
            Self::TripwireHook(b) => b.to_u32(),
            Self::Tripwire(b) => b.to_u32(),
            Self::EmeraldBlock(b) => b.to_u32(),
            Self::SpruceStairs(b) => b.to_u32(),
            Self::BirchStairs(b) => b.to_u32(),
            Self::JungleStairs(b) => b.to_u32(),
            Self::CommandBlock(b) => b.to_u32(),
            Self::Beacon(b) => b.to_u32(),
            Self::CobblestoneWall(b) => b.to_u32(),
            Self::MossyCobblestoneWall(b) => b.to_u32(),
            Self::FlowerPot(b) => b.to_u32(),
            Self::PottedTorchflower(b) => b.to_u32(),
            Self::PottedOakSapling(b) => b.to_u32(),
            Self::PottedSpruceSapling(b) => b.to_u32(),
            Self::PottedBirchSapling(b) => b.to_u32(),
            Self::PottedJungleSapling(b) => b.to_u32(),
            Self::PottedAcaciaSapling(b) => b.to_u32(),
            Self::PottedCherrySapling(b) => b.to_u32(),
            Self::PottedDarkOakSapling(b) => b.to_u32(),
            Self::PottedMangrovePropagule(b) => b.to_u32(),
            Self::PottedFern(b) => b.to_u32(),
            Self::PottedDandelion(b) => b.to_u32(),
            Self::PottedPoppy(b) => b.to_u32(),
            Self::PottedBlueOrchid(b) => b.to_u32(),
            Self::PottedAllium(b) => b.to_u32(),
            Self::PottedAzureBluet(b) => b.to_u32(),
            Self::PottedRedTulip(b) => b.to_u32(),
            Self::PottedOrangeTulip(b) => b.to_u32(),
            Self::PottedWhiteTulip(b) => b.to_u32(),
            Self::PottedPinkTulip(b) => b.to_u32(),
            Self::PottedOxeyeDaisy(b) => b.to_u32(),
            Self::PottedCornflower(b) => b.to_u32(),
            Self::PottedLilyOfTheValley(b) => b.to_u32(),
            Self::PottedWitherRose(b) => b.to_u32(),
            Self::PottedRedMushroom(b) => b.to_u32(),
            Self::PottedBrownMushroom(b) => b.to_u32(),
            Self::PottedDeadBush(b) => b.to_u32(),
            Self::PottedCactus(b) => b.to_u32(),
            Self::Carrots(b) => b.to_u32(),
            Self::Potatoes(b) => b.to_u32(),
            Self::OakButton(b) => b.to_u32(),
            Self::SpruceButton(b) => b.to_u32(),
            Self::BirchButton(b) => b.to_u32(),
            Self::JungleButton(b) => b.to_u32(),
            Self::AcaciaButton(b) => b.to_u32(),
            Self::CherryButton(b) => b.to_u32(),
            Self::DarkOakButton(b) => b.to_u32(),
            Self::MangroveButton(b) => b.to_u32(),
            Self::BambooButton(b) => b.to_u32(),
            Self::SkeletonSkull(b) => b.to_u32(),
            Self::SkeletonWallSkull(b) => b.to_u32(),
            Self::WitherSkeletonSkull(b) => b.to_u32(),
            Self::WitherSkeletonWallSkull(b) => b.to_u32(),
            Self::ZombieHead(b) => b.to_u32(),
            Self::ZombieWallHead(b) => b.to_u32(),
            Self::PlayerHead(b) => b.to_u32(),
            Self::PlayerWallHead(b) => b.to_u32(),
            Self::CreeperHead(b) => b.to_u32(),
            Self::CreeperWallHead(b) => b.to_u32(),
            Self::DragonHead(b) => b.to_u32(),
            Self::DragonWallHead(b) => b.to_u32(),
            Self::PiglinHead(b) => b.to_u32(),
            Self::PiglinWallHead(b) => b.to_u32(),
            Self::Anvil(b) => b.to_u32(),
            Self::ChippedAnvil(b) => b.to_u32(),
            Self::DamagedAnvil(b) => b.to_u32(),
            Self::TrappedChest(b) => b.to_u32(),
            Self::LightWeightedPressurePlate(b) => b.to_u32(),
            Self::HeavyWeightedPressurePlate(b) => b.to_u32(),
            Self::Comparator(b) => b.to_u32(),
            Self::DaylightDetector(b) => b.to_u32(),
            Self::RedstoneBlock(b) => b.to_u32(),
            Self::NetherQuartzOre(b) => b.to_u32(),
            Self::Hopper(b) => b.to_u32(),
            Self::QuartzBlock(b) => b.to_u32(),
            Self::ChiseledQuartzBlock(b) => b.to_u32(),
            Self::QuartzPillar(b) => b.to_u32(),
            Self::QuartzStairs(b) => b.to_u32(),
            Self::ActivatorRail(b) => b.to_u32(),
            Self::Dropper(b) => b.to_u32(),
            Self::WhiteTerracotta(b) => b.to_u32(),
            Self::OrangeTerracotta(b) => b.to_u32(),
            Self::MagentaTerracotta(b) => b.to_u32(),
            Self::LightBlueTerracotta(b) => b.to_u32(),
            Self::YellowTerracotta(b) => b.to_u32(),
            Self::LimeTerracotta(b) => b.to_u32(),
            Self::PinkTerracotta(b) => b.to_u32(),
            Self::GrayTerracotta(b) => b.to_u32(),
            Self::LightGrayTerracotta(b) => b.to_u32(),
            Self::CyanTerracotta(b) => b.to_u32(),
            Self::PurpleTerracotta(b) => b.to_u32(),
            Self::BlueTerracotta(b) => b.to_u32(),
            Self::BrownTerracotta(b) => b.to_u32(),
            Self::GreenTerracotta(b) => b.to_u32(),
            Self::RedTerracotta(b) => b.to_u32(),
            Self::BlackTerracotta(b) => b.to_u32(),
            Self::WhiteStainedGlassPane(b) => b.to_u32(),
            Self::OrangeStainedGlassPane(b) => b.to_u32(),
            Self::MagentaStainedGlassPane(b) => b.to_u32(),
            Self::LightBlueStainedGlassPane(b) => b.to_u32(),
            Self::YellowStainedGlassPane(b) => b.to_u32(),
            Self::LimeStainedGlassPane(b) => b.to_u32(),
            Self::PinkStainedGlassPane(b) => b.to_u32(),
            Self::GrayStainedGlassPane(b) => b.to_u32(),
            Self::LightGrayStainedGlassPane(b) => b.to_u32(),
            Self::CyanStainedGlassPane(b) => b.to_u32(),
            Self::PurpleStainedGlassPane(b) => b.to_u32(),
            Self::BlueStainedGlassPane(b) => b.to_u32(),
            Self::BrownStainedGlassPane(b) => b.to_u32(),
            Self::GreenStainedGlassPane(b) => b.to_u32(),
            Self::RedStainedGlassPane(b) => b.to_u32(),
            Self::BlackStainedGlassPane(b) => b.to_u32(),
            Self::AcaciaStairs(b) => b.to_u32(),
            Self::CherryStairs(b) => b.to_u32(),
            Self::DarkOakStairs(b) => b.to_u32(),
            Self::MangroveStairs(b) => b.to_u32(),
            Self::BambooStairs(b) => b.to_u32(),
            Self::BambooMosaicStairs(b) => b.to_u32(),
            Self::SlimeBlock(b) => b.to_u32(),
            Self::Barrier(b) => b.to_u32(),
            Self::Light(b) => b.to_u32(),
            Self::IronTrapdoor(b) => b.to_u32(),
            Self::Prismarine(b) => b.to_u32(),
            Self::PrismarineBricks(b) => b.to_u32(),
            Self::DarkPrismarine(b) => b.to_u32(),
            Self::PrismarineStairs(b) => b.to_u32(),
            Self::PrismarineBrickStairs(b) => b.to_u32(),
            Self::DarkPrismarineStairs(b) => b.to_u32(),
            Self::PrismarineSlab(b) => b.to_u32(),
            Self::PrismarineBrickSlab(b) => b.to_u32(),
            Self::DarkPrismarineSlab(b) => b.to_u32(),
            Self::SeaLantern(b) => b.to_u32(),
            Self::HayBlock(b) => b.to_u32(),
            Self::WhiteCarpet(b) => b.to_u32(),
            Self::OrangeCarpet(b) => b.to_u32(),
            Self::MagentaCarpet(b) => b.to_u32(),
            Self::LightBlueCarpet(b) => b.to_u32(),
            Self::YellowCarpet(b) => b.to_u32(),
            Self::LimeCarpet(b) => b.to_u32(),
            Self::PinkCarpet(b) => b.to_u32(),
            Self::GrayCarpet(b) => b.to_u32(),
            Self::LightGrayCarpet(b) => b.to_u32(),
            Self::CyanCarpet(b) => b.to_u32(),
            Self::PurpleCarpet(b) => b.to_u32(),
            Self::BlueCarpet(b) => b.to_u32(),
            Self::BrownCarpet(b) => b.to_u32(),
            Self::GreenCarpet(b) => b.to_u32(),
            Self::RedCarpet(b) => b.to_u32(),
            Self::BlackCarpet(b) => b.to_u32(),
            Self::Terracotta(b) => b.to_u32(),
            Self::CoalBlock(b) => b.to_u32(),
            Self::PackedIce(b) => b.to_u32(),
            Self::Sunflower(b) => b.to_u32(),
            Self::Lilac(b) => b.to_u32(),
            Self::RoseBush(b) => b.to_u32(),
            Self::Peony(b) => b.to_u32(),
            Self::TallGrass(b) => b.to_u32(),
            Self::LargeFern(b) => b.to_u32(),
            Self::WhiteBanner(b) => b.to_u32(),
            Self::OrangeBanner(b) => b.to_u32(),
            Self::MagentaBanner(b) => b.to_u32(),
            Self::LightBlueBanner(b) => b.to_u32(),
            Self::YellowBanner(b) => b.to_u32(),
            Self::LimeBanner(b) => b.to_u32(),
            Self::PinkBanner(b) => b.to_u32(),
            Self::GrayBanner(b) => b.to_u32(),
            Self::LightGrayBanner(b) => b.to_u32(),
            Self::CyanBanner(b) => b.to_u32(),
            Self::PurpleBanner(b) => b.to_u32(),
            Self::BlueBanner(b) => b.to_u32(),
            Self::BrownBanner(b) => b.to_u32(),
            Self::GreenBanner(b) => b.to_u32(),
            Self::RedBanner(b) => b.to_u32(),
            Self::BlackBanner(b) => b.to_u32(),
            Self::WhiteWallBanner(b) => b.to_u32(),
            Self::OrangeWallBanner(b) => b.to_u32(),
            Self::MagentaWallBanner(b) => b.to_u32(),
            Self::LightBlueWallBanner(b) => b.to_u32(),
            Self::YellowWallBanner(b) => b.to_u32(),
            Self::LimeWallBanner(b) => b.to_u32(),
            Self::PinkWallBanner(b) => b.to_u32(),
            Self::GrayWallBanner(b) => b.to_u32(),
            Self::LightGrayWallBanner(b) => b.to_u32(),
            Self::CyanWallBanner(b) => b.to_u32(),
            Self::PurpleWallBanner(b) => b.to_u32(),
            Self::BlueWallBanner(b) => b.to_u32(),
            Self::BrownWallBanner(b) => b.to_u32(),
            Self::GreenWallBanner(b) => b.to_u32(),
            Self::RedWallBanner(b) => b.to_u32(),
            Self::BlackWallBanner(b) => b.to_u32(),
            Self::RedSandstone(b) => b.to_u32(),
            Self::ChiseledRedSandstone(b) => b.to_u32(),
            Self::CutRedSandstone(b) => b.to_u32(),
            Self::RedSandstoneStairs(b) => b.to_u32(),
            Self::OakSlab(b) => b.to_u32(),
            Self::SpruceSlab(b) => b.to_u32(),
            Self::BirchSlab(b) => b.to_u32(),
            Self::JungleSlab(b) => b.to_u32(),
            Self::AcaciaSlab(b) => b.to_u32(),
            Self::CherrySlab(b) => b.to_u32(),
            Self::DarkOakSlab(b) => b.to_u32(),
            Self::MangroveSlab(b) => b.to_u32(),
            Self::BambooSlab(b) => b.to_u32(),
            Self::BambooMosaicSlab(b) => b.to_u32(),
            Self::StoneSlab(b) => b.to_u32(),
            Self::SmoothStoneSlab(b) => b.to_u32(),
            Self::SandstoneSlab(b) => b.to_u32(),
            Self::CutSandstoneSlab(b) => b.to_u32(),
            Self::PetrifiedOakSlab(b) => b.to_u32(),
            Self::CobblestoneSlab(b) => b.to_u32(),
            Self::BrickSlab(b) => b.to_u32(),
            Self::StoneBrickSlab(b) => b.to_u32(),
            Self::MudBrickSlab(b) => b.to_u32(),
            Self::NetherBrickSlab(b) => b.to_u32(),
            Self::QuartzSlab(b) => b.to_u32(),
            Self::RedSandstoneSlab(b) => b.to_u32(),
            Self::CutRedSandstoneSlab(b) => b.to_u32(),
            Self::PurpurSlab(b) => b.to_u32(),
            Self::SmoothStone(b) => b.to_u32(),
            Self::SmoothSandstone(b) => b.to_u32(),
            Self::SmoothQuartz(b) => b.to_u32(),
            Self::SmoothRedSandstone(b) => b.to_u32(),
            Self::SpruceFenceGate(b) => b.to_u32(),
            Self::BirchFenceGate(b) => b.to_u32(),
            Self::JungleFenceGate(b) => b.to_u32(),
            Self::AcaciaFenceGate(b) => b.to_u32(),
            Self::CherryFenceGate(b) => b.to_u32(),
            Self::DarkOakFenceGate(b) => b.to_u32(),
            Self::MangroveFenceGate(b) => b.to_u32(),
            Self::BambooFenceGate(b) => b.to_u32(),
            Self::SpruceFence(b) => b.to_u32(),
            Self::BirchFence(b) => b.to_u32(),
            Self::JungleFence(b) => b.to_u32(),
            Self::AcaciaFence(b) => b.to_u32(),
            Self::CherryFence(b) => b.to_u32(),
            Self::DarkOakFence(b) => b.to_u32(),
            Self::MangroveFence(b) => b.to_u32(),
            Self::BambooFence(b) => b.to_u32(),
            Self::SpruceDoor(b) => b.to_u32(),
            Self::BirchDoor(b) => b.to_u32(),
            Self::JungleDoor(b) => b.to_u32(),
            Self::AcaciaDoor(b) => b.to_u32(),
            Self::CherryDoor(b) => b.to_u32(),
            Self::DarkOakDoor(b) => b.to_u32(),
            Self::MangroveDoor(b) => b.to_u32(),
            Self::BambooDoor(b) => b.to_u32(),
            Self::EndRod(b) => b.to_u32(),
            Self::ChorusPlant(b) => b.to_u32(),
            Self::ChorusFlower(b) => b.to_u32(),
            Self::PurpurBlock(b) => b.to_u32(),
            Self::PurpurPillar(b) => b.to_u32(),
            Self::PurpurStairs(b) => b.to_u32(),
            Self::EndStoneBricks(b) => b.to_u32(),
            Self::TorchflowerCrop(b) => b.to_u32(),
            Self::PitcherCrop(b) => b.to_u32(),
            Self::PitcherPlant(b) => b.to_u32(),
            Self::Beetroots(b) => b.to_u32(),
            Self::DirtPath(b) => b.to_u32(),
            Self::EndGateway(b) => b.to_u32(),
            Self::RepeatingCommandBlock(b) => b.to_u32(),
            Self::ChainCommandBlock(b) => b.to_u32(),
            Self::FrostedIce(b) => b.to_u32(),
            Self::MagmaBlock(b) => b.to_u32(),
            Self::NetherWartBlock(b) => b.to_u32(),
            Self::RedNetherBricks(b) => b.to_u32(),
            Self::BoneBlock(b) => b.to_u32(),
            Self::StructureVoid(b) => b.to_u32(),
            Self::Observer(b) => b.to_u32(),
            Self::ShulkerBox(b) => b.to_u32(),
            Self::WhiteShulkerBox(b) => b.to_u32(),
            Self::OrangeShulkerBox(b) => b.to_u32(),
            Self::MagentaShulkerBox(b) => b.to_u32(),
            Self::LightBlueShulkerBox(b) => b.to_u32(),
            Self::YellowShulkerBox(b) => b.to_u32(),
            Self::LimeShulkerBox(b) => b.to_u32(),
            Self::PinkShulkerBox(b) => b.to_u32(),
            Self::GrayShulkerBox(b) => b.to_u32(),
            Self::LightGrayShulkerBox(b) => b.to_u32(),
            Self::CyanShulkerBox(b) => b.to_u32(),
            Self::PurpleShulkerBox(b) => b.to_u32(),
            Self::BlueShulkerBox(b) => b.to_u32(),
            Self::BrownShulkerBox(b) => b.to_u32(),
            Self::GreenShulkerBox(b) => b.to_u32(),
            Self::RedShulkerBox(b) => b.to_u32(),
            Self::BlackShulkerBox(b) => b.to_u32(),
            Self::WhiteGlazedTerracotta(b) => b.to_u32(),
            Self::OrangeGlazedTerracotta(b) => b.to_u32(),
            Self::MagentaGlazedTerracotta(b) => b.to_u32(),
            Self::LightBlueGlazedTerracotta(b) => b.to_u32(),
            Self::YellowGlazedTerracotta(b) => b.to_u32(),
            Self::LimeGlazedTerracotta(b) => b.to_u32(),
            Self::PinkGlazedTerracotta(b) => b.to_u32(),
            Self::GrayGlazedTerracotta(b) => b.to_u32(),
            Self::LightGrayGlazedTerracotta(b) => b.to_u32(),
            Self::CyanGlazedTerracotta(b) => b.to_u32(),
            Self::PurpleGlazedTerracotta(b) => b.to_u32(),
            Self::BlueGlazedTerracotta(b) => b.to_u32(),
            Self::BrownGlazedTerracotta(b) => b.to_u32(),
            Self::GreenGlazedTerracotta(b) => b.to_u32(),
            Self::RedGlazedTerracotta(b) => b.to_u32(),
            Self::BlackGlazedTerracotta(b) => b.to_u32(),
            Self::WhiteConcrete(b) => b.to_u32(),
            Self::OrangeConcrete(b) => b.to_u32(),
            Self::MagentaConcrete(b) => b.to_u32(),
            Self::LightBlueConcrete(b) => b.to_u32(),
            Self::YellowConcrete(b) => b.to_u32(),
            Self::LimeConcrete(b) => b.to_u32(),
            Self::PinkConcrete(b) => b.to_u32(),
            Self::GrayConcrete(b) => b.to_u32(),
            Self::LightGrayConcrete(b) => b.to_u32(),
            Self::CyanConcrete(b) => b.to_u32(),
            Self::PurpleConcrete(b) => b.to_u32(),
            Self::BlueConcrete(b) => b.to_u32(),
            Self::BrownConcrete(b) => b.to_u32(),
            Self::GreenConcrete(b) => b.to_u32(),
            Self::RedConcrete(b) => b.to_u32(),
            Self::BlackConcrete(b) => b.to_u32(),
            Self::WhiteConcretePowder(b) => b.to_u32(),
            Self::OrangeConcretePowder(b) => b.to_u32(),
            Self::MagentaConcretePowder(b) => b.to_u32(),
            Self::LightBlueConcretePowder(b) => b.to_u32(),
            Self::YellowConcretePowder(b) => b.to_u32(),
            Self::LimeConcretePowder(b) => b.to_u32(),
            Self::PinkConcretePowder(b) => b.to_u32(),
            Self::GrayConcretePowder(b) => b.to_u32(),
            Self::LightGrayConcretePowder(b) => b.to_u32(),
            Self::CyanConcretePowder(b) => b.to_u32(),
            Self::PurpleConcretePowder(b) => b.to_u32(),
            Self::BlueConcretePowder(b) => b.to_u32(),
            Self::BrownConcretePowder(b) => b.to_u32(),
            Self::GreenConcretePowder(b) => b.to_u32(),
            Self::RedConcretePowder(b) => b.to_u32(),
            Self::BlackConcretePowder(b) => b.to_u32(),
            Self::Kelp(b) => b.to_u32(),
            Self::KelpPlant(b) => b.to_u32(),
            Self::DriedKelpBlock(b) => b.to_u32(),
            Self::TurtleEgg(b) => b.to_u32(),
            Self::SnifferEgg(b) => b.to_u32(),
            Self::DeadTubeCoralBlock(b) => b.to_u32(),
            Self::DeadBrainCoralBlock(b) => b.to_u32(),
            Self::DeadBubbleCoralBlock(b) => b.to_u32(),
            Self::DeadFireCoralBlock(b) => b.to_u32(),
            Self::DeadHornCoralBlock(b) => b.to_u32(),
            Self::TubeCoralBlock(b) => b.to_u32(),
            Self::BrainCoralBlock(b) => b.to_u32(),
            Self::BubbleCoralBlock(b) => b.to_u32(),
            Self::FireCoralBlock(b) => b.to_u32(),
            Self::HornCoralBlock(b) => b.to_u32(),
            Self::DeadTubeCoral(b) => b.to_u32(),
            Self::DeadBrainCoral(b) => b.to_u32(),
            Self::DeadBubbleCoral(b) => b.to_u32(),
            Self::DeadFireCoral(b) => b.to_u32(),
            Self::DeadHornCoral(b) => b.to_u32(),
            Self::TubeCoral(b) => b.to_u32(),
            Self::BrainCoral(b) => b.to_u32(),
            Self::BubbleCoral(b) => b.to_u32(),
            Self::FireCoral(b) => b.to_u32(),
            Self::HornCoral(b) => b.to_u32(),
            Self::DeadTubeCoralFan(b) => b.to_u32(),
            Self::DeadBrainCoralFan(b) => b.to_u32(),
            Self::DeadBubbleCoralFan(b) => b.to_u32(),
            Self::DeadFireCoralFan(b) => b.to_u32(),
            Self::DeadHornCoralFan(b) => b.to_u32(),
            Self::TubeCoralFan(b) => b.to_u32(),
            Self::BrainCoralFan(b) => b.to_u32(),
            Self::BubbleCoralFan(b) => b.to_u32(),
            Self::FireCoralFan(b) => b.to_u32(),
            Self::HornCoralFan(b) => b.to_u32(),
            Self::DeadTubeCoralWallFan(b) => b.to_u32(),
            Self::DeadBrainCoralWallFan(b) => b.to_u32(),
            Self::DeadBubbleCoralWallFan(b) => b.to_u32(),
            Self::DeadFireCoralWallFan(b) => b.to_u32(),
            Self::DeadHornCoralWallFan(b) => b.to_u32(),
            Self::TubeCoralWallFan(b) => b.to_u32(),
            Self::BrainCoralWallFan(b) => b.to_u32(),
            Self::BubbleCoralWallFan(b) => b.to_u32(),
            Self::FireCoralWallFan(b) => b.to_u32(),
            Self::HornCoralWallFan(b) => b.to_u32(),
            Self::SeaPickle(b) => b.to_u32(),
            Self::BlueIce(b) => b.to_u32(),
            Self::Conduit(b) => b.to_u32(),
            Self::BambooSapling(b) => b.to_u32(),
            Self::Bamboo(b) => b.to_u32(),
            Self::PottedBamboo(b) => b.to_u32(),
            Self::VoidAir(b) => b.to_u32(),
            Self::CaveAir(b) => b.to_u32(),
            Self::BubbleColumn(b) => b.to_u32(),
            Self::PolishedGraniteStairs(b) => b.to_u32(),
            Self::SmoothRedSandstoneStairs(b) => b.to_u32(),
            Self::MossyStoneBrickStairs(b) => b.to_u32(),
            Self::PolishedDioriteStairs(b) => b.to_u32(),
            Self::MossyCobblestoneStairs(b) => b.to_u32(),
            Self::EndStoneBrickStairs(b) => b.to_u32(),
            Self::StoneStairs(b) => b.to_u32(),
            Self::SmoothSandstoneStairs(b) => b.to_u32(),
            Self::SmoothQuartzStairs(b) => b.to_u32(),
            Self::GraniteStairs(b) => b.to_u32(),
            Self::AndesiteStairs(b) => b.to_u32(),
            Self::RedNetherBrickStairs(b) => b.to_u32(),
            Self::PolishedAndesiteStairs(b) => b.to_u32(),
            Self::DioriteStairs(b) => b.to_u32(),
            Self::PolishedGraniteSlab(b) => b.to_u32(),
            Self::SmoothRedSandstoneSlab(b) => b.to_u32(),
            Self::MossyStoneBrickSlab(b) => b.to_u32(),
            Self::PolishedDioriteSlab(b) => b.to_u32(),
            Self::MossyCobblestoneSlab(b) => b.to_u32(),
            Self::EndStoneBrickSlab(b) => b.to_u32(),
            Self::SmoothSandstoneSlab(b) => b.to_u32(),
            Self::SmoothQuartzSlab(b) => b.to_u32(),
            Self::GraniteSlab(b) => b.to_u32(),
            Self::AndesiteSlab(b) => b.to_u32(),
            Self::RedNetherBrickSlab(b) => b.to_u32(),
            Self::PolishedAndesiteSlab(b) => b.to_u32(),
            Self::DioriteSlab(b) => b.to_u32(),
            Self::BrickWall(b) => b.to_u32(),
            Self::PrismarineWall(b) => b.to_u32(),
            Self::RedSandstoneWall(b) => b.to_u32(),
            Self::MossyStoneBrickWall(b) => b.to_u32(),
            Self::GraniteWall(b) => b.to_u32(),
            Self::StoneBrickWall(b) => b.to_u32(),
            Self::MudBrickWall(b) => b.to_u32(),
            Self::NetherBrickWall(b) => b.to_u32(),
            Self::AndesiteWall(b) => b.to_u32(),
            Self::RedNetherBrickWall(b) => b.to_u32(),
            Self::SandstoneWall(b) => b.to_u32(),
            Self::EndStoneBrickWall(b) => b.to_u32(),
            Self::DioriteWall(b) => b.to_u32(),
            Self::Scaffolding(b) => b.to_u32(),
            Self::Loom(b) => b.to_u32(),
            Self::Barrel(b) => b.to_u32(),
            Self::Smoker(b) => b.to_u32(),
            Self::BlastFurnace(b) => b.to_u32(),
            Self::CartographyTable(b) => b.to_u32(),
            Self::FletchingTable(b) => b.to_u32(),
            Self::Grindstone(b) => b.to_u32(),
            Self::Lectern(b) => b.to_u32(),
            Self::SmithingTable(b) => b.to_u32(),
            Self::Stonecutter(b) => b.to_u32(),
            Self::Bell(b) => b.to_u32(),
            Self::Lantern(b) => b.to_u32(),
            Self::SoulLantern(b) => b.to_u32(),
            Self::Campfire(b) => b.to_u32(),
            Self::SoulCampfire(b) => b.to_u32(),
            Self::SweetBerryBush(b) => b.to_u32(),
            Self::WarpedStem(b) => b.to_u32(),
            Self::StrippedWarpedStem(b) => b.to_u32(),
            Self::WarpedHyphae(b) => b.to_u32(),
            Self::StrippedWarpedHyphae(b) => b.to_u32(),
            Self::WarpedNylium(b) => b.to_u32(),
            Self::WarpedFungus(b) => b.to_u32(),
            Self::WarpedWartBlock(b) => b.to_u32(),
            Self::WarpedRoots(b) => b.to_u32(),
            Self::NetherSprouts(b) => b.to_u32(),
            Self::CrimsonStem(b) => b.to_u32(),
            Self::StrippedCrimsonStem(b) => b.to_u32(),
            Self::CrimsonHyphae(b) => b.to_u32(),
            Self::StrippedCrimsonHyphae(b) => b.to_u32(),
            Self::CrimsonNylium(b) => b.to_u32(),
            Self::CrimsonFungus(b) => b.to_u32(),
            Self::Shroomlight(b) => b.to_u32(),
            Self::WeepingVines(b) => b.to_u32(),
            Self::WeepingVinesPlant(b) => b.to_u32(),
            Self::TwistingVines(b) => b.to_u32(),
            Self::TwistingVinesPlant(b) => b.to_u32(),
            Self::CrimsonRoots(b) => b.to_u32(),
            Self::CrimsonPlanks(b) => b.to_u32(),
            Self::WarpedPlanks(b) => b.to_u32(),
            Self::CrimsonSlab(b) => b.to_u32(),
            Self::WarpedSlab(b) => b.to_u32(),
            Self::CrimsonPressurePlate(b) => b.to_u32(),
            Self::WarpedPressurePlate(b) => b.to_u32(),
            Self::CrimsonFence(b) => b.to_u32(),
            Self::WarpedFence(b) => b.to_u32(),
            Self::CrimsonTrapdoor(b) => b.to_u32(),
            Self::WarpedTrapdoor(b) => b.to_u32(),
            Self::CrimsonFenceGate(b) => b.to_u32(),
            Self::WarpedFenceGate(b) => b.to_u32(),
            Self::CrimsonStairs(b) => b.to_u32(),
            Self::WarpedStairs(b) => b.to_u32(),
            Self::CrimsonButton(b) => b.to_u32(),
            Self::WarpedButton(b) => b.to_u32(),
            Self::CrimsonDoor(b) => b.to_u32(),
            Self::WarpedDoor(b) => b.to_u32(),
            Self::CrimsonSign(b) => b.to_u32(),
            Self::WarpedSign(b) => b.to_u32(),
            Self::CrimsonWallSign(b) => b.to_u32(),
            Self::WarpedWallSign(b) => b.to_u32(),
            Self::StructureBlock(b) => b.to_u32(),
            Self::Jigsaw(b) => b.to_u32(),
            Self::Composter(b) => b.to_u32(),
            Self::Target(b) => b.to_u32(),
            Self::BeeNest(b) => b.to_u32(),
            Self::Beehive(b) => b.to_u32(),
            Self::HoneyBlock(b) => b.to_u32(),
            Self::HoneycombBlock(b) => b.to_u32(),
            Self::NetheriteBlock(b) => b.to_u32(),
            Self::AncientDebris(b) => b.to_u32(),
            Self::CryingObsidian(b) => b.to_u32(),
            Self::RespawnAnchor(b) => b.to_u32(),
            Self::PottedCrimsonFungus(b) => b.to_u32(),
            Self::PottedWarpedFungus(b) => b.to_u32(),
            Self::PottedCrimsonRoots(b) => b.to_u32(),
            Self::PottedWarpedRoots(b) => b.to_u32(),
            Self::Lodestone(b) => b.to_u32(),
            Self::Blackstone(b) => b.to_u32(),
            Self::BlackstoneStairs(b) => b.to_u32(),
            Self::BlackstoneWall(b) => b.to_u32(),
            Self::BlackstoneSlab(b) => b.to_u32(),
            Self::PolishedBlackstone(b) => b.to_u32(),
            Self::PolishedBlackstoneBricks(b) => b.to_u32(),
            Self::CrackedPolishedBlackstoneBricks(b) => b.to_u32(),
            Self::ChiseledPolishedBlackstone(b) => b.to_u32(),
            Self::PolishedBlackstoneBrickSlab(b) => b.to_u32(),
            Self::PolishedBlackstoneBrickStairs(b) => b.to_u32(),
            Self::PolishedBlackstoneBrickWall(b) => b.to_u32(),
            Self::GildedBlackstone(b) => b.to_u32(),
            Self::PolishedBlackstoneStairs(b) => b.to_u32(),
            Self::PolishedBlackstoneSlab(b) => b.to_u32(),
            Self::PolishedBlackstonePressurePlate(b) => b.to_u32(),
            Self::PolishedBlackstoneButton(b) => b.to_u32(),
            Self::PolishedBlackstoneWall(b) => b.to_u32(),
            Self::ChiseledNetherBricks(b) => b.to_u32(),
            Self::CrackedNetherBricks(b) => b.to_u32(),
            Self::QuartzBricks(b) => b.to_u32(),
            Self::Candle(b) => b.to_u32(),
            Self::WhiteCandle(b) => b.to_u32(),
            Self::OrangeCandle(b) => b.to_u32(),
            Self::MagentaCandle(b) => b.to_u32(),
            Self::LightBlueCandle(b) => b.to_u32(),
            Self::YellowCandle(b) => b.to_u32(),
            Self::LimeCandle(b) => b.to_u32(),
            Self::PinkCandle(b) => b.to_u32(),
            Self::GrayCandle(b) => b.to_u32(),
            Self::LightGrayCandle(b) => b.to_u32(),
            Self::CyanCandle(b) => b.to_u32(),
            Self::PurpleCandle(b) => b.to_u32(),
            Self::BlueCandle(b) => b.to_u32(),
            Self::BrownCandle(b) => b.to_u32(),
            Self::GreenCandle(b) => b.to_u32(),
            Self::RedCandle(b) => b.to_u32(),
            Self::BlackCandle(b) => b.to_u32(),
            Self::CandleCake(b) => b.to_u32(),
            Self::WhiteCandleCake(b) => b.to_u32(),
            Self::OrangeCandleCake(b) => b.to_u32(),
            Self::MagentaCandleCake(b) => b.to_u32(),
            Self::LightBlueCandleCake(b) => b.to_u32(),
            Self::YellowCandleCake(b) => b.to_u32(),
            Self::LimeCandleCake(b) => b.to_u32(),
            Self::PinkCandleCake(b) => b.to_u32(),
            Self::GrayCandleCake(b) => b.to_u32(),
            Self::LightGrayCandleCake(b) => b.to_u32(),
            Self::CyanCandleCake(b) => b.to_u32(),
            Self::PurpleCandleCake(b) => b.to_u32(),
            Self::BlueCandleCake(b) => b.to_u32(),
            Self::BrownCandleCake(b) => b.to_u32(),
            Self::GreenCandleCake(b) => b.to_u32(),
            Self::RedCandleCake(b) => b.to_u32(),
            Self::BlackCandleCake(b) => b.to_u32(),
            Self::AmethystBlock(b) => b.to_u32(),
            Self::BuddingAmethyst(b) => b.to_u32(),
            Self::AmethystCluster(b) => b.to_u32(),
            Self::LargeAmethystBud(b) => b.to_u32(),
            Self::MediumAmethystBud(b) => b.to_u32(),
            Self::SmallAmethystBud(b) => b.to_u32(),
            Self::Tuff(b) => b.to_u32(),
            Self::Calcite(b) => b.to_u32(),
            Self::TintedGlass(b) => b.to_u32(),
            Self::PowderSnow(b) => b.to_u32(),
            Self::SculkSensor(b) => b.to_u32(),
            Self::CalibratedSculkSensor(b) => b.to_u32(),
            Self::Sculk(b) => b.to_u32(),
            Self::SculkVein(b) => b.to_u32(),
            Self::SculkCatalyst(b) => b.to_u32(),
            Self::SculkShrieker(b) => b.to_u32(),
            Self::OxidizedCopper(b) => b.to_u32(),
            Self::WeatheredCopper(b) => b.to_u32(),
            Self::ExposedCopper(b) => b.to_u32(),
            Self::CopperBlock(b) => b.to_u32(),
            Self::CopperOre(b) => b.to_u32(),
            Self::DeepslateCopperOre(b) => b.to_u32(),
            Self::OxidizedCutCopper(b) => b.to_u32(),
            Self::WeatheredCutCopper(b) => b.to_u32(),
            Self::ExposedCutCopper(b) => b.to_u32(),
            Self::CutCopper(b) => b.to_u32(),
            Self::OxidizedCutCopperStairs(b) => b.to_u32(),
            Self::WeatheredCutCopperStairs(b) => b.to_u32(),
            Self::ExposedCutCopperStairs(b) => b.to_u32(),
            Self::CutCopperStairs(b) => b.to_u32(),
            Self::OxidizedCutCopperSlab(b) => b.to_u32(),
            Self::WeatheredCutCopperSlab(b) => b.to_u32(),
            Self::ExposedCutCopperSlab(b) => b.to_u32(),
            Self::CutCopperSlab(b) => b.to_u32(),
            Self::WaxedCopperBlock(b) => b.to_u32(),
            Self::WaxedWeatheredCopper(b) => b.to_u32(),
            Self::WaxedExposedCopper(b) => b.to_u32(),
            Self::WaxedOxidizedCopper(b) => b.to_u32(),
            Self::WaxedOxidizedCutCopper(b) => b.to_u32(),
            Self::WaxedWeatheredCutCopper(b) => b.to_u32(),
            Self::WaxedExposedCutCopper(b) => b.to_u32(),
            Self::WaxedCutCopper(b) => b.to_u32(),
            Self::WaxedOxidizedCutCopperStairs(b) => b.to_u32(),
            Self::WaxedWeatheredCutCopperStairs(b) => b.to_u32(),
            Self::WaxedExposedCutCopperStairs(b) => b.to_u32(),
            Self::WaxedCutCopperStairs(b) => b.to_u32(),
            Self::WaxedOxidizedCutCopperSlab(b) => b.to_u32(),
            Self::WaxedWeatheredCutCopperSlab(b) => b.to_u32(),
            Self::WaxedExposedCutCopperSlab(b) => b.to_u32(),
            Self::WaxedCutCopperSlab(b) => b.to_u32(),
            Self::LightningRod(b) => b.to_u32(),
            Self::PointedDripstone(b) => b.to_u32(),
            Self::DripstoneBlock(b) => b.to_u32(),
            Self::CaveVines(b) => b.to_u32(),
            Self::CaveVinesPlant(b) => b.to_u32(),
            Self::SporeBlossom(b) => b.to_u32(),
            Self::Azalea(b) => b.to_u32(),
            Self::FloweringAzalea(b) => b.to_u32(),
            Self::MossCarpet(b) => b.to_u32(),
            Self::PinkPetals(b) => b.to_u32(),
            Self::MossBlock(b) => b.to_u32(),
            Self::BigDripleaf(b) => b.to_u32(),
            Self::BigDripleafStem(b) => b.to_u32(),
            Self::SmallDripleaf(b) => b.to_u32(),
            Self::HangingRoots(b) => b.to_u32(),
            Self::RootedDirt(b) => b.to_u32(),
            Self::Mud(b) => b.to_u32(),
            Self::Deepslate(b) => b.to_u32(),
            Self::CobbledDeepslate(b) => b.to_u32(),
            Self::CobbledDeepslateStairs(b) => b.to_u32(),
            Self::CobbledDeepslateSlab(b) => b.to_u32(),
            Self::CobbledDeepslateWall(b) => b.to_u32(),
            Self::PolishedDeepslate(b) => b.to_u32(),
            Self::PolishedDeepslateStairs(b) => b.to_u32(),
            Self::PolishedDeepslateSlab(b) => b.to_u32(),
            Self::PolishedDeepslateWall(b) => b.to_u32(),
            Self::DeepslateTiles(b) => b.to_u32(),
            Self::DeepslateTileStairs(b) => b.to_u32(),
            Self::DeepslateTileSlab(b) => b.to_u32(),
            Self::DeepslateTileWall(b) => b.to_u32(),
            Self::DeepslateBricks(b) => b.to_u32(),
            Self::DeepslateBrickStairs(b) => b.to_u32(),
            Self::DeepslateBrickSlab(b) => b.to_u32(),
            Self::DeepslateBrickWall(b) => b.to_u32(),
            Self::ChiseledDeepslate(b) => b.to_u32(),
            Self::CrackedDeepslateBricks(b) => b.to_u32(),
            Self::CrackedDeepslateTiles(b) => b.to_u32(),
            Self::InfestedDeepslate(b) => b.to_u32(),
            Self::SmoothBasalt(b) => b.to_u32(),
            Self::RawIronBlock(b) => b.to_u32(),
            Self::RawCopperBlock(b) => b.to_u32(),
            Self::RawGoldBlock(b) => b.to_u32(),
            Self::PottedAzaleaBush(b) => b.to_u32(),
            Self::PottedFloweringAzaleaBush(b) => b.to_u32(),
            Self::OchreFroglight(b) => b.to_u32(),
            Self::VerdantFroglight(b) => b.to_u32(),
            Self::PearlescentFroglight(b) => b.to_u32(),
            Self::Frogspawn(b) => b.to_u32(),
            Self::ReinforcedDeepslate(b) => b.to_u32(),
            Self::DecoratedPot(b) => b.to_u32(),
        }
    }
    fn from_u32(id: u32) -> Self {
        match id {
            0u32 => BlockAir::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1u32 => BlockStone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2u32 => BlockGranite::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            3u32 => BlockPolishedGranite::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4u32 => BlockDiorite::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5u32 => BlockPolishedDiorite::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6u32 => BlockAndesite::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7u32 => BlockPolishedAndesite::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8u32..=9u32 => BlockGrassBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10u32 => BlockDirt::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11u32 => BlockCoarseDirt::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12u32..=13u32 => BlockPodzol::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            14u32 => BlockCobblestone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            15u32 => BlockOakPlanks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            16u32 => BlockSprucePlanks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            17u32 => BlockBirchPlanks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18u32 => BlockJunglePlanks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19u32 => BlockAcaciaPlanks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20u32 => BlockCherryPlanks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21u32 => BlockDarkOakPlanks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22u32 => BlockMangrovePlanks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23u32 => BlockBambooPlanks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            24u32 => BlockBambooMosaic::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            25u32..=26u32 => BlockOakSapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            27u32..=28u32 => BlockSpruceSapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            29u32..=30u32 => BlockBirchSapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            31u32..=32u32 => BlockJungleSapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            33u32..=34u32 => BlockAcaciaSapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            35u32..=36u32 => BlockCherrySapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            37u32..=38u32 => BlockDarkOakSapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            39u32..=78u32 => BlockMangrovePropagule::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            79u32 => BlockBedrock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            80u32..=95u32 => BlockWater::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            96u32..=111u32 => BlockLava::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            112u32 => BlockSand::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            113u32..=116u32 => BlockSuspiciousSand::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            117u32 => BlockRedSand::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            118u32 => BlockGravel::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            119u32..=122u32 => BlockSuspiciousGravel::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            123u32 => BlockGoldOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            124u32 => BlockDeepslateGoldOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            125u32 => BlockIronOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            126u32 => BlockDeepslateIronOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            127u32 => BlockCoalOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            128u32 => BlockDeepslateCoalOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            129u32 => BlockNetherGoldOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            130u32..=132u32 => BlockOakLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            133u32..=135u32 => BlockSpruceLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            136u32..=138u32 => BlockBirchLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            139u32..=141u32 => BlockJungleLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            142u32..=144u32 => BlockAcaciaLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            145u32..=147u32 => BlockCherryLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            148u32..=150u32 => BlockDarkOakLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            151u32..=153u32 => BlockMangroveLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            154u32..=155u32 => BlockMangroveRoots::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            156u32..=158u32 => BlockMuddyMangroveRoots::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            159u32..=161u32 => BlockBambooBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            162u32..=164u32 => BlockStrippedSpruceLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            165u32..=167u32 => BlockStrippedBirchLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            168u32..=170u32 => BlockStrippedJungleLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            171u32..=173u32 => BlockStrippedAcaciaLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            174u32..=176u32 => BlockStrippedCherryLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            177u32..=179u32 => BlockStrippedDarkOakLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            180u32..=182u32 => BlockStrippedOakLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            183u32..=185u32 => BlockStrippedMangroveLog::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            186u32..=188u32 => BlockStrippedBambooBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            189u32..=191u32 => BlockOakWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            192u32..=194u32 => BlockSpruceWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            195u32..=197u32 => BlockBirchWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            198u32..=200u32 => BlockJungleWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            201u32..=203u32 => BlockAcaciaWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            204u32..=206u32 => BlockCherryWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            207u32..=209u32 => BlockDarkOakWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            210u32..=212u32 => BlockMangroveWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            213u32..=215u32 => BlockStrippedOakWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            216u32..=218u32 => BlockStrippedSpruceWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            219u32..=221u32 => BlockStrippedBirchWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            222u32..=224u32 => BlockStrippedJungleWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            225u32..=227u32 => BlockStrippedAcaciaWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            228u32..=230u32 => BlockStrippedCherryWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            231u32..=233u32 => BlockStrippedDarkOakWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            234u32..=236u32 => BlockStrippedMangroveWood::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            237u32..=264u32 => BlockOakLeaves::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            265u32..=292u32 => BlockSpruceLeaves::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            293u32..=320u32 => BlockBirchLeaves::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            321u32..=348u32 => BlockJungleLeaves::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            349u32..=376u32 => BlockAcaciaLeaves::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            377u32..=404u32 => BlockCherryLeaves::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            405u32..=432u32 => BlockDarkOakLeaves::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            433u32..=460u32 => BlockMangroveLeaves::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            461u32..=488u32 => BlockAzaleaLeaves::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            489u32..=516u32 => BlockFloweringAzaleaLeaves::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            517u32 => BlockSponge::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            518u32 => BlockWetSponge::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            519u32 => BlockGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            520u32 => BlockLapisOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            521u32 => BlockDeepslateLapisOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            522u32 => BlockLapisBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            523u32..=534u32 => BlockDispenser::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            535u32 => BlockSandstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            536u32 => BlockChiseledSandstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            537u32 => BlockCutSandstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            538u32..=1687u32 => BlockNoteBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1688u32..=1703u32 => BlockWhiteBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1704u32..=1719u32 => BlockOrangeBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1720u32..=1735u32 => BlockMagentaBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1736u32..=1751u32 => BlockLightBlueBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1752u32..=1767u32 => BlockYellowBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1768u32..=1783u32 => BlockLimeBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1784u32..=1799u32 => BlockPinkBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1800u32..=1815u32 => BlockGrayBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1816u32..=1831u32 => BlockLightGrayBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1832u32..=1847u32 => BlockCyanBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1848u32..=1863u32 => BlockPurpleBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1864u32..=1879u32 => BlockBlueBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1880u32..=1895u32 => BlockBrownBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1896u32..=1911u32 => BlockGreenBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1912u32..=1927u32 => BlockRedBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1928u32..=1943u32 => BlockBlackBed::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1944u32..=1983u32 => BlockPoweredRail::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            1984u32..=2023u32 => BlockDetectorRail::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2024u32..=2035u32 => BlockStickyPiston::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2036u32 => BlockCobweb::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2037u32 => BlockGrass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2038u32 => BlockFern::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2039u32 => BlockDeadBush::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2040u32 => BlockSeagrass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2041u32..=2042u32 => BlockTallSeagrass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2043u32..=2054u32 => BlockPiston::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2055u32..=2078u32 => BlockPistonHead::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2079u32 => BlockWhiteWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2080u32 => BlockOrangeWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2081u32 => BlockMagentaWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2082u32 => BlockLightBlueWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2083u32 => BlockYellowWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2084u32 => BlockLimeWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2085u32 => BlockPinkWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2086u32 => BlockGrayWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2087u32 => BlockLightGrayWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2088u32 => BlockCyanWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2089u32 => BlockPurpleWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2090u32 => BlockBlueWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2091u32 => BlockBrownWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2092u32 => BlockGreenWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2093u32 => BlockRedWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2094u32 => BlockBlackWool::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2095u32..=2096u32 => BlockMovingPiston::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2097u32 => BlockDandelion::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2098u32 => BlockTorchflower::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2099u32 => BlockPoppy::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2100u32 => BlockBlueOrchid::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2101u32 => BlockAllium::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2102u32 => BlockAzureBluet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2103u32 => BlockRedTulip::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2104u32 => BlockOrangeTulip::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2105u32 => BlockWhiteTulip::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2106u32 => BlockPinkTulip::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2107u32 => BlockOxeyeDaisy::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2108u32 => BlockCornflower::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2109u32 => BlockWitherRose::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2110u32 => BlockLilyOfTheValley::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2111u32 => BlockBrownMushroom::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2112u32 => BlockRedMushroom::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2113u32 => BlockGoldBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2114u32 => BlockIronBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2115u32 => BlockBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2116u32..=2117u32 => BlockTnt::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2118u32 => BlockBookshelf::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2119u32 => BlockChiseledBookshelf::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2120u32 => BlockMossyCobblestone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2121u32 => BlockObsidian::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2122u32 => BlockTorch::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2123u32..=2126u32 => BlockWallTorch::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2127u32..=2638u32 => BlockFire::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2639u32 => BlockSoulFire::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2640u32 => BlockSpawner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2641u32..=2720u32 => BlockOakStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2721u32..=2744u32 => BlockChest::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            2745u32..=4040u32 => BlockRedstoneWire::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4041u32 => BlockDiamondOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4042u32 => BlockDeepslateDiamondOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4043u32 => BlockDiamondBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4044u32 => BlockCraftingTable::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4045u32..=4052u32 => BlockWheat::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4053u32..=4060u32 => BlockFarmland::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4061u32..=4068u32 => BlockFurnace::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4069u32..=4100u32 => BlockOakSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4101u32..=4132u32 => BlockSpruceSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4133u32..=4164u32 => BlockBirchSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4165u32..=4196u32 => BlockAcaciaSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4197u32..=4228u32 => BlockCherrySign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4229u32..=4260u32 => BlockJungleSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4261u32..=4292u32 => BlockDarkOakSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4293u32..=4324u32 => BlockMangroveSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4325u32..=4356u32 => BlockBambooSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4357u32..=4420u32 => BlockOakDoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4421u32..=4428u32 => BlockLadder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4429u32..=4448u32 => BlockRail::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4449u32..=4528u32 => BlockCobblestoneStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4529u32..=4536u32 => BlockOakWallSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4537u32..=4544u32 => BlockSpruceWallSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4545u32..=4552u32 => BlockBirchWallSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4553u32..=4560u32 => BlockAcaciaWallSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4561u32..=4568u32 => BlockCherryWallSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4569u32..=4576u32 => BlockJungleWallSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4577u32..=4584u32 => BlockDarkOakWallSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4585u32..=4592u32 => BlockMangroveWallSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4593u32..=4600u32 => BlockBambooWallSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4601u32..=4664u32 => BlockOakHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4665u32..=4728u32 => BlockSpruceHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4729u32..=4792u32 => BlockBirchHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4793u32..=4856u32 => BlockAcaciaHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4857u32..=4920u32 => BlockCherryHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4921u32..=4984u32 => BlockJungleHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            4985u32..=5048u32 => BlockDarkOakHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5049u32..=5112u32 => BlockCrimsonHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5113u32..=5176u32 => BlockWarpedHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5177u32..=5240u32 => BlockMangroveHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5241u32..=5304u32 => BlockBambooHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5305u32..=5312u32 => BlockOakWallHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5313u32..=5320u32 => BlockSpruceWallHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5321u32..=5328u32 => BlockBirchWallHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5329u32..=5336u32 => BlockAcaciaWallHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5337u32..=5344u32 => BlockCherryWallHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5345u32..=5352u32 => BlockJungleWallHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5353u32..=5360u32 => BlockDarkOakWallHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5361u32..=5368u32 => BlockMangroveWallHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5369u32..=5376u32 => BlockCrimsonWallHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5377u32..=5384u32 => BlockWarpedWallHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5385u32..=5392u32 => BlockBambooWallHangingSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5393u32..=5416u32 => BlockLever::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5417u32..=5418u32 => BlockStonePressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5419u32..=5482u32 => BlockIronDoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5483u32..=5484u32 => BlockOakPressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5485u32..=5486u32 => BlockSprucePressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5487u32..=5488u32 => BlockBirchPressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5489u32..=5490u32 => BlockJunglePressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5491u32..=5492u32 => BlockAcaciaPressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5493u32..=5494u32 => BlockCherryPressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5495u32..=5496u32 => BlockDarkOakPressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5497u32..=5498u32 => BlockMangrovePressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5499u32..=5500u32 => BlockBambooPressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5501u32..=5502u32 => BlockRedstoneOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5503u32..=5504u32 => BlockDeepslateRedstoneOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5505u32..=5506u32 => BlockRedstoneTorch::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5507u32..=5514u32 => BlockRedstoneWallTorch::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5515u32..=5538u32 => BlockStoneButton::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5539u32..=5546u32 => BlockSnow::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5547u32 => BlockIce::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5548u32 => BlockSnowBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5549u32..=5564u32 => BlockCactus::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5565u32 => BlockClay::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5566u32..=5581u32 => BlockSugarCane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5582u32..=5583u32 => BlockJukebox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5584u32..=5615u32 => BlockOakFence::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5616u32 => BlockPumpkin::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5617u32 => BlockNetherrack::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5618u32 => BlockSoulSand::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5619u32 => BlockSoulSoil::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5620u32..=5622u32 => BlockBasalt::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5623u32..=5625u32 => BlockPolishedBasalt::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5626u32 => BlockSoulTorch::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5627u32..=5630u32 => BlockSoulWallTorch::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5631u32 => BlockGlowstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5632u32..=5634u32 => BlockNetherPortal::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5635u32..=5638u32 => BlockCarvedPumpkin::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5639u32..=5642u32 => BlockJackOLantern::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5643u32..=5649u32 => BlockCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5650u32..=5713u32 => BlockRepeater::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5714u32 => BlockWhiteStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5715u32 => BlockOrangeStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5716u32 => BlockMagentaStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5717u32 => BlockLightBlueStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5718u32 => BlockYellowStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5719u32 => BlockLimeStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5720u32 => BlockPinkStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5721u32 => BlockGrayStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5722u32 => BlockLightGrayStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5723u32 => BlockCyanStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5724u32 => BlockPurpleStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5725u32 => BlockBlueStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5726u32 => BlockBrownStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5727u32 => BlockGreenStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5728u32 => BlockRedStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5729u32 => BlockBlackStainedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5730u32..=5793u32 => BlockOakTrapdoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5794u32..=5857u32 => BlockSpruceTrapdoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5858u32..=5921u32 => BlockBirchTrapdoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5922u32..=5985u32 => BlockJungleTrapdoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            5986u32..=6049u32 => BlockAcaciaTrapdoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6050u32..=6113u32 => BlockCherryTrapdoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6114u32..=6177u32 => BlockDarkOakTrapdoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6178u32..=6241u32 => BlockMangroveTrapdoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6242u32..=6305u32 => BlockBambooTrapdoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6306u32 => BlockStoneBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6307u32 => BlockMossyStoneBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6308u32 => BlockCrackedStoneBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6309u32 => BlockChiseledStoneBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6310u32 => BlockPackedMud::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6311u32 => BlockMudBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6312u32 => BlockInfestedStone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6313u32 => BlockInfestedCobblestone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6314u32 => BlockInfestedStoneBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6315u32 => BlockInfestedMossyStoneBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6316u32 => BlockInfestedCrackedStoneBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6317u32 => BlockInfestedChiseledStoneBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6318u32..=6381u32 => BlockBrownMushroomBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6382u32..=6445u32 => BlockRedMushroomBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6446u32..=6509u32 => BlockMushroomStem::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6510u32..=6541u32 => BlockIronBars::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6542u32..=6547u32 => BlockChain::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6548u32..=6579u32 => BlockGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6580u32 => BlockMelon::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6581u32..=6584u32 => BlockAttachedPumpkinStem::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6585u32..=6588u32 => BlockAttachedMelonStem::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6589u32..=6596u32 => BlockPumpkinStem::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6597u32..=6604u32 => BlockMelonStem::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6605u32..=6636u32 => BlockVine::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6637u32..=6638u32 => BlockGlowLichen::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6639u32..=6670u32 => BlockOakFenceGate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6671u32..=6750u32 => BlockBrickStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6751u32..=6830u32 => BlockStoneBrickStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6831u32..=6910u32 => BlockMudBrickStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6911u32..=6912u32 => BlockMycelium::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6913u32 => BlockLilyPad::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6914u32 => BlockNetherBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6915u32..=6946u32 => BlockNetherBrickFence::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            6947u32..=7026u32 => BlockNetherBrickStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7027u32..=7030u32 => BlockNetherWart::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7031u32 => BlockEnchantingTable::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7032u32 => BlockBrewingStand::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7033u32 => BlockCauldron::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7034u32..=7036u32 => BlockWaterCauldron::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7037u32 => BlockLavaCauldron::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7038u32..=7040u32 => BlockPowderSnowCauldron::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7041u32 => BlockEndPortal::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7042u32..=7049u32 => BlockEndPortalFrame::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7050u32 => BlockEndStone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7051u32 => BlockDragonEgg::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7052u32..=7053u32 => BlockRedstoneLamp::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7054u32..=7065u32 => BlockCocoa::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7066u32..=7145u32 => BlockSandstoneStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7146u32 => BlockEmeraldOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7147u32 => BlockDeepslateEmeraldOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7148u32..=7155u32 => BlockEnderChest::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7156u32..=7171u32 => BlockTripwireHook::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7172u32..=7299u32 => BlockTripwire::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7300u32 => BlockEmeraldBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7301u32..=7380u32 => BlockSpruceStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7381u32..=7460u32 => BlockBirchStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7461u32..=7540u32 => BlockJungleStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7541u32..=7552u32 => BlockCommandBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7553u32 => BlockBeacon::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7554u32..=7877u32 => BlockCobblestoneWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            7878u32..=8201u32 => BlockMossyCobblestoneWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8202u32 => BlockFlowerPot::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8203u32 => BlockPottedTorchflower::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8204u32 => BlockPottedOakSapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8205u32 => BlockPottedSpruceSapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8206u32 => BlockPottedBirchSapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8207u32 => BlockPottedJungleSapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8208u32 => BlockPottedAcaciaSapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8209u32 => BlockPottedCherrySapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8210u32 => BlockPottedDarkOakSapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8211u32 => BlockPottedMangrovePropagule::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8212u32 => BlockPottedFern::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8213u32 => BlockPottedDandelion::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8214u32 => BlockPottedPoppy::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8215u32 => BlockPottedBlueOrchid::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8216u32 => BlockPottedAllium::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8217u32 => BlockPottedAzureBluet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8218u32 => BlockPottedRedTulip::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8219u32 => BlockPottedOrangeTulip::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8220u32 => BlockPottedWhiteTulip::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8221u32 => BlockPottedPinkTulip::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8222u32 => BlockPottedOxeyeDaisy::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8223u32 => BlockPottedCornflower::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8224u32 => BlockPottedLilyOfTheValley::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8225u32 => BlockPottedWitherRose::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8226u32 => BlockPottedRedMushroom::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8227u32 => BlockPottedBrownMushroom::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8228u32 => BlockPottedDeadBush::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8229u32 => BlockPottedCactus::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8230u32..=8237u32 => BlockCarrots::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8238u32..=8245u32 => BlockPotatoes::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8246u32..=8269u32 => BlockOakButton::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8270u32..=8293u32 => BlockSpruceButton::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8294u32..=8317u32 => BlockBirchButton::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8318u32..=8341u32 => BlockJungleButton::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8342u32..=8365u32 => BlockAcaciaButton::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8366u32..=8389u32 => BlockCherryButton::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8390u32..=8413u32 => BlockDarkOakButton::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8414u32..=8437u32 => BlockMangroveButton::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8438u32..=8461u32 => BlockBambooButton::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8462u32..=8477u32 => BlockSkeletonSkull::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8478u32..=8481u32 => BlockSkeletonWallSkull::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8482u32..=8497u32 => BlockWitherSkeletonSkull::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8498u32..=8501u32 => BlockWitherSkeletonWallSkull::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8502u32..=8517u32 => BlockZombieHead::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8518u32..=8521u32 => BlockZombieWallHead::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8522u32..=8537u32 => BlockPlayerHead::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8538u32..=8541u32 => BlockPlayerWallHead::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8542u32..=8557u32 => BlockCreeperHead::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8558u32..=8561u32 => BlockCreeperWallHead::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8562u32..=8577u32 => BlockDragonHead::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8578u32..=8581u32 => BlockDragonWallHead::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8582u32..=8597u32 => BlockPiglinHead::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8598u32..=8601u32 => BlockPiglinWallHead::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8602u32..=8605u32 => BlockAnvil::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8606u32..=8609u32 => BlockChippedAnvil::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8610u32..=8613u32 => BlockDamagedAnvil::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8614u32..=8637u32 => BlockTrappedChest::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8638u32..=8653u32 => BlockLightWeightedPressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8654u32..=8669u32 => BlockHeavyWeightedPressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8670u32..=8685u32 => BlockComparator::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8686u32..=8717u32 => BlockDaylightDetector::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8718u32 => BlockRedstoneBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8719u32 => BlockNetherQuartzOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8720u32..=8729u32 => BlockHopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8730u32 => BlockQuartzBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8731u32 => BlockChiseledQuartzBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8732u32..=8734u32 => BlockQuartzPillar::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8735u32..=8814u32 => BlockQuartzStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8815u32..=8854u32 => BlockActivatorRail::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8855u32..=8866u32 => BlockDropper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8867u32 => BlockWhiteTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8868u32 => BlockOrangeTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8869u32 => BlockMagentaTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8870u32 => BlockLightBlueTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8871u32 => BlockYellowTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8872u32 => BlockLimeTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8873u32 => BlockPinkTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8874u32 => BlockGrayTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8875u32 => BlockLightGrayTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8876u32 => BlockCyanTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8877u32 => BlockPurpleTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8878u32 => BlockBlueTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8879u32 => BlockBrownTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8880u32 => BlockGreenTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8881u32 => BlockRedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8882u32 => BlockBlackTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8883u32..=8914u32 => BlockWhiteStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8915u32..=8946u32 => BlockOrangeStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8947u32..=8978u32 => BlockMagentaStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            8979u32..=9010u32 => BlockLightBlueStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9011u32..=9042u32 => BlockYellowStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9043u32..=9074u32 => BlockLimeStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9075u32..=9106u32 => BlockPinkStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9107u32..=9138u32 => BlockGrayStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9139u32..=9170u32 => BlockLightGrayStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9171u32..=9202u32 => BlockCyanStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9203u32..=9234u32 => BlockPurpleStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9235u32..=9266u32 => BlockBlueStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9267u32..=9298u32 => BlockBrownStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9299u32..=9330u32 => BlockGreenStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9331u32..=9362u32 => BlockRedStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9363u32..=9394u32 => BlockBlackStainedGlassPane::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9395u32..=9474u32 => BlockAcaciaStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9475u32..=9554u32 => BlockCherryStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9555u32..=9634u32 => BlockDarkOakStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9635u32..=9714u32 => BlockMangroveStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9715u32..=9794u32 => BlockBambooStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9795u32..=9874u32 => BlockBambooMosaicStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9875u32 => BlockSlimeBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9876u32 => BlockBarrier::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9877u32..=9908u32 => BlockLight::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9909u32..=9972u32 => BlockIronTrapdoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9973u32 => BlockPrismarine::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9974u32 => BlockPrismarineBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9975u32 => BlockDarkPrismarine::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            9976u32..=10055u32 => BlockPrismarineStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10056u32..=10135u32 => BlockPrismarineBrickStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10136u32..=10215u32 => BlockDarkPrismarineStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10216u32..=10221u32 => BlockPrismarineSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10222u32..=10227u32 => BlockPrismarineBrickSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10228u32..=10233u32 => BlockDarkPrismarineSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10234u32 => BlockSeaLantern::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10235u32..=10237u32 => BlockHayBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10238u32 => BlockWhiteCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10239u32 => BlockOrangeCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10240u32 => BlockMagentaCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10241u32 => BlockLightBlueCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10242u32 => BlockYellowCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10243u32 => BlockLimeCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10244u32 => BlockPinkCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10245u32 => BlockGrayCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10246u32 => BlockLightGrayCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10247u32 => BlockCyanCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10248u32 => BlockPurpleCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10249u32 => BlockBlueCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10250u32 => BlockBrownCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10251u32 => BlockGreenCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10252u32 => BlockRedCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10253u32 => BlockBlackCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10254u32 => BlockTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10255u32 => BlockCoalBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10256u32 => BlockPackedIce::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10257u32..=10258u32 => BlockSunflower::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10259u32..=10260u32 => BlockLilac::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10261u32..=10262u32 => BlockRoseBush::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10263u32..=10264u32 => BlockPeony::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10265u32..=10266u32 => BlockTallGrass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10267u32..=10268u32 => BlockLargeFern::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10269u32..=10284u32 => BlockWhiteBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10285u32..=10300u32 => BlockOrangeBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10301u32..=10316u32 => BlockMagentaBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10317u32..=10332u32 => BlockLightBlueBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10333u32..=10348u32 => BlockYellowBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10349u32..=10364u32 => BlockLimeBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10365u32..=10380u32 => BlockPinkBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10381u32..=10396u32 => BlockGrayBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10397u32..=10412u32 => BlockLightGrayBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10413u32..=10428u32 => BlockCyanBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10429u32..=10444u32 => BlockPurpleBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10445u32..=10460u32 => BlockBlueBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10461u32..=10476u32 => BlockBrownBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10477u32..=10492u32 => BlockGreenBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10493u32..=10508u32 => BlockRedBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10509u32..=10524u32 => BlockBlackBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10525u32..=10528u32 => BlockWhiteWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10529u32..=10532u32 => BlockOrangeWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10533u32..=10536u32 => BlockMagentaWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10537u32..=10540u32 => BlockLightBlueWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10541u32..=10544u32 => BlockYellowWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10545u32..=10548u32 => BlockLimeWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10549u32..=10552u32 => BlockPinkWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10553u32..=10556u32 => BlockGrayWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10557u32..=10560u32 => BlockLightGrayWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10561u32..=10564u32 => BlockCyanWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10565u32..=10568u32 => BlockPurpleWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10569u32..=10572u32 => BlockBlueWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10573u32..=10576u32 => BlockBrownWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10577u32..=10580u32 => BlockGreenWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10581u32..=10584u32 => BlockRedWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10585u32..=10588u32 => BlockBlackWallBanner::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10589u32 => BlockRedSandstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10590u32 => BlockChiseledRedSandstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10591u32 => BlockCutRedSandstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10592u32..=10671u32 => BlockRedSandstoneStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10672u32..=10677u32 => BlockOakSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10678u32..=10683u32 => BlockSpruceSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10684u32..=10689u32 => BlockBirchSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10690u32..=10695u32 => BlockJungleSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10696u32..=10701u32 => BlockAcaciaSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10702u32..=10707u32 => BlockCherrySlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10708u32..=10713u32 => BlockDarkOakSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10714u32..=10719u32 => BlockMangroveSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10720u32..=10725u32 => BlockBambooSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10726u32..=10731u32 => BlockBambooMosaicSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10732u32..=10737u32 => BlockStoneSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10738u32..=10743u32 => BlockSmoothStoneSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10744u32..=10749u32 => BlockSandstoneSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10750u32..=10755u32 => BlockCutSandstoneSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10756u32..=10761u32 => BlockPetrifiedOakSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10762u32..=10767u32 => BlockCobblestoneSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10768u32..=10773u32 => BlockBrickSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10774u32..=10779u32 => BlockStoneBrickSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10780u32..=10785u32 => BlockMudBrickSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10786u32..=10791u32 => BlockNetherBrickSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10792u32..=10797u32 => BlockQuartzSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10798u32..=10803u32 => BlockRedSandstoneSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10804u32..=10809u32 => BlockCutRedSandstoneSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10810u32..=10815u32 => BlockPurpurSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10816u32 => BlockSmoothStone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10817u32 => BlockSmoothSandstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10818u32 => BlockSmoothQuartz::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10819u32 => BlockSmoothRedSandstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10820u32..=10851u32 => BlockSpruceFenceGate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10852u32..=10883u32 => BlockBirchFenceGate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10884u32..=10915u32 => BlockJungleFenceGate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10916u32..=10947u32 => BlockAcaciaFenceGate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10948u32..=10979u32 => BlockCherryFenceGate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            10980u32..=11011u32 => BlockDarkOakFenceGate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11012u32..=11043u32 => BlockMangroveFenceGate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11044u32..=11075u32 => BlockBambooFenceGate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11076u32..=11107u32 => BlockSpruceFence::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11108u32..=11139u32 => BlockBirchFence::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11140u32..=11171u32 => BlockJungleFence::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11172u32..=11203u32 => BlockAcaciaFence::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11204u32..=11235u32 => BlockCherryFence::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11236u32..=11267u32 => BlockDarkOakFence::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11268u32..=11299u32 => BlockMangroveFence::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11300u32..=11331u32 => BlockBambooFence::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11332u32..=11395u32 => BlockSpruceDoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11396u32..=11459u32 => BlockBirchDoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11460u32..=11523u32 => BlockJungleDoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11524u32..=11587u32 => BlockAcaciaDoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11588u32..=11651u32 => BlockCherryDoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11652u32..=11715u32 => BlockDarkOakDoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11716u32..=11779u32 => BlockMangroveDoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11780u32..=11843u32 => BlockBambooDoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11844u32..=11849u32 => BlockEndRod::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11850u32..=11913u32 => BlockChorusPlant::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11914u32..=11919u32 => BlockChorusFlower::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11920u32 => BlockPurpurBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11921u32..=11923u32 => BlockPurpurPillar::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            11924u32..=12003u32 => BlockPurpurStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12004u32 => BlockEndStoneBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12005u32..=12020u32 => BlockTorchflowerCrop::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12021u32..=12030u32 => BlockPitcherCrop::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12031u32..=12032u32 => BlockPitcherPlant::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12033u32..=12064u32 => BlockBeetroots::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12065u32 => BlockDirtPath::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12066u32 => BlockEndGateway::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12067u32..=12078u32 => BlockRepeatingCommandBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12079u32..=12090u32 => BlockChainCommandBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12091u32..=12094u32 => BlockFrostedIce::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12095u32 => BlockMagmaBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12096u32 => BlockNetherWartBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12097u32 => BlockRedNetherBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12098u32..=12100u32 => BlockBoneBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12101u32 => BlockStructureVoid::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12102u32..=12113u32 => BlockObserver::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12114u32..=12119u32 => BlockShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12120u32..=12125u32 => BlockWhiteShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12126u32..=12131u32 => BlockOrangeShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12132u32..=12137u32 => BlockMagentaShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12138u32..=12143u32 => BlockLightBlueShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12144u32..=12149u32 => BlockYellowShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12150u32..=12155u32 => BlockLimeShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12156u32..=12161u32 => BlockPinkShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12162u32..=12167u32 => BlockGrayShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12168u32..=12173u32 => BlockLightGrayShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12174u32..=12179u32 => BlockCyanShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12180u32..=12185u32 => BlockPurpleShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12186u32..=12191u32 => BlockBlueShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12192u32..=12197u32 => BlockBrownShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12198u32..=12203u32 => BlockGreenShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12204u32..=12209u32 => BlockRedShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12210u32..=12215u32 => BlockBlackShulkerBox::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12216u32..=12219u32 => BlockWhiteGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12220u32..=12223u32 => BlockOrangeGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12224u32..=12227u32 => BlockMagentaGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12228u32..=12231u32 => BlockLightBlueGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12232u32..=12235u32 => BlockYellowGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12236u32..=12239u32 => BlockLimeGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12240u32..=12243u32 => BlockPinkGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12244u32..=12247u32 => BlockGrayGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12248u32..=12251u32 => BlockLightGrayGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12252u32..=12255u32 => BlockCyanGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12256u32..=12259u32 => BlockPurpleGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12260u32..=12263u32 => BlockBlueGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12264u32..=12267u32 => BlockBrownGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12268u32..=12271u32 => BlockGreenGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12272u32..=12275u32 => BlockRedGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12276u32..=12279u32 => BlockBlackGlazedTerracotta::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12280u32 => BlockWhiteConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12281u32 => BlockOrangeConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12282u32 => BlockMagentaConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12283u32 => BlockLightBlueConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12284u32 => BlockYellowConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12285u32 => BlockLimeConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12286u32 => BlockPinkConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12287u32 => BlockGrayConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12288u32 => BlockLightGrayConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12289u32 => BlockCyanConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12290u32 => BlockPurpleConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12291u32 => BlockBlueConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12292u32 => BlockBrownConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12293u32 => BlockGreenConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12294u32 => BlockRedConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12295u32 => BlockBlackConcrete::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12296u32 => BlockWhiteConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12297u32 => BlockOrangeConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12298u32 => BlockMagentaConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12299u32 => BlockLightBlueConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12300u32 => BlockYellowConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12301u32 => BlockLimeConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12302u32 => BlockPinkConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12303u32 => BlockGrayConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12304u32 => BlockLightGrayConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12305u32 => BlockCyanConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12306u32 => BlockPurpleConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12307u32 => BlockBlueConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12308u32 => BlockBrownConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12309u32 => BlockGreenConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12310u32 => BlockRedConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12311u32 => BlockBlackConcretePowder::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12312u32..=12337u32 => BlockKelp::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12338u32 => BlockKelpPlant::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12339u32 => BlockDriedKelpBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12340u32..=12351u32 => BlockTurtleEgg::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12352u32..=12354u32 => BlockSnifferEgg::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12355u32 => BlockDeadTubeCoralBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12356u32 => BlockDeadBrainCoralBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12357u32 => BlockDeadBubbleCoralBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12358u32 => BlockDeadFireCoralBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12359u32 => BlockDeadHornCoralBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12360u32 => BlockTubeCoralBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12361u32 => BlockBrainCoralBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12362u32 => BlockBubbleCoralBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12363u32 => BlockFireCoralBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12364u32 => BlockHornCoralBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12365u32..=12366u32 => BlockDeadTubeCoral::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12367u32..=12368u32 => BlockDeadBrainCoral::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12369u32..=12370u32 => BlockDeadBubbleCoral::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12371u32..=12372u32 => BlockDeadFireCoral::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12373u32..=12374u32 => BlockDeadHornCoral::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12375u32..=12376u32 => BlockTubeCoral::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12377u32..=12378u32 => BlockBrainCoral::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12379u32..=12380u32 => BlockBubbleCoral::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12381u32..=12382u32 => BlockFireCoral::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12383u32..=12384u32 => BlockHornCoral::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12385u32..=12386u32 => BlockDeadTubeCoralFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12387u32..=12388u32 => BlockDeadBrainCoralFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12389u32..=12390u32 => BlockDeadBubbleCoralFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12391u32..=12392u32 => BlockDeadFireCoralFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12393u32..=12394u32 => BlockDeadHornCoralFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12395u32..=12396u32 => BlockTubeCoralFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12397u32..=12398u32 => BlockBrainCoralFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12399u32..=12400u32 => BlockBubbleCoralFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12401u32..=12402u32 => BlockFireCoralFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12403u32..=12404u32 => BlockHornCoralFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12405u32..=12412u32 => BlockDeadTubeCoralWallFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12413u32..=12420u32 => BlockDeadBrainCoralWallFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12421u32..=12428u32 => BlockDeadBubbleCoralWallFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12429u32..=12436u32 => BlockDeadFireCoralWallFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12437u32..=12444u32 => BlockDeadHornCoralWallFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12445u32..=12452u32 => BlockTubeCoralWallFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12453u32..=12460u32 => BlockBrainCoralWallFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12461u32..=12468u32 => BlockBubbleCoralWallFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12469u32..=12476u32 => BlockFireCoralWallFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12477u32..=12484u32 => BlockHornCoralWallFan::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12485u32..=12492u32 => BlockSeaPickle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12493u32 => BlockBlueIce::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12494u32..=12495u32 => BlockConduit::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12496u32 => BlockBambooSapling::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12497u32..=12508u32 => BlockBamboo::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12509u32 => BlockPottedBamboo::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12510u32 => BlockVoidAir::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12511u32 => BlockCaveAir::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12512u32..=12513u32 => BlockBubbleColumn::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12514u32..=12593u32 => BlockPolishedGraniteStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12594u32..=12673u32 => BlockSmoothRedSandstoneStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12674u32..=12753u32 => BlockMossyStoneBrickStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12754u32..=12833u32 => BlockPolishedDioriteStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12834u32..=12913u32 => BlockMossyCobblestoneStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12914u32..=12993u32 => BlockEndStoneBrickStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            12994u32..=13073u32 => BlockStoneStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13074u32..=13153u32 => BlockSmoothSandstoneStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13154u32..=13233u32 => BlockSmoothQuartzStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13234u32..=13313u32 => BlockGraniteStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13314u32..=13393u32 => BlockAndesiteStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13394u32..=13473u32 => BlockRedNetherBrickStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13474u32..=13553u32 => BlockPolishedAndesiteStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13554u32..=13633u32 => BlockDioriteStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13634u32..=13639u32 => BlockPolishedGraniteSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13640u32..=13645u32 => BlockSmoothRedSandstoneSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13646u32..=13651u32 => BlockMossyStoneBrickSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13652u32..=13657u32 => BlockPolishedDioriteSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13658u32..=13663u32 => BlockMossyCobblestoneSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13664u32..=13669u32 => BlockEndStoneBrickSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13670u32..=13675u32 => BlockSmoothSandstoneSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13676u32..=13681u32 => BlockSmoothQuartzSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13682u32..=13687u32 => BlockGraniteSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13688u32..=13693u32 => BlockAndesiteSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13694u32..=13699u32 => BlockRedNetherBrickSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13700u32..=13705u32 => BlockPolishedAndesiteSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13706u32..=13711u32 => BlockDioriteSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            13712u32..=14035u32 => BlockBrickWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            14036u32..=14359u32 => BlockPrismarineWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            14360u32..=14683u32 => BlockRedSandstoneWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            14684u32..=15007u32 => BlockMossyStoneBrickWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            15008u32..=15331u32 => BlockGraniteWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            15332u32..=15655u32 => BlockStoneBrickWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            15656u32..=15979u32 => BlockMudBrickWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            15980u32..=16303u32 => BlockNetherBrickWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            16304u32..=16627u32 => BlockAndesiteWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            16628u32..=16951u32 => BlockRedNetherBrickWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            16952u32..=17275u32 => BlockSandstoneWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            17276u32..=17599u32 => BlockEndStoneBrickWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            17600u32..=17923u32 => BlockDioriteWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            17924u32..=17955u32 => BlockScaffolding::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            17956u32..=17959u32 => BlockLoom::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            17960u32..=17971u32 => BlockBarrel::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            17972u32..=17979u32 => BlockSmoker::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            17980u32..=17987u32 => BlockBlastFurnace::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            17988u32 => BlockCartographyTable::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            17989u32 => BlockFletchingTable::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            17990u32..=18001u32 => BlockGrindstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18002u32..=18017u32 => BlockLectern::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18018u32 => BlockSmithingTable::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18019u32..=18022u32 => BlockStonecutter::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18023u32..=18054u32 => BlockBell::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18055u32..=18058u32 => BlockLantern::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18059u32..=18062u32 => BlockSoulLantern::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18063u32..=18094u32 => BlockCampfire::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18095u32..=18126u32 => BlockSoulCampfire::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18127u32..=18130u32 => BlockSweetBerryBush::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18131u32 => BlockWarpedStem::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18132u32 => BlockStrippedWarpedStem::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18133u32..=18135u32 => BlockWarpedHyphae::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18136u32..=18138u32 => BlockStrippedWarpedHyphae::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18139u32 => BlockWarpedNylium::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18140u32 => BlockWarpedFungus::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18141u32 => BlockWarpedWartBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18142u32 => BlockWarpedRoots::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18143u32 => BlockNetherSprouts::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18144u32 => BlockCrimsonStem::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18145u32 => BlockStrippedCrimsonStem::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18146u32..=18148u32 => BlockCrimsonHyphae::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18149u32..=18151u32 => BlockStrippedCrimsonHyphae::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18152u32 => BlockCrimsonNylium::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18153u32 => BlockCrimsonFungus::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18154u32 => BlockShroomlight::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18155u32..=18180u32 => BlockWeepingVines::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18181u32 => BlockWeepingVinesPlant::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18182u32..=18207u32 => BlockTwistingVines::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18208u32 => BlockTwistingVinesPlant::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18209u32 => BlockCrimsonRoots::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18210u32 => BlockCrimsonPlanks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18211u32 => BlockWarpedPlanks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18212u32..=18217u32 => BlockCrimsonSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18218u32..=18223u32 => BlockWarpedSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18224u32..=18225u32 => BlockCrimsonPressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18226u32..=18227u32 => BlockWarpedPressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18228u32..=18259u32 => BlockCrimsonFence::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18260u32..=18291u32 => BlockWarpedFence::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18292u32..=18355u32 => BlockCrimsonTrapdoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18356u32..=18419u32 => BlockWarpedTrapdoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18420u32..=18451u32 => BlockCrimsonFenceGate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18452u32..=18483u32 => BlockWarpedFenceGate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18484u32..=18563u32 => BlockCrimsonStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18564u32..=18643u32 => BlockWarpedStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18644u32..=18667u32 => BlockCrimsonButton::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18668u32..=18691u32 => BlockWarpedButton::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18692u32..=18755u32 => BlockCrimsonDoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18756u32..=18819u32 => BlockWarpedDoor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18820u32..=18851u32 => BlockCrimsonSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18852u32..=18883u32 => BlockWarpedSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18884u32..=18891u32 => BlockCrimsonWallSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18892u32..=18899u32 => BlockWarpedWallSign::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18900u32..=18903u32 => BlockStructureBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18904u32..=18915u32 => BlockJigsaw::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18916u32..=18924u32 => BlockComposter::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18925u32..=18940u32 => BlockTarget::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18941u32..=18964u32 => BlockBeeNest::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18965u32..=18988u32 => BlockBeehive::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18989u32 => BlockHoneyBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18990u32 => BlockHoneycombBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18991u32 => BlockNetheriteBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18992u32 => BlockAncientDebris::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18993u32 => BlockCryingObsidian::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18994u32..=18998u32 => BlockRespawnAnchor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            18999u32 => BlockPottedCrimsonFungus::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19000u32 => BlockPottedWarpedFungus::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19001u32 => BlockPottedCrimsonRoots::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19002u32 => BlockPottedWarpedRoots::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19003u32 => BlockLodestone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19004u32 => BlockBlackstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19005u32..=19084u32 => BlockBlackstoneStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19085u32..=19408u32 => BlockBlackstoneWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19409u32..=19414u32 => BlockBlackstoneSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19415u32 => BlockPolishedBlackstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19416u32 => BlockPolishedBlackstoneBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19417u32 => BlockCrackedPolishedBlackstoneBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19418u32 => BlockChiseledPolishedBlackstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19419u32..=19424u32 => BlockPolishedBlackstoneBrickSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19425u32..=19504u32 => BlockPolishedBlackstoneBrickStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19505u32..=19828u32 => BlockPolishedBlackstoneBrickWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19829u32 => BlockGildedBlackstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19830u32..=19909u32 => BlockPolishedBlackstoneStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19910u32..=19915u32 => BlockPolishedBlackstoneSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19916u32..=19917u32 => BlockPolishedBlackstonePressurePlate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19918u32..=19941u32 => BlockPolishedBlackstoneButton::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            19942u32..=20265u32 => BlockPolishedBlackstoneWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20266u32 => BlockChiseledNetherBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20267u32 => BlockCrackedNetherBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20268u32 => BlockQuartzBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20269u32..=20284u32 => BlockCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20285u32..=20300u32 => BlockWhiteCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20301u32..=20316u32 => BlockOrangeCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20317u32..=20332u32 => BlockMagentaCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20333u32..=20348u32 => BlockLightBlueCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20349u32..=20364u32 => BlockYellowCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20365u32..=20380u32 => BlockLimeCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20381u32..=20396u32 => BlockPinkCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20397u32..=20412u32 => BlockGrayCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20413u32..=20428u32 => BlockLightGrayCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20429u32..=20444u32 => BlockCyanCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20445u32..=20460u32 => BlockPurpleCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20461u32..=20476u32 => BlockBlueCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20477u32..=20492u32 => BlockBrownCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20493u32..=20508u32 => BlockGreenCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20509u32..=20524u32 => BlockRedCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20525u32..=20540u32 => BlockBlackCandle::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20541u32..=20542u32 => BlockCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20543u32..=20544u32 => BlockWhiteCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20545u32..=20546u32 => BlockOrangeCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20547u32..=20548u32 => BlockMagentaCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20549u32..=20550u32 => BlockLightBlueCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20551u32..=20552u32 => BlockYellowCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20553u32..=20554u32 => BlockLimeCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20555u32..=20556u32 => BlockPinkCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20557u32..=20558u32 => BlockGrayCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20559u32..=20560u32 => BlockLightGrayCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20561u32..=20562u32 => BlockCyanCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20563u32..=20564u32 => BlockPurpleCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20565u32..=20566u32 => BlockBlueCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20567u32..=20568u32 => BlockBrownCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20569u32..=20570u32 => BlockGreenCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20571u32..=20572u32 => BlockRedCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20573u32..=20574u32 => BlockBlackCandleCake::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20575u32 => BlockAmethystBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20576u32 => BlockBuddingAmethyst::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20577u32..=20588u32 => BlockAmethystCluster::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20589u32..=20600u32 => BlockLargeAmethystBud::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20601u32..=20612u32 => BlockMediumAmethystBud::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20613u32..=20624u32 => BlockSmallAmethystBud::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20625u32 => BlockTuff::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20626u32 => BlockCalcite::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20627u32 => BlockTintedGlass::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20628u32 => BlockPowderSnow::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20629u32..=20724u32 => BlockSculkSensor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            20725u32..=21108u32 => BlockCalibratedSculkSensor::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21109u32 => BlockSculk::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21110u32..=21111u32 => BlockSculkVein::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21112u32..=21113u32 => BlockSculkCatalyst::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21114u32..=21121u32 => BlockSculkShrieker::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21122u32 => BlockOxidizedCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21123u32 => BlockWeatheredCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21124u32 => BlockExposedCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21125u32 => BlockCopperBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21126u32 => BlockCopperOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21127u32 => BlockDeepslateCopperOre::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21128u32 => BlockOxidizedCutCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21129u32 => BlockWeatheredCutCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21130u32 => BlockExposedCutCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21131u32 => BlockCutCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21132u32..=21211u32 => BlockOxidizedCutCopperStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21212u32..=21291u32 => BlockWeatheredCutCopperStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21292u32..=21371u32 => BlockExposedCutCopperStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21372u32..=21451u32 => BlockCutCopperStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21452u32..=21457u32 => BlockOxidizedCutCopperSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21458u32..=21463u32 => BlockWeatheredCutCopperSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21464u32..=21469u32 => BlockExposedCutCopperSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21470u32..=21475u32 => BlockCutCopperSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21476u32 => BlockWaxedCopperBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21477u32 => BlockWaxedWeatheredCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21478u32 => BlockWaxedExposedCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21479u32 => BlockWaxedOxidizedCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21480u32 => BlockWaxedOxidizedCutCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21481u32 => BlockWaxedWeatheredCutCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21482u32 => BlockWaxedExposedCutCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21483u32 => BlockWaxedCutCopper::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21484u32..=21563u32 => BlockWaxedOxidizedCutCopperStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21564u32..=21643u32 => BlockWaxedWeatheredCutCopperStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21644u32..=21723u32 => BlockWaxedExposedCutCopperStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21724u32..=21803u32 => BlockWaxedCutCopperStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21804u32..=21809u32 => BlockWaxedOxidizedCutCopperSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21810u32..=21815u32 => BlockWaxedWeatheredCutCopperSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21816u32..=21821u32 => BlockWaxedExposedCutCopperSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21822u32..=21827u32 => BlockWaxedCutCopperSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21828u32..=21851u32 => BlockLightningRod::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21852u32..=21871u32 => BlockPointedDripstone::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21872u32 => BlockDripstoneBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21873u32..=21924u32 => BlockCaveVines::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21925u32..=21926u32 => BlockCaveVinesPlant::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21927u32 => BlockSporeBlossom::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21928u32 => BlockAzalea::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21929u32 => BlockFloweringAzalea::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21930u32 => BlockMossCarpet::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21931u32..=21946u32 => BlockPinkPetals::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21947u32 => BlockMossBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21948u32..=21979u32 => BlockBigDripleaf::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21980u32..=21987u32 => BlockBigDripleafStem::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            21988u32..=22003u32 => BlockSmallDripleaf::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22004u32..=22005u32 => BlockHangingRoots::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22006u32 => BlockRootedDirt::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22007u32 => BlockMud::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22008u32..=22010u32 => BlockDeepslate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22011u32 => BlockCobbledDeepslate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22012u32..=22091u32 => BlockCobbledDeepslateStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22092u32..=22097u32 => BlockCobbledDeepslateSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22098u32..=22421u32 => BlockCobbledDeepslateWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22422u32 => BlockPolishedDeepslate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22423u32..=22502u32 => BlockPolishedDeepslateStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22503u32..=22508u32 => BlockPolishedDeepslateSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22509u32..=22832u32 => BlockPolishedDeepslateWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22833u32 => BlockDeepslateTiles::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22834u32..=22913u32 => BlockDeepslateTileStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22914u32..=22919u32 => BlockDeepslateTileSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            22920u32..=23243u32 => BlockDeepslateTileWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23244u32 => BlockDeepslateBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23245u32..=23324u32 => BlockDeepslateBrickStairs::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23325u32..=23330u32 => BlockDeepslateBrickSlab::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23331u32..=23654u32 => BlockDeepslateBrickWall::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23655u32 => BlockChiseledDeepslate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23656u32 => BlockCrackedDeepslateBricks::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23657u32 => BlockCrackedDeepslateTiles::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23658u32 => BlockInfestedDeepslate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23659u32 => BlockSmoothBasalt::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23660u32 => BlockRawIronBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23661u32 => BlockRawCopperBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23662u32 => BlockRawGoldBlock::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23663u32 => BlockPottedAzaleaBush::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23664u32 => BlockPottedFloweringAzaleaBush::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23665u32..=23667u32 => BlockOchreFroglight::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23668u32..=23670u32 => BlockVerdantFroglight::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23671u32..=23673u32 => BlockPearlescentFroglight::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23674u32 => BlockFrogspawn::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23675u32 => BlockReinforcedDeepslate::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            23676u32..=23691u32 => BlockDecoratedPot::try_from_u32(id)
                .map(Blocks::from)
                .unwrap_or(Self::Error(BlockError)),
            _ => Self::Error(BlockError),
        }
    }
}

impl BlockTrait<V1_20_0> for BlockError {
    fn resource_location(&self) -> &'static str { "mc-rs:error" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { u32::MAX }
}

impl BlockTrait<V1_20_0> for BlockAir {
    fn resource_location(&self) -> &'static str { "minecraft:air" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 0u32 }
}

impl BlockTrait<V1_20_0> for BlockStone {
    fn resource_location(&self) -> &'static str { "minecraft:stone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 1u32 }
}

impl BlockTrait<V1_20_0> for BlockGranite {
    fn resource_location(&self) -> &'static str { "minecraft:granite" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedGranite {
    fn resource_location(&self) -> &'static str { "minecraft:polished_granite" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 3u32 }
}

impl BlockTrait<V1_20_0> for BlockDiorite {
    fn resource_location(&self) -> &'static str { "minecraft:diorite" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 4u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedDiorite {
    fn resource_location(&self) -> &'static str { "minecraft:polished_diorite" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5u32 }
}

impl BlockTrait<V1_20_0> for BlockAndesite {
    fn resource_location(&self) -> &'static str { "minecraft:andesite" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedAndesite {
    fn resource_location(&self) -> &'static str { "minecraft:polished_andesite" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 7u32 }
}

impl BlockTrait<V1_20_0> for BlockGrassBlock {
    fn resource_location(&self) -> &'static str { "minecraft:grass_block" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8u32 }
}

impl BlockTrait<V1_20_0> for BlockDirt {
    fn resource_location(&self) -> &'static str { "minecraft:dirt" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10u32 }
}

impl BlockTrait<V1_20_0> for BlockCoarseDirt {
    fn resource_location(&self) -> &'static str { "minecraft:coarse_dirt" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 11u32 }
}

impl BlockTrait<V1_20_0> for BlockPodzol {
    fn resource_location(&self) -> &'static str { "minecraft:podzol" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12u32 }
}

impl BlockTrait<V1_20_0> for BlockCobblestone {
    fn resource_location(&self) -> &'static str { "minecraft:cobblestone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 14u32 }
}

impl BlockTrait<V1_20_0> for BlockOakPlanks {
    fn resource_location(&self) -> &'static str { "minecraft:oak_planks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 15u32 }
}

impl BlockTrait<V1_20_0> for BlockSprucePlanks {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_planks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 16u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchPlanks {
    fn resource_location(&self) -> &'static str { "minecraft:birch_planks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 17u32 }
}

impl BlockTrait<V1_20_0> for BlockJunglePlanks {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_planks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaPlanks {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_planks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 19u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryPlanks {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_planks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 20u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakPlanks {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_planks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21u32 }
}

impl BlockTrait<V1_20_0> for BlockMangrovePlanks {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_planks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 22u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooPlanks {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_planks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooMosaic {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_mosaic" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 24u32 }
}

impl BlockTrait<V1_20_0> for BlockOakSapling {
    fn resource_location(&self) -> &'static str { "minecraft:oak_sapling" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 25u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceSapling {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_sapling" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 27u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchSapling {
    fn resource_location(&self) -> &'static str { "minecraft:birch_sapling" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 29u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleSapling {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_sapling" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 31u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaSapling {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_sapling" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 33u32 }
}

impl BlockTrait<V1_20_0> for BlockCherrySapling {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_sapling" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 35u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakSapling {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_sapling" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 37u32 }
}

impl BlockTrait<V1_20_0> for BlockMangrovePropagule {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_propagule" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 39u32 }
}

impl BlockTrait<V1_20_0> for BlockBedrock {
    fn resource_location(&self) -> &'static str { "minecraft:bedrock" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 79u32 }
}

impl BlockTrait<V1_20_0> for BlockWater {
    fn resource_location(&self) -> &'static str { "minecraft:water" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 80u32 }
}

impl BlockTrait<V1_20_0> for BlockLava {
    fn resource_location(&self) -> &'static str { "minecraft:lava" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 96u32 }
}

impl BlockTrait<V1_20_0> for BlockSand {
    fn resource_location(&self) -> &'static str { "minecraft:sand" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 112u32 }
}

impl BlockTrait<V1_20_0> for BlockSuspiciousSand {
    fn resource_location(&self) -> &'static str { "minecraft:suspicious_sand" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 113u32 }
}

impl BlockTrait<V1_20_0> for BlockRedSand {
    fn resource_location(&self) -> &'static str { "minecraft:red_sand" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 117u32 }
}

impl BlockTrait<V1_20_0> for BlockGravel {
    fn resource_location(&self) -> &'static str { "minecraft:gravel" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 118u32 }
}

impl BlockTrait<V1_20_0> for BlockSuspiciousGravel {
    fn resource_location(&self) -> &'static str { "minecraft:suspicious_gravel" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 119u32 }
}

impl BlockTrait<V1_20_0> for BlockGoldOre {
    fn resource_location(&self) -> &'static str { "minecraft:gold_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 123u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateGoldOre {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_gold_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 124u32 }
}

impl BlockTrait<V1_20_0> for BlockIronOre {
    fn resource_location(&self) -> &'static str { "minecraft:iron_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 125u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateIronOre {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_iron_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 126u32 }
}

impl BlockTrait<V1_20_0> for BlockCoalOre {
    fn resource_location(&self) -> &'static str { "minecraft:coal_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 127u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateCoalOre {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_coal_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 128u32 }
}

impl BlockTrait<V1_20_0> for BlockNetherGoldOre {
    fn resource_location(&self) -> &'static str { "minecraft:nether_gold_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 129u32 }
}

impl BlockTrait<V1_20_0> for BlockOakLog {
    fn resource_location(&self) -> &'static str { "minecraft:oak_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 130u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceLog {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 133u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchLog {
    fn resource_location(&self) -> &'static str { "minecraft:birch_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 136u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleLog {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 139u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaLog {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 142u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryLog {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 145u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakLog {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 148u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveLog {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 151u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveRoots {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_roots" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 154u32 }
}

impl BlockTrait<V1_20_0> for BlockMuddyMangroveRoots {
    fn resource_location(&self) -> &'static str { "minecraft:muddy_mangrove_roots" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 156u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooBlock {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_block" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 159u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedSpruceLog {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_spruce_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 162u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedBirchLog {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_birch_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 165u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedJungleLog {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_jungle_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 168u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedAcaciaLog {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_acacia_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 171u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedCherryLog {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_cherry_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 174u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedDarkOakLog {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_dark_oak_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 177u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedOakLog {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_oak_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 180u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedMangroveLog {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_mangrove_log" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 183u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedBambooBlock {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_bamboo_block" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 186u32 }
}

impl BlockTrait<V1_20_0> for BlockOakWood {
    fn resource_location(&self) -> &'static str { "minecraft:oak_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 189u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceWood {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 192u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchWood {
    fn resource_location(&self) -> &'static str { "minecraft:birch_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 195u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleWood {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 198u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaWood {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 201u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryWood {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 204u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakWood {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 207u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveWood {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 210u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedOakWood {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_oak_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 213u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedSpruceWood {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_spruce_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 216u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedBirchWood {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_birch_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 219u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedJungleWood {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_jungle_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 222u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedAcaciaWood {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_acacia_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 225u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedCherryWood {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_cherry_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 228u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedDarkOakWood {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_dark_oak_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 231u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedMangroveWood {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_mangrove_wood" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 234u32 }
}

impl BlockTrait<V1_20_0> for BlockOakLeaves {
    fn resource_location(&self) -> &'static str { "minecraft:oak_leaves" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 237u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceLeaves {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_leaves" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 265u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchLeaves {
    fn resource_location(&self) -> &'static str { "minecraft:birch_leaves" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 293u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleLeaves {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_leaves" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 321u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaLeaves {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_leaves" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 349u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryLeaves {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_leaves" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 377u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakLeaves {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_leaves" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 405u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveLeaves {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_leaves" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 433u32 }
}

impl BlockTrait<V1_20_0> for BlockAzaleaLeaves {
    fn resource_location(&self) -> &'static str { "minecraft:azalea_leaves" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 461u32 }
}

impl BlockTrait<V1_20_0> for BlockFloweringAzaleaLeaves {
    fn resource_location(&self) -> &'static str { "minecraft:flowering_azalea_leaves" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 489u32 }
}

impl BlockTrait<V1_20_0> for BlockSponge {
    fn resource_location(&self) -> &'static str { "minecraft:sponge" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 517u32 }
}

impl BlockTrait<V1_20_0> for BlockWetSponge {
    fn resource_location(&self) -> &'static str { "minecraft:wet_sponge" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 518u32 }
}

impl BlockTrait<V1_20_0> for BlockGlass {
    fn resource_location(&self) -> &'static str { "minecraft:glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 519u32 }
}

impl BlockTrait<V1_20_0> for BlockLapisOre {
    fn resource_location(&self) -> &'static str { "minecraft:lapis_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 520u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateLapisOre {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_lapis_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 521u32 }
}

impl BlockTrait<V1_20_0> for BlockLapisBlock {
    fn resource_location(&self) -> &'static str { "minecraft:lapis_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 522u32 }
}

impl BlockTrait<V1_20_0> for BlockDispenser {
    fn resource_location(&self) -> &'static str { "minecraft:dispenser" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 523u32 }
}

impl BlockTrait<V1_20_0> for BlockSandstone {
    fn resource_location(&self) -> &'static str { "minecraft:sandstone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 535u32 }
}

impl BlockTrait<V1_20_0> for BlockChiseledSandstone {
    fn resource_location(&self) -> &'static str { "minecraft:chiseled_sandstone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 536u32 }
}

impl BlockTrait<V1_20_0> for BlockCutSandstone {
    fn resource_location(&self) -> &'static str { "minecraft:cut_sandstone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 537u32 }
}

impl BlockTrait<V1_20_0> for BlockNoteBlock {
    fn resource_location(&self) -> &'static str { "minecraft:note_block" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 538u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteBed {
    fn resource_location(&self) -> &'static str { "minecraft:white_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1688u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeBed {
    fn resource_location(&self) -> &'static str { "minecraft:orange_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1704u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaBed {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1720u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueBed {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1736u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowBed {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1752u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeBed {
    fn resource_location(&self) -> &'static str { "minecraft:lime_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1768u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkBed {
    fn resource_location(&self) -> &'static str { "minecraft:pink_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1784u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayBed {
    fn resource_location(&self) -> &'static str { "minecraft:gray_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1800u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayBed {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1816u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanBed {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1832u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleBed {
    fn resource_location(&self) -> &'static str { "minecraft:purple_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1848u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueBed {
    fn resource_location(&self) -> &'static str { "minecraft:blue_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1864u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownBed {
    fn resource_location(&self) -> &'static str { "minecraft:brown_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1880u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenBed {
    fn resource_location(&self) -> &'static str { "minecraft:green_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1896u32 }
}

impl BlockTrait<V1_20_0> for BlockRedBed {
    fn resource_location(&self) -> &'static str { "minecraft:red_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1912u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackBed {
    fn resource_location(&self) -> &'static str { "minecraft:black_bed" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1928u32 }
}

impl BlockTrait<V1_20_0> for BlockPoweredRail {
    fn resource_location(&self) -> &'static str { "minecraft:powered_rail" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1944u32 }
}

impl BlockTrait<V1_20_0> for BlockDetectorRail {
    fn resource_location(&self) -> &'static str { "minecraft:detector_rail" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 1984u32 }
}

impl BlockTrait<V1_20_0> for BlockStickyPiston {
    fn resource_location(&self) -> &'static str { "minecraft:sticky_piston" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 2024u32 }
}

impl BlockTrait<V1_20_0> for BlockCobweb {
    fn resource_location(&self) -> &'static str { "minecraft:cobweb" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2036u32 }
}

impl BlockTrait<V1_20_0> for BlockGrass {
    fn resource_location(&self) -> &'static str { "minecraft:grass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2037u32 }
}

impl BlockTrait<V1_20_0> for BlockFern {
    fn resource_location(&self) -> &'static str { "minecraft:fern" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2038u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadBush {
    fn resource_location(&self) -> &'static str { "minecraft:dead_bush" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2039u32 }
}

impl BlockTrait<V1_20_0> for BlockSeagrass {
    fn resource_location(&self) -> &'static str { "minecraft:seagrass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2040u32 }
}

impl BlockTrait<V1_20_0> for BlockTallSeagrass {
    fn resource_location(&self) -> &'static str { "minecraft:tall_seagrass" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 2041u32 }
}

impl BlockTrait<V1_20_0> for BlockPiston {
    fn resource_location(&self) -> &'static str { "minecraft:piston" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 2043u32 }
}

impl BlockTrait<V1_20_0> for BlockPistonHead {
    fn resource_location(&self) -> &'static str { "minecraft:piston_head" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 2055u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteWool {
    fn resource_location(&self) -> &'static str { "minecraft:white_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2079u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeWool {
    fn resource_location(&self) -> &'static str { "minecraft:orange_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2080u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaWool {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2081u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueWool {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2082u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowWool {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2083u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeWool {
    fn resource_location(&self) -> &'static str { "minecraft:lime_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2084u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkWool {
    fn resource_location(&self) -> &'static str { "minecraft:pink_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2085u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayWool {
    fn resource_location(&self) -> &'static str { "minecraft:gray_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2086u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayWool {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2087u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanWool {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2088u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleWool {
    fn resource_location(&self) -> &'static str { "minecraft:purple_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2089u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueWool {
    fn resource_location(&self) -> &'static str { "minecraft:blue_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2090u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownWool {
    fn resource_location(&self) -> &'static str { "minecraft:brown_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2091u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenWool {
    fn resource_location(&self) -> &'static str { "minecraft:green_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2092u32 }
}

impl BlockTrait<V1_20_0> for BlockRedWool {
    fn resource_location(&self) -> &'static str { "minecraft:red_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2093u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackWool {
    fn resource_location(&self) -> &'static str { "minecraft:black_wool" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2094u32 }
}

impl BlockTrait<V1_20_0> for BlockMovingPiston {
    fn resource_location(&self) -> &'static str { "minecraft:moving_piston" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 2095u32 }
}

impl BlockTrait<V1_20_0> for BlockDandelion {
    fn resource_location(&self) -> &'static str { "minecraft:dandelion" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2097u32 }
}

impl BlockTrait<V1_20_0> for BlockTorchflower {
    fn resource_location(&self) -> &'static str { "minecraft:torchflower" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2098u32 }
}

impl BlockTrait<V1_20_0> for BlockPoppy {
    fn resource_location(&self) -> &'static str { "minecraft:poppy" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2099u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueOrchid {
    fn resource_location(&self) -> &'static str { "minecraft:blue_orchid" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2100u32 }
}

impl BlockTrait<V1_20_0> for BlockAllium {
    fn resource_location(&self) -> &'static str { "minecraft:allium" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2101u32 }
}

impl BlockTrait<V1_20_0> for BlockAzureBluet {
    fn resource_location(&self) -> &'static str { "minecraft:azure_bluet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2102u32 }
}

impl BlockTrait<V1_20_0> for BlockRedTulip {
    fn resource_location(&self) -> &'static str { "minecraft:red_tulip" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2103u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeTulip {
    fn resource_location(&self) -> &'static str { "minecraft:orange_tulip" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2104u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteTulip {
    fn resource_location(&self) -> &'static str { "minecraft:white_tulip" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2105u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkTulip {
    fn resource_location(&self) -> &'static str { "minecraft:pink_tulip" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2106u32 }
}

impl BlockTrait<V1_20_0> for BlockOxeyeDaisy {
    fn resource_location(&self) -> &'static str { "minecraft:oxeye_daisy" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2107u32 }
}

impl BlockTrait<V1_20_0> for BlockCornflower {
    fn resource_location(&self) -> &'static str { "minecraft:cornflower" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2108u32 }
}

impl BlockTrait<V1_20_0> for BlockWitherRose {
    fn resource_location(&self) -> &'static str { "minecraft:wither_rose" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2109u32 }
}

impl BlockTrait<V1_20_0> for BlockLilyOfTheValley {
    fn resource_location(&self) -> &'static str { "minecraft:lily_of_the_valley" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2110u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownMushroom {
    fn resource_location(&self) -> &'static str { "minecraft:brown_mushroom" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2111u32 }
}

impl BlockTrait<V1_20_0> for BlockRedMushroom {
    fn resource_location(&self) -> &'static str { "minecraft:red_mushroom" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2112u32 }
}

impl BlockTrait<V1_20_0> for BlockGoldBlock {
    fn resource_location(&self) -> &'static str { "minecraft:gold_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2113u32 }
}

impl BlockTrait<V1_20_0> for BlockIronBlock {
    fn resource_location(&self) -> &'static str { "minecraft:iron_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2114u32 }
}

impl BlockTrait<V1_20_0> for BlockBricks {
    fn resource_location(&self) -> &'static str { "minecraft:bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2115u32 }
}

impl BlockTrait<V1_20_0> for BlockTnt {
    fn resource_location(&self) -> &'static str { "minecraft:tnt" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 2116u32 }
}

impl BlockTrait<V1_20_0> for BlockBookshelf {
    fn resource_location(&self) -> &'static str { "minecraft:bookshelf" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2118u32 }
}

impl BlockTrait<V1_20_0> for BlockChiseledBookshelf {
    fn resource_location(&self) -> &'static str { "minecraft:chiseled_bookshelf" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2119u32 }
}

impl BlockTrait<V1_20_0> for BlockMossyCobblestone {
    fn resource_location(&self) -> &'static str { "minecraft:mossy_cobblestone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2120u32 }
}

impl BlockTrait<V1_20_0> for BlockObsidian {
    fn resource_location(&self) -> &'static str { "minecraft:obsidian" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2121u32 }
}

impl BlockTrait<V1_20_0> for BlockTorch {
    fn resource_location(&self) -> &'static str { "minecraft:torch" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2122u32 }
}

impl BlockTrait<V1_20_0> for BlockWallTorch {
    fn resource_location(&self) -> &'static str { "minecraft:wall_torch" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 2123u32 }
}

impl BlockTrait<V1_20_0> for BlockFire {
    fn resource_location(&self) -> &'static str { "minecraft:fire" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 2127u32 }
}

impl BlockTrait<V1_20_0> for BlockSoulFire {
    fn resource_location(&self) -> &'static str { "minecraft:soul_fire" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2639u32 }
}

impl BlockTrait<V1_20_0> for BlockSpawner {
    fn resource_location(&self) -> &'static str { "minecraft:spawner" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 2640u32 }
}

impl BlockTrait<V1_20_0> for BlockOakStairs {
    fn resource_location(&self) -> &'static str { "minecraft:oak_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 2641u32 }
}

impl BlockTrait<V1_20_0> for BlockChest {
    fn resource_location(&self) -> &'static str { "minecraft:chest" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 2721u32 }
}

impl BlockTrait<V1_20_0> for BlockRedstoneWire {
    fn resource_location(&self) -> &'static str { "minecraft:redstone_wire" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 2745u32 }
}

impl BlockTrait<V1_20_0> for BlockDiamondOre {
    fn resource_location(&self) -> &'static str { "minecraft:diamond_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 4041u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateDiamondOre {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_diamond_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 4042u32 }
}

impl BlockTrait<V1_20_0> for BlockDiamondBlock {
    fn resource_location(&self) -> &'static str { "minecraft:diamond_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 4043u32 }
}

impl BlockTrait<V1_20_0> for BlockCraftingTable {
    fn resource_location(&self) -> &'static str { "minecraft:crafting_table" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 4044u32 }
}

impl BlockTrait<V1_20_0> for BlockWheat {
    fn resource_location(&self) -> &'static str { "minecraft:wheat" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4045u32 }
}

impl BlockTrait<V1_20_0> for BlockFarmland {
    fn resource_location(&self) -> &'static str { "minecraft:farmland" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4053u32 }
}

impl BlockTrait<V1_20_0> for BlockFurnace {
    fn resource_location(&self) -> &'static str { "minecraft:furnace" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4061u32 }
}

impl BlockTrait<V1_20_0> for BlockOakSign {
    fn resource_location(&self) -> &'static str { "minecraft:oak_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4069u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceSign {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4101u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchSign {
    fn resource_location(&self) -> &'static str { "minecraft:birch_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4133u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaSign {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4165u32 }
}

impl BlockTrait<V1_20_0> for BlockCherrySign {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4197u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleSign {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4229u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakSign {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4261u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveSign {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4293u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooSign {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4325u32 }
}

impl BlockTrait<V1_20_0> for BlockOakDoor {
    fn resource_location(&self) -> &'static str { "minecraft:oak_door" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4357u32 }
}

impl BlockTrait<V1_20_0> for BlockLadder {
    fn resource_location(&self) -> &'static str { "minecraft:ladder" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4421u32 }
}

impl BlockTrait<V1_20_0> for BlockRail {
    fn resource_location(&self) -> &'static str { "minecraft:rail" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4429u32 }
}

impl BlockTrait<V1_20_0> for BlockCobblestoneStairs {
    fn resource_location(&self) -> &'static str { "minecraft:cobblestone_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4449u32 }
}

impl BlockTrait<V1_20_0> for BlockOakWallSign {
    fn resource_location(&self) -> &'static str { "minecraft:oak_wall_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4529u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceWallSign {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_wall_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4537u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchWallSign {
    fn resource_location(&self) -> &'static str { "minecraft:birch_wall_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4545u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaWallSign {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_wall_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4553u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryWallSign {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_wall_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4561u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleWallSign {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_wall_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4569u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakWallSign {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_wall_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4577u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveWallSign {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_wall_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4585u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooWallSign {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_wall_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4593u32 }
}

impl BlockTrait<V1_20_0> for BlockOakHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:oak_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4601u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4665u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:birch_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4729u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4793u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4857u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4921u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 4985u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5049u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:warped_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5113u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5177u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5241u32 }
}

impl BlockTrait<V1_20_0> for BlockOakWallHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:oak_wall_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5305u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceWallHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_wall_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5313u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchWallHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:birch_wall_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5321u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaWallHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_wall_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5329u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryWallHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_wall_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5337u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleWallHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_wall_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5345u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakWallHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_wall_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5353u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveWallHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_wall_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5361u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonWallHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_wall_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5369u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedWallHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:warped_wall_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5377u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooWallHangingSign {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_wall_hanging_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5385u32 }
}

impl BlockTrait<V1_20_0> for BlockLever {
    fn resource_location(&self) -> &'static str { "minecraft:lever" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5393u32 }
}

impl BlockTrait<V1_20_0> for BlockStonePressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:stone_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5417u32 }
}

impl BlockTrait<V1_20_0> for BlockIronDoor {
    fn resource_location(&self) -> &'static str { "minecraft:iron_door" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5419u32 }
}

impl BlockTrait<V1_20_0> for BlockOakPressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:oak_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5483u32 }
}

impl BlockTrait<V1_20_0> for BlockSprucePressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5485u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchPressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:birch_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5487u32 }
}

impl BlockTrait<V1_20_0> for BlockJunglePressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5489u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaPressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5491u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryPressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5493u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakPressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5495u32 }
}

impl BlockTrait<V1_20_0> for BlockMangrovePressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5497u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooPressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5499u32 }
}

impl BlockTrait<V1_20_0> for BlockRedstoneOre {
    fn resource_location(&self) -> &'static str { "minecraft:redstone_ore" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5501u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateRedstoneOre {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_redstone_ore" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5503u32 }
}

impl BlockTrait<V1_20_0> for BlockRedstoneTorch {
    fn resource_location(&self) -> &'static str { "minecraft:redstone_torch" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5505u32 }
}

impl BlockTrait<V1_20_0> for BlockRedstoneWallTorch {
    fn resource_location(&self) -> &'static str { "minecraft:redstone_wall_torch" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5507u32 }
}

impl BlockTrait<V1_20_0> for BlockStoneButton {
    fn resource_location(&self) -> &'static str { "minecraft:stone_button" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5515u32 }
}

impl BlockTrait<V1_20_0> for BlockSnow {
    fn resource_location(&self) -> &'static str { "minecraft:snow" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5539u32 }
}

impl BlockTrait<V1_20_0> for BlockIce {
    fn resource_location(&self) -> &'static str { "minecraft:ice" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5547u32 }
}

impl BlockTrait<V1_20_0> for BlockSnowBlock {
    fn resource_location(&self) -> &'static str { "minecraft:snow_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5548u32 }
}

impl BlockTrait<V1_20_0> for BlockCactus {
    fn resource_location(&self) -> &'static str { "minecraft:cactus" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5549u32 }
}

impl BlockTrait<V1_20_0> for BlockClay {
    fn resource_location(&self) -> &'static str { "minecraft:clay" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5565u32 }
}

impl BlockTrait<V1_20_0> for BlockSugarCane {
    fn resource_location(&self) -> &'static str { "minecraft:sugar_cane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5566u32 }
}

impl BlockTrait<V1_20_0> for BlockJukebox {
    fn resource_location(&self) -> &'static str { "minecraft:jukebox" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5582u32 }
}

impl BlockTrait<V1_20_0> for BlockOakFence {
    fn resource_location(&self) -> &'static str { "minecraft:oak_fence" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5584u32 }
}

impl BlockTrait<V1_20_0> for BlockPumpkin {
    fn resource_location(&self) -> &'static str { "minecraft:pumpkin" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5616u32 }
}

impl BlockTrait<V1_20_0> for BlockNetherrack {
    fn resource_location(&self) -> &'static str { "minecraft:netherrack" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5617u32 }
}

impl BlockTrait<V1_20_0> for BlockSoulSand {
    fn resource_location(&self) -> &'static str { "minecraft:soul_sand" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5618u32 }
}

impl BlockTrait<V1_20_0> for BlockSoulSoil {
    fn resource_location(&self) -> &'static str { "minecraft:soul_soil" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5619u32 }
}

impl BlockTrait<V1_20_0> for BlockBasalt {
    fn resource_location(&self) -> &'static str { "minecraft:basalt" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5620u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedBasalt {
    fn resource_location(&self) -> &'static str { "minecraft:polished_basalt" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5623u32 }
}

impl BlockTrait<V1_20_0> for BlockSoulTorch {
    fn resource_location(&self) -> &'static str { "minecraft:soul_torch" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5626u32 }
}

impl BlockTrait<V1_20_0> for BlockSoulWallTorch {
    fn resource_location(&self) -> &'static str { "minecraft:soul_wall_torch" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5627u32 }
}

impl BlockTrait<V1_20_0> for BlockGlowstone {
    fn resource_location(&self) -> &'static str { "minecraft:glowstone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5631u32 }
}

impl BlockTrait<V1_20_0> for BlockNetherPortal {
    fn resource_location(&self) -> &'static str { "minecraft:nether_portal" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5632u32 }
}

impl BlockTrait<V1_20_0> for BlockCarvedPumpkin {
    fn resource_location(&self) -> &'static str { "minecraft:carved_pumpkin" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5635u32 }
}

impl BlockTrait<V1_20_0> for BlockJackOLantern {
    fn resource_location(&self) -> &'static str { "minecraft:jack_o_lantern" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5639u32 }
}

impl BlockTrait<V1_20_0> for BlockCake {
    fn resource_location(&self) -> &'static str { "minecraft:cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5643u32 }
}

impl BlockTrait<V1_20_0> for BlockRepeater {
    fn resource_location(&self) -> &'static str { "minecraft:repeater" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5650u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:white_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5714u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:orange_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5715u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5716u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5717u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5718u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:lime_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5719u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:pink_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5720u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:gray_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5721u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5722u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5723u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:purple_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5724u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:blue_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5725u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:brown_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5726u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:green_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5727u32 }
}

impl BlockTrait<V1_20_0> for BlockRedStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:red_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5728u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackStainedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:black_stained_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 5729u32 }
}

impl BlockTrait<V1_20_0> for BlockOakTrapdoor {
    fn resource_location(&self) -> &'static str { "minecraft:oak_trapdoor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5730u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceTrapdoor {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_trapdoor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5794u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchTrapdoor {
    fn resource_location(&self) -> &'static str { "minecraft:birch_trapdoor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5858u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleTrapdoor {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_trapdoor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5922u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaTrapdoor {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_trapdoor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 5986u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryTrapdoor {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_trapdoor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6050u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakTrapdoor {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_trapdoor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6114u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveTrapdoor {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_trapdoor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6178u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooTrapdoor {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_trapdoor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6242u32 }
}

impl BlockTrait<V1_20_0> for BlockStoneBricks {
    fn resource_location(&self) -> &'static str { "minecraft:stone_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6306u32 }
}

impl BlockTrait<V1_20_0> for BlockMossyStoneBricks {
    fn resource_location(&self) -> &'static str { "minecraft:mossy_stone_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6307u32 }
}

impl BlockTrait<V1_20_0> for BlockCrackedStoneBricks {
    fn resource_location(&self) -> &'static str { "minecraft:cracked_stone_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6308u32 }
}

impl BlockTrait<V1_20_0> for BlockChiseledStoneBricks {
    fn resource_location(&self) -> &'static str { "minecraft:chiseled_stone_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6309u32 }
}

impl BlockTrait<V1_20_0> for BlockPackedMud {
    fn resource_location(&self) -> &'static str { "minecraft:packed_mud" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6310u32 }
}

impl BlockTrait<V1_20_0> for BlockMudBricks {
    fn resource_location(&self) -> &'static str { "minecraft:mud_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6311u32 }
}

impl BlockTrait<V1_20_0> for BlockInfestedStone {
    fn resource_location(&self) -> &'static str { "minecraft:infested_stone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6312u32 }
}

impl BlockTrait<V1_20_0> for BlockInfestedCobblestone {
    fn resource_location(&self) -> &'static str { "minecraft:infested_cobblestone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6313u32 }
}

impl BlockTrait<V1_20_0> for BlockInfestedStoneBricks {
    fn resource_location(&self) -> &'static str { "minecraft:infested_stone_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6314u32 }
}

impl BlockTrait<V1_20_0> for BlockInfestedMossyStoneBricks {
    fn resource_location(&self) -> &'static str { "minecraft:infested_mossy_stone_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6315u32 }
}

impl BlockTrait<V1_20_0> for BlockInfestedCrackedStoneBricks {
    fn resource_location(&self) -> &'static str { "minecraft:infested_cracked_stone_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6316u32 }
}

impl BlockTrait<V1_20_0> for BlockInfestedChiseledStoneBricks {
    fn resource_location(&self) -> &'static str { "minecraft:infested_chiseled_stone_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6317u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownMushroomBlock {
    fn resource_location(&self) -> &'static str { "minecraft:brown_mushroom_block" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6318u32 }
}

impl BlockTrait<V1_20_0> for BlockRedMushroomBlock {
    fn resource_location(&self) -> &'static str { "minecraft:red_mushroom_block" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6382u32 }
}

impl BlockTrait<V1_20_0> for BlockMushroomStem {
    fn resource_location(&self) -> &'static str { "minecraft:mushroom_stem" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6446u32 }
}

impl BlockTrait<V1_20_0> for BlockIronBars {
    fn resource_location(&self) -> &'static str { "minecraft:iron_bars" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6510u32 }
}

impl BlockTrait<V1_20_0> for BlockChain {
    fn resource_location(&self) -> &'static str { "minecraft:chain" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6542u32 }
}

impl BlockTrait<V1_20_0> for BlockGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6548u32 }
}

impl BlockTrait<V1_20_0> for BlockMelon {
    fn resource_location(&self) -> &'static str { "minecraft:melon" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6580u32 }
}

impl BlockTrait<V1_20_0> for BlockAttachedPumpkinStem {
    fn resource_location(&self) -> &'static str { "minecraft:attached_pumpkin_stem" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6581u32 }
}

impl BlockTrait<V1_20_0> for BlockAttachedMelonStem {
    fn resource_location(&self) -> &'static str { "minecraft:attached_melon_stem" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6585u32 }
}

impl BlockTrait<V1_20_0> for BlockPumpkinStem {
    fn resource_location(&self) -> &'static str { "minecraft:pumpkin_stem" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6589u32 }
}

impl BlockTrait<V1_20_0> for BlockMelonStem {
    fn resource_location(&self) -> &'static str { "minecraft:melon_stem" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6597u32 }
}

impl BlockTrait<V1_20_0> for BlockVine {
    fn resource_location(&self) -> &'static str { "minecraft:vine" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6605u32 }
}

impl BlockTrait<V1_20_0> for BlockGlowLichen {
    fn resource_location(&self) -> &'static str { "minecraft:glow_lichen" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6637u32 }
}

impl BlockTrait<V1_20_0> for BlockOakFenceGate {
    fn resource_location(&self) -> &'static str { "minecraft:oak_fence_gate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6639u32 }
}

impl BlockTrait<V1_20_0> for BlockBrickStairs {
    fn resource_location(&self) -> &'static str { "minecraft:brick_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6671u32 }
}

impl BlockTrait<V1_20_0> for BlockStoneBrickStairs {
    fn resource_location(&self) -> &'static str { "minecraft:stone_brick_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6751u32 }
}

impl BlockTrait<V1_20_0> for BlockMudBrickStairs {
    fn resource_location(&self) -> &'static str { "minecraft:mud_brick_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6831u32 }
}

impl BlockTrait<V1_20_0> for BlockMycelium {
    fn resource_location(&self) -> &'static str { "minecraft:mycelium" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6911u32 }
}

impl BlockTrait<V1_20_0> for BlockLilyPad {
    fn resource_location(&self) -> &'static str { "minecraft:lily_pad" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6913u32 }
}

impl BlockTrait<V1_20_0> for BlockNetherBricks {
    fn resource_location(&self) -> &'static str { "minecraft:nether_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 6914u32 }
}

impl BlockTrait<V1_20_0> for BlockNetherBrickFence {
    fn resource_location(&self) -> &'static str { "minecraft:nether_brick_fence" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6915u32 }
}

impl BlockTrait<V1_20_0> for BlockNetherBrickStairs {
    fn resource_location(&self) -> &'static str { "minecraft:nether_brick_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 6947u32 }
}

impl BlockTrait<V1_20_0> for BlockNetherWart {
    fn resource_location(&self) -> &'static str { "minecraft:nether_wart" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7027u32 }
}

impl BlockTrait<V1_20_0> for BlockEnchantingTable {
    fn resource_location(&self) -> &'static str { "minecraft:enchanting_table" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 7031u32 }
}

impl BlockTrait<V1_20_0> for BlockBrewingStand {
    fn resource_location(&self) -> &'static str { "minecraft:brewing_stand" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 7032u32 }
}

impl BlockTrait<V1_20_0> for BlockCauldron {
    fn resource_location(&self) -> &'static str { "minecraft:cauldron" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 7033u32 }
}

impl BlockTrait<V1_20_0> for BlockWaterCauldron {
    fn resource_location(&self) -> &'static str { "minecraft:water_cauldron" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7034u32 }
}

impl BlockTrait<V1_20_0> for BlockLavaCauldron {
    fn resource_location(&self) -> &'static str { "minecraft:lava_cauldron" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 7037u32 }
}

impl BlockTrait<V1_20_0> for BlockPowderSnowCauldron {
    fn resource_location(&self) -> &'static str { "minecraft:powder_snow_cauldron" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7038u32 }
}

impl BlockTrait<V1_20_0> for BlockEndPortal {
    fn resource_location(&self) -> &'static str { "minecraft:end_portal" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 7041u32 }
}

impl BlockTrait<V1_20_0> for BlockEndPortalFrame {
    fn resource_location(&self) -> &'static str { "minecraft:end_portal_frame" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7042u32 }
}

impl BlockTrait<V1_20_0> for BlockEndStone {
    fn resource_location(&self) -> &'static str { "minecraft:end_stone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 7050u32 }
}

impl BlockTrait<V1_20_0> for BlockDragonEgg {
    fn resource_location(&self) -> &'static str { "minecraft:dragon_egg" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 7051u32 }
}

impl BlockTrait<V1_20_0> for BlockRedstoneLamp {
    fn resource_location(&self) -> &'static str { "minecraft:redstone_lamp" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7052u32 }
}

impl BlockTrait<V1_20_0> for BlockCocoa {
    fn resource_location(&self) -> &'static str { "minecraft:cocoa" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7054u32 }
}

impl BlockTrait<V1_20_0> for BlockSandstoneStairs {
    fn resource_location(&self) -> &'static str { "minecraft:sandstone_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7066u32 }
}

impl BlockTrait<V1_20_0> for BlockEmeraldOre {
    fn resource_location(&self) -> &'static str { "minecraft:emerald_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 7146u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateEmeraldOre {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_emerald_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 7147u32 }
}

impl BlockTrait<V1_20_0> for BlockEnderChest {
    fn resource_location(&self) -> &'static str { "minecraft:ender_chest" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7148u32 }
}

impl BlockTrait<V1_20_0> for BlockTripwireHook {
    fn resource_location(&self) -> &'static str { "minecraft:tripwire_hook" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7156u32 }
}

impl BlockTrait<V1_20_0> for BlockTripwire {
    fn resource_location(&self) -> &'static str { "minecraft:tripwire" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7172u32 }
}

impl BlockTrait<V1_20_0> for BlockEmeraldBlock {
    fn resource_location(&self) -> &'static str { "minecraft:emerald_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 7300u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceStairs {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7301u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchStairs {
    fn resource_location(&self) -> &'static str { "minecraft:birch_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7381u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleStairs {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7461u32 }
}

impl BlockTrait<V1_20_0> for BlockCommandBlock {
    fn resource_location(&self) -> &'static str { "minecraft:command_block" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7541u32 }
}

impl BlockTrait<V1_20_0> for BlockBeacon {
    fn resource_location(&self) -> &'static str { "minecraft:beacon" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 7553u32 }
}

impl BlockTrait<V1_20_0> for BlockCobblestoneWall {
    fn resource_location(&self) -> &'static str { "minecraft:cobblestone_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7554u32 }
}

impl BlockTrait<V1_20_0> for BlockMossyCobblestoneWall {
    fn resource_location(&self) -> &'static str { "minecraft:mossy_cobblestone_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 7878u32 }
}

impl BlockTrait<V1_20_0> for BlockFlowerPot {
    fn resource_location(&self) -> &'static str { "minecraft:flower_pot" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8202u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedTorchflower {
    fn resource_location(&self) -> &'static str { "minecraft:potted_torchflower" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8203u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedOakSapling {
    fn resource_location(&self) -> &'static str { "minecraft:potted_oak_sapling" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8204u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedSpruceSapling {
    fn resource_location(&self) -> &'static str { "minecraft:potted_spruce_sapling" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8205u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedBirchSapling {
    fn resource_location(&self) -> &'static str { "minecraft:potted_birch_sapling" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8206u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedJungleSapling {
    fn resource_location(&self) -> &'static str { "minecraft:potted_jungle_sapling" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8207u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedAcaciaSapling {
    fn resource_location(&self) -> &'static str { "minecraft:potted_acacia_sapling" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8208u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedCherrySapling {
    fn resource_location(&self) -> &'static str { "minecraft:potted_cherry_sapling" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8209u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedDarkOakSapling {
    fn resource_location(&self) -> &'static str { "minecraft:potted_dark_oak_sapling" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8210u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedMangrovePropagule {
    fn resource_location(&self) -> &'static str { "minecraft:potted_mangrove_propagule" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8211u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedFern {
    fn resource_location(&self) -> &'static str { "minecraft:potted_fern" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8212u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedDandelion {
    fn resource_location(&self) -> &'static str { "minecraft:potted_dandelion" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8213u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedPoppy {
    fn resource_location(&self) -> &'static str { "minecraft:potted_poppy" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8214u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedBlueOrchid {
    fn resource_location(&self) -> &'static str { "minecraft:potted_blue_orchid" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8215u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedAllium {
    fn resource_location(&self) -> &'static str { "minecraft:potted_allium" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8216u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedAzureBluet {
    fn resource_location(&self) -> &'static str { "minecraft:potted_azure_bluet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8217u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedRedTulip {
    fn resource_location(&self) -> &'static str { "minecraft:potted_red_tulip" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8218u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedOrangeTulip {
    fn resource_location(&self) -> &'static str { "minecraft:potted_orange_tulip" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8219u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedWhiteTulip {
    fn resource_location(&self) -> &'static str { "minecraft:potted_white_tulip" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8220u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedPinkTulip {
    fn resource_location(&self) -> &'static str { "minecraft:potted_pink_tulip" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8221u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedOxeyeDaisy {
    fn resource_location(&self) -> &'static str { "minecraft:potted_oxeye_daisy" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8222u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedCornflower {
    fn resource_location(&self) -> &'static str { "minecraft:potted_cornflower" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8223u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedLilyOfTheValley {
    fn resource_location(&self) -> &'static str { "minecraft:potted_lily_of_the_valley" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8224u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedWitherRose {
    fn resource_location(&self) -> &'static str { "minecraft:potted_wither_rose" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8225u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedRedMushroom {
    fn resource_location(&self) -> &'static str { "minecraft:potted_red_mushroom" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8226u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedBrownMushroom {
    fn resource_location(&self) -> &'static str { "minecraft:potted_brown_mushroom" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8227u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedDeadBush {
    fn resource_location(&self) -> &'static str { "minecraft:potted_dead_bush" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8228u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedCactus {
    fn resource_location(&self) -> &'static str { "minecraft:potted_cactus" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8229u32 }
}

impl BlockTrait<V1_20_0> for BlockCarrots {
    fn resource_location(&self) -> &'static str { "minecraft:carrots" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8230u32 }
}

impl BlockTrait<V1_20_0> for BlockPotatoes {
    fn resource_location(&self) -> &'static str { "minecraft:potatoes" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8238u32 }
}

impl BlockTrait<V1_20_0> for BlockOakButton {
    fn resource_location(&self) -> &'static str { "minecraft:oak_button" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8246u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceButton {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_button" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8270u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchButton {
    fn resource_location(&self) -> &'static str { "minecraft:birch_button" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8294u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleButton {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_button" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8318u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaButton {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_button" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8342u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryButton {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_button" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8366u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakButton {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_button" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8390u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveButton {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_button" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8414u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooButton {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_button" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8438u32 }
}

impl BlockTrait<V1_20_0> for BlockSkeletonSkull {
    fn resource_location(&self) -> &'static str { "minecraft:skeleton_skull" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8462u32 }
}

impl BlockTrait<V1_20_0> for BlockSkeletonWallSkull {
    fn resource_location(&self) -> &'static str { "minecraft:skeleton_wall_skull" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8478u32 }
}

impl BlockTrait<V1_20_0> for BlockWitherSkeletonSkull {
    fn resource_location(&self) -> &'static str { "minecraft:wither_skeleton_skull" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8482u32 }
}

impl BlockTrait<V1_20_0> for BlockWitherSkeletonWallSkull {
    fn resource_location(&self) -> &'static str { "minecraft:wither_skeleton_wall_skull" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8498u32 }
}

impl BlockTrait<V1_20_0> for BlockZombieHead {
    fn resource_location(&self) -> &'static str { "minecraft:zombie_head" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8502u32 }
}

impl BlockTrait<V1_20_0> for BlockZombieWallHead {
    fn resource_location(&self) -> &'static str { "minecraft:zombie_wall_head" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8518u32 }
}

impl BlockTrait<V1_20_0> for BlockPlayerHead {
    fn resource_location(&self) -> &'static str { "minecraft:player_head" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8522u32 }
}

impl BlockTrait<V1_20_0> for BlockPlayerWallHead {
    fn resource_location(&self) -> &'static str { "minecraft:player_wall_head" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8538u32 }
}

impl BlockTrait<V1_20_0> for BlockCreeperHead {
    fn resource_location(&self) -> &'static str { "minecraft:creeper_head" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8542u32 }
}

impl BlockTrait<V1_20_0> for BlockCreeperWallHead {
    fn resource_location(&self) -> &'static str { "minecraft:creeper_wall_head" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8558u32 }
}

impl BlockTrait<V1_20_0> for BlockDragonHead {
    fn resource_location(&self) -> &'static str { "minecraft:dragon_head" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8562u32 }
}

impl BlockTrait<V1_20_0> for BlockDragonWallHead {
    fn resource_location(&self) -> &'static str { "minecraft:dragon_wall_head" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8578u32 }
}

impl BlockTrait<V1_20_0> for BlockPiglinHead {
    fn resource_location(&self) -> &'static str { "minecraft:piglin_head" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8582u32 }
}

impl BlockTrait<V1_20_0> for BlockPiglinWallHead {
    fn resource_location(&self) -> &'static str { "minecraft:piglin_wall_head" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8598u32 }
}

impl BlockTrait<V1_20_0> for BlockAnvil {
    fn resource_location(&self) -> &'static str { "minecraft:anvil" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8602u32 }
}

impl BlockTrait<V1_20_0> for BlockChippedAnvil {
    fn resource_location(&self) -> &'static str { "minecraft:chipped_anvil" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8606u32 }
}

impl BlockTrait<V1_20_0> for BlockDamagedAnvil {
    fn resource_location(&self) -> &'static str { "minecraft:damaged_anvil" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8610u32 }
}

impl BlockTrait<V1_20_0> for BlockTrappedChest {
    fn resource_location(&self) -> &'static str { "minecraft:trapped_chest" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8614u32 }
}

impl BlockTrait<V1_20_0> for BlockLightWeightedPressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:light_weighted_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8638u32 }
}

impl BlockTrait<V1_20_0> for BlockHeavyWeightedPressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:heavy_weighted_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8654u32 }
}

impl BlockTrait<V1_20_0> for BlockComparator {
    fn resource_location(&self) -> &'static str { "minecraft:comparator" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8670u32 }
}

impl BlockTrait<V1_20_0> for BlockDaylightDetector {
    fn resource_location(&self) -> &'static str { "minecraft:daylight_detector" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8686u32 }
}

impl BlockTrait<V1_20_0> for BlockRedstoneBlock {
    fn resource_location(&self) -> &'static str { "minecraft:redstone_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8718u32 }
}

impl BlockTrait<V1_20_0> for BlockNetherQuartzOre {
    fn resource_location(&self) -> &'static str { "minecraft:nether_quartz_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8719u32 }
}

impl BlockTrait<V1_20_0> for BlockHopper {
    fn resource_location(&self) -> &'static str { "minecraft:hopper" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8720u32 }
}

impl BlockTrait<V1_20_0> for BlockQuartzBlock {
    fn resource_location(&self) -> &'static str { "minecraft:quartz_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8730u32 }
}

impl BlockTrait<V1_20_0> for BlockChiseledQuartzBlock {
    fn resource_location(&self) -> &'static str { "minecraft:chiseled_quartz_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8731u32 }
}

impl BlockTrait<V1_20_0> for BlockQuartzPillar {
    fn resource_location(&self) -> &'static str { "minecraft:quartz_pillar" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8732u32 }
}

impl BlockTrait<V1_20_0> for BlockQuartzStairs {
    fn resource_location(&self) -> &'static str { "minecraft:quartz_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8735u32 }
}

impl BlockTrait<V1_20_0> for BlockActivatorRail {
    fn resource_location(&self) -> &'static str { "minecraft:activator_rail" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8815u32 }
}

impl BlockTrait<V1_20_0> for BlockDropper {
    fn resource_location(&self) -> &'static str { "minecraft:dropper" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8855u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:white_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8867u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:orange_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8868u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8869u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8870u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8871u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:lime_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8872u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:pink_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8873u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:gray_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8874u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8875u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8876u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:purple_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8877u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:blue_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8878u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:brown_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8879u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:green_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8880u32 }
}

impl BlockTrait<V1_20_0> for BlockRedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:red_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8881u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:black_terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 8882u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:white_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8883u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:orange_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8915u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8947u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 8979u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9011u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:lime_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9043u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:pink_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9075u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:gray_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9107u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9139u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9171u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:purple_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9203u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:blue_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9235u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:brown_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9267u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:green_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9299u32 }
}

impl BlockTrait<V1_20_0> for BlockRedStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:red_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9331u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackStainedGlassPane {
    fn resource_location(&self) -> &'static str { "minecraft:black_stained_glass_pane" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9363u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaStairs {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9395u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryStairs {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9475u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakStairs {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9555u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveStairs {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9635u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooStairs {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9715u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooMosaicStairs {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_mosaic_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9795u32 }
}

impl BlockTrait<V1_20_0> for BlockSlimeBlock {
    fn resource_location(&self) -> &'static str { "minecraft:slime_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 9875u32 }
}

impl BlockTrait<V1_20_0> for BlockBarrier {
    fn resource_location(&self) -> &'static str { "minecraft:barrier" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 9876u32 }
}

impl BlockTrait<V1_20_0> for BlockLight {
    fn resource_location(&self) -> &'static str { "minecraft:light" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9877u32 }
}

impl BlockTrait<V1_20_0> for BlockIronTrapdoor {
    fn resource_location(&self) -> &'static str { "minecraft:iron_trapdoor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9909u32 }
}

impl BlockTrait<V1_20_0> for BlockPrismarine {
    fn resource_location(&self) -> &'static str { "minecraft:prismarine" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 9973u32 }
}

impl BlockTrait<V1_20_0> for BlockPrismarineBricks {
    fn resource_location(&self) -> &'static str { "minecraft:prismarine_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 9974u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkPrismarine {
    fn resource_location(&self) -> &'static str { "minecraft:dark_prismarine" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 9975u32 }
}

impl BlockTrait<V1_20_0> for BlockPrismarineStairs {
    fn resource_location(&self) -> &'static str { "minecraft:prismarine_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 9976u32 }
}

impl BlockTrait<V1_20_0> for BlockPrismarineBrickStairs {
    fn resource_location(&self) -> &'static str { "minecraft:prismarine_brick_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10056u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkPrismarineStairs {
    fn resource_location(&self) -> &'static str { "minecraft:dark_prismarine_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10136u32 }
}

impl BlockTrait<V1_20_0> for BlockPrismarineSlab {
    fn resource_location(&self) -> &'static str { "minecraft:prismarine_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10216u32 }
}

impl BlockTrait<V1_20_0> for BlockPrismarineBrickSlab {
    fn resource_location(&self) -> &'static str { "minecraft:prismarine_brick_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10222u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkPrismarineSlab {
    fn resource_location(&self) -> &'static str { "minecraft:dark_prismarine_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10228u32 }
}

impl BlockTrait<V1_20_0> for BlockSeaLantern {
    fn resource_location(&self) -> &'static str { "minecraft:sea_lantern" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10234u32 }
}

impl BlockTrait<V1_20_0> for BlockHayBlock {
    fn resource_location(&self) -> &'static str { "minecraft:hay_block" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10235u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:white_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10238u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:orange_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10239u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10240u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10241u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10242u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:lime_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10243u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:pink_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10244u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:gray_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10245u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10246u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10247u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:purple_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10248u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:blue_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10249u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:brown_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10250u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:green_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10251u32 }
}

impl BlockTrait<V1_20_0> for BlockRedCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:red_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10252u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:black_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10253u32 }
}

impl BlockTrait<V1_20_0> for BlockTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:terracotta" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10254u32 }
}

impl BlockTrait<V1_20_0> for BlockCoalBlock {
    fn resource_location(&self) -> &'static str { "minecraft:coal_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10255u32 }
}

impl BlockTrait<V1_20_0> for BlockPackedIce {
    fn resource_location(&self) -> &'static str { "minecraft:packed_ice" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10256u32 }
}

impl BlockTrait<V1_20_0> for BlockSunflower {
    fn resource_location(&self) -> &'static str { "minecraft:sunflower" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10257u32 }
}

impl BlockTrait<V1_20_0> for BlockLilac {
    fn resource_location(&self) -> &'static str { "minecraft:lilac" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10259u32 }
}

impl BlockTrait<V1_20_0> for BlockRoseBush {
    fn resource_location(&self) -> &'static str { "minecraft:rose_bush" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10261u32 }
}

impl BlockTrait<V1_20_0> for BlockPeony {
    fn resource_location(&self) -> &'static str { "minecraft:peony" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10263u32 }
}

impl BlockTrait<V1_20_0> for BlockTallGrass {
    fn resource_location(&self) -> &'static str { "minecraft:tall_grass" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10265u32 }
}

impl BlockTrait<V1_20_0> for BlockLargeFern {
    fn resource_location(&self) -> &'static str { "minecraft:large_fern" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10267u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteBanner {
    fn resource_location(&self) -> &'static str { "minecraft:white_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10269u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeBanner {
    fn resource_location(&self) -> &'static str { "minecraft:orange_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10285u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaBanner {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10301u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueBanner {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10317u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowBanner {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10333u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeBanner {
    fn resource_location(&self) -> &'static str { "minecraft:lime_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10349u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkBanner {
    fn resource_location(&self) -> &'static str { "minecraft:pink_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10365u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayBanner {
    fn resource_location(&self) -> &'static str { "minecraft:gray_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10381u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayBanner {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10397u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanBanner {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10413u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleBanner {
    fn resource_location(&self) -> &'static str { "minecraft:purple_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10429u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueBanner {
    fn resource_location(&self) -> &'static str { "minecraft:blue_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10445u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownBanner {
    fn resource_location(&self) -> &'static str { "minecraft:brown_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10461u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenBanner {
    fn resource_location(&self) -> &'static str { "minecraft:green_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10477u32 }
}

impl BlockTrait<V1_20_0> for BlockRedBanner {
    fn resource_location(&self) -> &'static str { "minecraft:red_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10493u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackBanner {
    fn resource_location(&self) -> &'static str { "minecraft:black_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10509u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:white_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10525u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:orange_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10529u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10533u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10537u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10541u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:lime_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10545u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:pink_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10549u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:gray_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10553u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10557u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10561u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:purple_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10565u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:blue_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10569u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:brown_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10573u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:green_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10577u32 }
}

impl BlockTrait<V1_20_0> for BlockRedWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:red_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10581u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackWallBanner {
    fn resource_location(&self) -> &'static str { "minecraft:black_wall_banner" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10585u32 }
}

impl BlockTrait<V1_20_0> for BlockRedSandstone {
    fn resource_location(&self) -> &'static str { "minecraft:red_sandstone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10589u32 }
}

impl BlockTrait<V1_20_0> for BlockChiseledRedSandstone {
    fn resource_location(&self) -> &'static str { "minecraft:chiseled_red_sandstone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10590u32 }
}

impl BlockTrait<V1_20_0> for BlockCutRedSandstone {
    fn resource_location(&self) -> &'static str { "minecraft:cut_red_sandstone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10591u32 }
}

impl BlockTrait<V1_20_0> for BlockRedSandstoneStairs {
    fn resource_location(&self) -> &'static str { "minecraft:red_sandstone_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10592u32 }
}

impl BlockTrait<V1_20_0> for BlockOakSlab {
    fn resource_location(&self) -> &'static str { "minecraft:oak_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10672u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceSlab {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10678u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchSlab {
    fn resource_location(&self) -> &'static str { "minecraft:birch_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10684u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleSlab {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10690u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaSlab {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10696u32 }
}

impl BlockTrait<V1_20_0> for BlockCherrySlab {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10702u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakSlab {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10708u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveSlab {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10714u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooSlab {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10720u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooMosaicSlab {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_mosaic_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10726u32 }
}

impl BlockTrait<V1_20_0> for BlockStoneSlab {
    fn resource_location(&self) -> &'static str { "minecraft:stone_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10732u32 }
}

impl BlockTrait<V1_20_0> for BlockSmoothStoneSlab {
    fn resource_location(&self) -> &'static str { "minecraft:smooth_stone_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10738u32 }
}

impl BlockTrait<V1_20_0> for BlockSandstoneSlab {
    fn resource_location(&self) -> &'static str { "minecraft:sandstone_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10744u32 }
}

impl BlockTrait<V1_20_0> for BlockCutSandstoneSlab {
    fn resource_location(&self) -> &'static str { "minecraft:cut_sandstone_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10750u32 }
}

impl BlockTrait<V1_20_0> for BlockPetrifiedOakSlab {
    fn resource_location(&self) -> &'static str { "minecraft:petrified_oak_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10756u32 }
}

impl BlockTrait<V1_20_0> for BlockCobblestoneSlab {
    fn resource_location(&self) -> &'static str { "minecraft:cobblestone_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10762u32 }
}

impl BlockTrait<V1_20_0> for BlockBrickSlab {
    fn resource_location(&self) -> &'static str { "minecraft:brick_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10768u32 }
}

impl BlockTrait<V1_20_0> for BlockStoneBrickSlab {
    fn resource_location(&self) -> &'static str { "minecraft:stone_brick_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10774u32 }
}

impl BlockTrait<V1_20_0> for BlockMudBrickSlab {
    fn resource_location(&self) -> &'static str { "minecraft:mud_brick_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10780u32 }
}

impl BlockTrait<V1_20_0> for BlockNetherBrickSlab {
    fn resource_location(&self) -> &'static str { "minecraft:nether_brick_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10786u32 }
}

impl BlockTrait<V1_20_0> for BlockQuartzSlab {
    fn resource_location(&self) -> &'static str { "minecraft:quartz_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10792u32 }
}

impl BlockTrait<V1_20_0> for BlockRedSandstoneSlab {
    fn resource_location(&self) -> &'static str { "minecraft:red_sandstone_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10798u32 }
}

impl BlockTrait<V1_20_0> for BlockCutRedSandstoneSlab {
    fn resource_location(&self) -> &'static str { "minecraft:cut_red_sandstone_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10804u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpurSlab {
    fn resource_location(&self) -> &'static str { "minecraft:purpur_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10810u32 }
}

impl BlockTrait<V1_20_0> for BlockSmoothStone {
    fn resource_location(&self) -> &'static str { "minecraft:smooth_stone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10816u32 }
}

impl BlockTrait<V1_20_0> for BlockSmoothSandstone {
    fn resource_location(&self) -> &'static str { "minecraft:smooth_sandstone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10817u32 }
}

impl BlockTrait<V1_20_0> for BlockSmoothQuartz {
    fn resource_location(&self) -> &'static str { "minecraft:smooth_quartz" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10818u32 }
}

impl BlockTrait<V1_20_0> for BlockSmoothRedSandstone {
    fn resource_location(&self) -> &'static str { "minecraft:smooth_red_sandstone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 10819u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceFenceGate {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_fence_gate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10820u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchFenceGate {
    fn resource_location(&self) -> &'static str { "minecraft:birch_fence_gate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10852u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleFenceGate {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_fence_gate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10884u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaFenceGate {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_fence_gate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10916u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryFenceGate {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_fence_gate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10948u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakFenceGate {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_fence_gate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 10980u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveFenceGate {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_fence_gate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11012u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooFenceGate {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_fence_gate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11044u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceFence {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_fence" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11076u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchFence {
    fn resource_location(&self) -> &'static str { "minecraft:birch_fence" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11108u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleFence {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_fence" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11140u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaFence {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_fence" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11172u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryFence {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_fence" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11204u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakFence {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_fence" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11236u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveFence {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_fence" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11268u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooFence {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_fence" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11300u32 }
}

impl BlockTrait<V1_20_0> for BlockSpruceDoor {
    fn resource_location(&self) -> &'static str { "minecraft:spruce_door" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11332u32 }
}

impl BlockTrait<V1_20_0> for BlockBirchDoor {
    fn resource_location(&self) -> &'static str { "minecraft:birch_door" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11396u32 }
}

impl BlockTrait<V1_20_0> for BlockJungleDoor {
    fn resource_location(&self) -> &'static str { "minecraft:jungle_door" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11460u32 }
}

impl BlockTrait<V1_20_0> for BlockAcaciaDoor {
    fn resource_location(&self) -> &'static str { "minecraft:acacia_door" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11524u32 }
}

impl BlockTrait<V1_20_0> for BlockCherryDoor {
    fn resource_location(&self) -> &'static str { "minecraft:cherry_door" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11588u32 }
}

impl BlockTrait<V1_20_0> for BlockDarkOakDoor {
    fn resource_location(&self) -> &'static str { "minecraft:dark_oak_door" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11652u32 }
}

impl BlockTrait<V1_20_0> for BlockMangroveDoor {
    fn resource_location(&self) -> &'static str { "minecraft:mangrove_door" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11716u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooDoor {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_door" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11780u32 }
}

impl BlockTrait<V1_20_0> for BlockEndRod {
    fn resource_location(&self) -> &'static str { "minecraft:end_rod" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11844u32 }
}

impl BlockTrait<V1_20_0> for BlockChorusPlant {
    fn resource_location(&self) -> &'static str { "minecraft:chorus_plant" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11850u32 }
}

impl BlockTrait<V1_20_0> for BlockChorusFlower {
    fn resource_location(&self) -> &'static str { "minecraft:chorus_flower" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11914u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpurBlock {
    fn resource_location(&self) -> &'static str { "minecraft:purpur_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 11920u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpurPillar {
    fn resource_location(&self) -> &'static str { "minecraft:purpur_pillar" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11921u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpurStairs {
    fn resource_location(&self) -> &'static str { "minecraft:purpur_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 11924u32 }
}

impl BlockTrait<V1_20_0> for BlockEndStoneBricks {
    fn resource_location(&self) -> &'static str { "minecraft:end_stone_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12004u32 }
}

impl BlockTrait<V1_20_0> for BlockTorchflowerCrop {
    fn resource_location(&self) -> &'static str { "minecraft:torchflower_crop" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12005u32 }
}

impl BlockTrait<V1_20_0> for BlockPitcherCrop {
    fn resource_location(&self) -> &'static str { "minecraft:pitcher_crop" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12021u32 }
}

impl BlockTrait<V1_20_0> for BlockPitcherPlant {
    fn resource_location(&self) -> &'static str { "minecraft:pitcher_plant" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12031u32 }
}

impl BlockTrait<V1_20_0> for BlockBeetroots {
    fn resource_location(&self) -> &'static str { "minecraft:beetroots" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12033u32 }
}

impl BlockTrait<V1_20_0> for BlockDirtPath {
    fn resource_location(&self) -> &'static str { "minecraft:dirt_path" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12065u32 }
}

impl BlockTrait<V1_20_0> for BlockEndGateway {
    fn resource_location(&self) -> &'static str { "minecraft:end_gateway" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12066u32 }
}

impl BlockTrait<V1_20_0> for BlockRepeatingCommandBlock {
    fn resource_location(&self) -> &'static str { "minecraft:repeating_command_block" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12067u32 }
}

impl BlockTrait<V1_20_0> for BlockChainCommandBlock {
    fn resource_location(&self) -> &'static str { "minecraft:chain_command_block" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12079u32 }
}

impl BlockTrait<V1_20_0> for BlockFrostedIce {
    fn resource_location(&self) -> &'static str { "minecraft:frosted_ice" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12091u32 }
}

impl BlockTrait<V1_20_0> for BlockMagmaBlock {
    fn resource_location(&self) -> &'static str { "minecraft:magma_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12095u32 }
}

impl BlockTrait<V1_20_0> for BlockNetherWartBlock {
    fn resource_location(&self) -> &'static str { "minecraft:nether_wart_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12096u32 }
}

impl BlockTrait<V1_20_0> for BlockRedNetherBricks {
    fn resource_location(&self) -> &'static str { "minecraft:red_nether_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12097u32 }
}

impl BlockTrait<V1_20_0> for BlockBoneBlock {
    fn resource_location(&self) -> &'static str { "minecraft:bone_block" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12098u32 }
}

impl BlockTrait<V1_20_0> for BlockStructureVoid {
    fn resource_location(&self) -> &'static str { "minecraft:structure_void" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12101u32 }
}

impl BlockTrait<V1_20_0> for BlockObserver {
    fn resource_location(&self) -> &'static str { "minecraft:observer" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12102u32 }
}

impl BlockTrait<V1_20_0> for BlockShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12114u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:white_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12120u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:orange_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12126u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12132u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12138u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12144u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:lime_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12150u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:pink_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12156u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:gray_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12162u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12168u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12174u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:purple_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12180u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:blue_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12186u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:brown_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12192u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:green_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12198u32 }
}

impl BlockTrait<V1_20_0> for BlockRedShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:red_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12204u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackShulkerBox {
    fn resource_location(&self) -> &'static str { "minecraft:black_shulker_box" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12210u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:white_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12216u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:orange_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12220u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12224u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12228u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12232u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:lime_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12236u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:pink_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12240u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:gray_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12244u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12248u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12252u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:purple_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12256u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:blue_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12260u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:brown_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12264u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:green_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12268u32 }
}

impl BlockTrait<V1_20_0> for BlockRedGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:red_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12272u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackGlazedTerracotta {
    fn resource_location(&self) -> &'static str { "minecraft:black_glazed_terracotta" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12276u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:white_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12280u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:orange_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12281u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12282u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12283u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12284u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:lime_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12285u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:pink_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12286u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:gray_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12287u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12288u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12289u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:purple_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12290u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:blue_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12291u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:brown_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12292u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:green_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12293u32 }
}

impl BlockTrait<V1_20_0> for BlockRedConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:red_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12294u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackConcrete {
    fn resource_location(&self) -> &'static str { "minecraft:black_concrete" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12295u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:white_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12296u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:orange_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12297u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12298u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12299u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12300u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:lime_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12301u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:pink_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12302u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:gray_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12303u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12304u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12305u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:purple_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12306u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:blue_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12307u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:brown_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12308u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:green_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12309u32 }
}

impl BlockTrait<V1_20_0> for BlockRedConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:red_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12310u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackConcretePowder {
    fn resource_location(&self) -> &'static str { "minecraft:black_concrete_powder" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12311u32 }
}

impl BlockTrait<V1_20_0> for BlockKelp {
    fn resource_location(&self) -> &'static str { "minecraft:kelp" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12312u32 }
}

impl BlockTrait<V1_20_0> for BlockKelpPlant {
    fn resource_location(&self) -> &'static str { "minecraft:kelp_plant" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12338u32 }
}

impl BlockTrait<V1_20_0> for BlockDriedKelpBlock {
    fn resource_location(&self) -> &'static str { "minecraft:dried_kelp_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12339u32 }
}

impl BlockTrait<V1_20_0> for BlockTurtleEgg {
    fn resource_location(&self) -> &'static str { "minecraft:turtle_egg" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12340u32 }
}

impl BlockTrait<V1_20_0> for BlockSnifferEgg {
    fn resource_location(&self) -> &'static str { "minecraft:sniffer_egg" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12352u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadTubeCoralBlock {
    fn resource_location(&self) -> &'static str { "minecraft:dead_tube_coral_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12355u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadBrainCoralBlock {
    fn resource_location(&self) -> &'static str { "minecraft:dead_brain_coral_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12356u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadBubbleCoralBlock {
    fn resource_location(&self) -> &'static str { "minecraft:dead_bubble_coral_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12357u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadFireCoralBlock {
    fn resource_location(&self) -> &'static str { "minecraft:dead_fire_coral_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12358u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadHornCoralBlock {
    fn resource_location(&self) -> &'static str { "minecraft:dead_horn_coral_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12359u32 }
}

impl BlockTrait<V1_20_0> for BlockTubeCoralBlock {
    fn resource_location(&self) -> &'static str { "minecraft:tube_coral_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12360u32 }
}

impl BlockTrait<V1_20_0> for BlockBrainCoralBlock {
    fn resource_location(&self) -> &'static str { "minecraft:brain_coral_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12361u32 }
}

impl BlockTrait<V1_20_0> for BlockBubbleCoralBlock {
    fn resource_location(&self) -> &'static str { "minecraft:bubble_coral_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12362u32 }
}

impl BlockTrait<V1_20_0> for BlockFireCoralBlock {
    fn resource_location(&self) -> &'static str { "minecraft:fire_coral_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12363u32 }
}

impl BlockTrait<V1_20_0> for BlockHornCoralBlock {
    fn resource_location(&self) -> &'static str { "minecraft:horn_coral_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12364u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadTubeCoral {
    fn resource_location(&self) -> &'static str { "minecraft:dead_tube_coral" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12365u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadBrainCoral {
    fn resource_location(&self) -> &'static str { "minecraft:dead_brain_coral" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12367u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadBubbleCoral {
    fn resource_location(&self) -> &'static str { "minecraft:dead_bubble_coral" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12369u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadFireCoral {
    fn resource_location(&self) -> &'static str { "minecraft:dead_fire_coral" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12371u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadHornCoral {
    fn resource_location(&self) -> &'static str { "minecraft:dead_horn_coral" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12373u32 }
}

impl BlockTrait<V1_20_0> for BlockTubeCoral {
    fn resource_location(&self) -> &'static str { "minecraft:tube_coral" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12375u32 }
}

impl BlockTrait<V1_20_0> for BlockBrainCoral {
    fn resource_location(&self) -> &'static str { "minecraft:brain_coral" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12377u32 }
}

impl BlockTrait<V1_20_0> for BlockBubbleCoral {
    fn resource_location(&self) -> &'static str { "minecraft:bubble_coral" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12379u32 }
}

impl BlockTrait<V1_20_0> for BlockFireCoral {
    fn resource_location(&self) -> &'static str { "minecraft:fire_coral" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12381u32 }
}

impl BlockTrait<V1_20_0> for BlockHornCoral {
    fn resource_location(&self) -> &'static str { "minecraft:horn_coral" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12383u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadTubeCoralFan {
    fn resource_location(&self) -> &'static str { "minecraft:dead_tube_coral_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12385u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadBrainCoralFan {
    fn resource_location(&self) -> &'static str { "minecraft:dead_brain_coral_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12387u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadBubbleCoralFan {
    fn resource_location(&self) -> &'static str { "minecraft:dead_bubble_coral_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12389u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadFireCoralFan {
    fn resource_location(&self) -> &'static str { "minecraft:dead_fire_coral_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12391u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadHornCoralFan {
    fn resource_location(&self) -> &'static str { "minecraft:dead_horn_coral_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12393u32 }
}

impl BlockTrait<V1_20_0> for BlockTubeCoralFan {
    fn resource_location(&self) -> &'static str { "minecraft:tube_coral_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12395u32 }
}

impl BlockTrait<V1_20_0> for BlockBrainCoralFan {
    fn resource_location(&self) -> &'static str { "minecraft:brain_coral_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12397u32 }
}

impl BlockTrait<V1_20_0> for BlockBubbleCoralFan {
    fn resource_location(&self) -> &'static str { "minecraft:bubble_coral_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12399u32 }
}

impl BlockTrait<V1_20_0> for BlockFireCoralFan {
    fn resource_location(&self) -> &'static str { "minecraft:fire_coral_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12401u32 }
}

impl BlockTrait<V1_20_0> for BlockHornCoralFan {
    fn resource_location(&self) -> &'static str { "minecraft:horn_coral_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12403u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadTubeCoralWallFan {
    fn resource_location(&self) -> &'static str { "minecraft:dead_tube_coral_wall_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12405u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadBrainCoralWallFan {
    fn resource_location(&self) -> &'static str { "minecraft:dead_brain_coral_wall_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12413u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadBubbleCoralWallFan {
    fn resource_location(&self) -> &'static str { "minecraft:dead_bubble_coral_wall_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12421u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadFireCoralWallFan {
    fn resource_location(&self) -> &'static str { "minecraft:dead_fire_coral_wall_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12429u32 }
}

impl BlockTrait<V1_20_0> for BlockDeadHornCoralWallFan {
    fn resource_location(&self) -> &'static str { "minecraft:dead_horn_coral_wall_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12437u32 }
}

impl BlockTrait<V1_20_0> for BlockTubeCoralWallFan {
    fn resource_location(&self) -> &'static str { "minecraft:tube_coral_wall_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12445u32 }
}

impl BlockTrait<V1_20_0> for BlockBrainCoralWallFan {
    fn resource_location(&self) -> &'static str { "minecraft:brain_coral_wall_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12453u32 }
}

impl BlockTrait<V1_20_0> for BlockBubbleCoralWallFan {
    fn resource_location(&self) -> &'static str { "minecraft:bubble_coral_wall_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12461u32 }
}

impl BlockTrait<V1_20_0> for BlockFireCoralWallFan {
    fn resource_location(&self) -> &'static str { "minecraft:fire_coral_wall_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12469u32 }
}

impl BlockTrait<V1_20_0> for BlockHornCoralWallFan {
    fn resource_location(&self) -> &'static str { "minecraft:horn_coral_wall_fan" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12477u32 }
}

impl BlockTrait<V1_20_0> for BlockSeaPickle {
    fn resource_location(&self) -> &'static str { "minecraft:sea_pickle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12485u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueIce {
    fn resource_location(&self) -> &'static str { "minecraft:blue_ice" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12493u32 }
}

impl BlockTrait<V1_20_0> for BlockConduit {
    fn resource_location(&self) -> &'static str { "minecraft:conduit" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12494u32 }
}

impl BlockTrait<V1_20_0> for BlockBambooSapling {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo_sapling" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12496u32 }
}

impl BlockTrait<V1_20_0> for BlockBamboo {
    fn resource_location(&self) -> &'static str { "minecraft:bamboo" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12497u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedBamboo {
    fn resource_location(&self) -> &'static str { "minecraft:potted_bamboo" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12509u32 }
}

impl BlockTrait<V1_20_0> for BlockVoidAir {
    fn resource_location(&self) -> &'static str { "minecraft:void_air" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12510u32 }
}

impl BlockTrait<V1_20_0> for BlockCaveAir {
    fn resource_location(&self) -> &'static str { "minecraft:cave_air" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 12511u32 }
}

impl BlockTrait<V1_20_0> for BlockBubbleColumn {
    fn resource_location(&self) -> &'static str { "minecraft:bubble_column" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12512u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedGraniteStairs {
    fn resource_location(&self) -> &'static str { "minecraft:polished_granite_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12514u32 }
}

impl BlockTrait<V1_20_0> for BlockSmoothRedSandstoneStairs {
    fn resource_location(&self) -> &'static str { "minecraft:smooth_red_sandstone_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12594u32 }
}

impl BlockTrait<V1_20_0> for BlockMossyStoneBrickStairs {
    fn resource_location(&self) -> &'static str { "minecraft:mossy_stone_brick_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12674u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedDioriteStairs {
    fn resource_location(&self) -> &'static str { "minecraft:polished_diorite_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12754u32 }
}

impl BlockTrait<V1_20_0> for BlockMossyCobblestoneStairs {
    fn resource_location(&self) -> &'static str { "minecraft:mossy_cobblestone_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12834u32 }
}

impl BlockTrait<V1_20_0> for BlockEndStoneBrickStairs {
    fn resource_location(&self) -> &'static str { "minecraft:end_stone_brick_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12914u32 }
}

impl BlockTrait<V1_20_0> for BlockStoneStairs {
    fn resource_location(&self) -> &'static str { "minecraft:stone_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 12994u32 }
}

impl BlockTrait<V1_20_0> for BlockSmoothSandstoneStairs {
    fn resource_location(&self) -> &'static str { "minecraft:smooth_sandstone_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13074u32 }
}

impl BlockTrait<V1_20_0> for BlockSmoothQuartzStairs {
    fn resource_location(&self) -> &'static str { "minecraft:smooth_quartz_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13154u32 }
}

impl BlockTrait<V1_20_0> for BlockGraniteStairs {
    fn resource_location(&self) -> &'static str { "minecraft:granite_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13234u32 }
}

impl BlockTrait<V1_20_0> for BlockAndesiteStairs {
    fn resource_location(&self) -> &'static str { "minecraft:andesite_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13314u32 }
}

impl BlockTrait<V1_20_0> for BlockRedNetherBrickStairs {
    fn resource_location(&self) -> &'static str { "minecraft:red_nether_brick_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13394u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedAndesiteStairs {
    fn resource_location(&self) -> &'static str { "minecraft:polished_andesite_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13474u32 }
}

impl BlockTrait<V1_20_0> for BlockDioriteStairs {
    fn resource_location(&self) -> &'static str { "minecraft:diorite_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13554u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedGraniteSlab {
    fn resource_location(&self) -> &'static str { "minecraft:polished_granite_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13634u32 }
}

impl BlockTrait<V1_20_0> for BlockSmoothRedSandstoneSlab {
    fn resource_location(&self) -> &'static str { "minecraft:smooth_red_sandstone_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13640u32 }
}

impl BlockTrait<V1_20_0> for BlockMossyStoneBrickSlab {
    fn resource_location(&self) -> &'static str { "minecraft:mossy_stone_brick_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13646u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedDioriteSlab {
    fn resource_location(&self) -> &'static str { "minecraft:polished_diorite_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13652u32 }
}

impl BlockTrait<V1_20_0> for BlockMossyCobblestoneSlab {
    fn resource_location(&self) -> &'static str { "minecraft:mossy_cobblestone_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13658u32 }
}

impl BlockTrait<V1_20_0> for BlockEndStoneBrickSlab {
    fn resource_location(&self) -> &'static str { "minecraft:end_stone_brick_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13664u32 }
}

impl BlockTrait<V1_20_0> for BlockSmoothSandstoneSlab {
    fn resource_location(&self) -> &'static str { "minecraft:smooth_sandstone_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13670u32 }
}

impl BlockTrait<V1_20_0> for BlockSmoothQuartzSlab {
    fn resource_location(&self) -> &'static str { "minecraft:smooth_quartz_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13676u32 }
}

impl BlockTrait<V1_20_0> for BlockGraniteSlab {
    fn resource_location(&self) -> &'static str { "minecraft:granite_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13682u32 }
}

impl BlockTrait<V1_20_0> for BlockAndesiteSlab {
    fn resource_location(&self) -> &'static str { "minecraft:andesite_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13688u32 }
}

impl BlockTrait<V1_20_0> for BlockRedNetherBrickSlab {
    fn resource_location(&self) -> &'static str { "minecraft:red_nether_brick_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13694u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedAndesiteSlab {
    fn resource_location(&self) -> &'static str { "minecraft:polished_andesite_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13700u32 }
}

impl BlockTrait<V1_20_0> for BlockDioriteSlab {
    fn resource_location(&self) -> &'static str { "minecraft:diorite_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13706u32 }
}

impl BlockTrait<V1_20_0> for BlockBrickWall {
    fn resource_location(&self) -> &'static str { "minecraft:brick_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 13712u32 }
}

impl BlockTrait<V1_20_0> for BlockPrismarineWall {
    fn resource_location(&self) -> &'static str { "minecraft:prismarine_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 14036u32 }
}

impl BlockTrait<V1_20_0> for BlockRedSandstoneWall {
    fn resource_location(&self) -> &'static str { "minecraft:red_sandstone_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 14360u32 }
}

impl BlockTrait<V1_20_0> for BlockMossyStoneBrickWall {
    fn resource_location(&self) -> &'static str { "minecraft:mossy_stone_brick_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 14684u32 }
}

impl BlockTrait<V1_20_0> for BlockGraniteWall {
    fn resource_location(&self) -> &'static str { "minecraft:granite_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 15008u32 }
}

impl BlockTrait<V1_20_0> for BlockStoneBrickWall {
    fn resource_location(&self) -> &'static str { "minecraft:stone_brick_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 15332u32 }
}

impl BlockTrait<V1_20_0> for BlockMudBrickWall {
    fn resource_location(&self) -> &'static str { "minecraft:mud_brick_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 15656u32 }
}

impl BlockTrait<V1_20_0> for BlockNetherBrickWall {
    fn resource_location(&self) -> &'static str { "minecraft:nether_brick_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 15980u32 }
}

impl BlockTrait<V1_20_0> for BlockAndesiteWall {
    fn resource_location(&self) -> &'static str { "minecraft:andesite_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 16304u32 }
}

impl BlockTrait<V1_20_0> for BlockRedNetherBrickWall {
    fn resource_location(&self) -> &'static str { "minecraft:red_nether_brick_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 16628u32 }
}

impl BlockTrait<V1_20_0> for BlockSandstoneWall {
    fn resource_location(&self) -> &'static str { "minecraft:sandstone_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 16952u32 }
}

impl BlockTrait<V1_20_0> for BlockEndStoneBrickWall {
    fn resource_location(&self) -> &'static str { "minecraft:end_stone_brick_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 17276u32 }
}

impl BlockTrait<V1_20_0> for BlockDioriteWall {
    fn resource_location(&self) -> &'static str { "minecraft:diorite_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 17600u32 }
}

impl BlockTrait<V1_20_0> for BlockScaffolding {
    fn resource_location(&self) -> &'static str { "minecraft:scaffolding" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 17924u32 }
}

impl BlockTrait<V1_20_0> for BlockLoom {
    fn resource_location(&self) -> &'static str { "minecraft:loom" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 17956u32 }
}

impl BlockTrait<V1_20_0> for BlockBarrel {
    fn resource_location(&self) -> &'static str { "minecraft:barrel" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 17960u32 }
}

impl BlockTrait<V1_20_0> for BlockSmoker {
    fn resource_location(&self) -> &'static str { "minecraft:smoker" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 17972u32 }
}

impl BlockTrait<V1_20_0> for BlockBlastFurnace {
    fn resource_location(&self) -> &'static str { "minecraft:blast_furnace" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 17980u32 }
}

impl BlockTrait<V1_20_0> for BlockCartographyTable {
    fn resource_location(&self) -> &'static str { "minecraft:cartography_table" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 17988u32 }
}

impl BlockTrait<V1_20_0> for BlockFletchingTable {
    fn resource_location(&self) -> &'static str { "minecraft:fletching_table" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 17989u32 }
}

impl BlockTrait<V1_20_0> for BlockGrindstone {
    fn resource_location(&self) -> &'static str { "minecraft:grindstone" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 17990u32 }
}

impl BlockTrait<V1_20_0> for BlockLectern {
    fn resource_location(&self) -> &'static str { "minecraft:lectern" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18002u32 }
}

impl BlockTrait<V1_20_0> for BlockSmithingTable {
    fn resource_location(&self) -> &'static str { "minecraft:smithing_table" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18018u32 }
}

impl BlockTrait<V1_20_0> for BlockStonecutter {
    fn resource_location(&self) -> &'static str { "minecraft:stonecutter" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18019u32 }
}

impl BlockTrait<V1_20_0> for BlockBell {
    fn resource_location(&self) -> &'static str { "minecraft:bell" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18023u32 }
}

impl BlockTrait<V1_20_0> for BlockLantern {
    fn resource_location(&self) -> &'static str { "minecraft:lantern" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18055u32 }
}

impl BlockTrait<V1_20_0> for BlockSoulLantern {
    fn resource_location(&self) -> &'static str { "minecraft:soul_lantern" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18059u32 }
}

impl BlockTrait<V1_20_0> for BlockCampfire {
    fn resource_location(&self) -> &'static str { "minecraft:campfire" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18063u32 }
}

impl BlockTrait<V1_20_0> for BlockSoulCampfire {
    fn resource_location(&self) -> &'static str { "minecraft:soul_campfire" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18095u32 }
}

impl BlockTrait<V1_20_0> for BlockSweetBerryBush {
    fn resource_location(&self) -> &'static str { "minecraft:sweet_berry_bush" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18127u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedStem {
    fn resource_location(&self) -> &'static str { "minecraft:warped_stem" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18131u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedWarpedStem {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_warped_stem" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18132u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedHyphae {
    fn resource_location(&self) -> &'static str { "minecraft:warped_hyphae" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18133u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedWarpedHyphae {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_warped_hyphae" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18136u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedNylium {
    fn resource_location(&self) -> &'static str { "minecraft:warped_nylium" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18139u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedFungus {
    fn resource_location(&self) -> &'static str { "minecraft:warped_fungus" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18140u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedWartBlock {
    fn resource_location(&self) -> &'static str { "minecraft:warped_wart_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18141u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedRoots {
    fn resource_location(&self) -> &'static str { "minecraft:warped_roots" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18142u32 }
}

impl BlockTrait<V1_20_0> for BlockNetherSprouts {
    fn resource_location(&self) -> &'static str { "minecraft:nether_sprouts" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18143u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonStem {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_stem" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18144u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedCrimsonStem {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_crimson_stem" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18145u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonHyphae {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_hyphae" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18146u32 }
}

impl BlockTrait<V1_20_0> for BlockStrippedCrimsonHyphae {
    fn resource_location(&self) -> &'static str { "minecraft:stripped_crimson_hyphae" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18149u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonNylium {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_nylium" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18152u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonFungus {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_fungus" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18153u32 }
}

impl BlockTrait<V1_20_0> for BlockShroomlight {
    fn resource_location(&self) -> &'static str { "minecraft:shroomlight" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18154u32 }
}

impl BlockTrait<V1_20_0> for BlockWeepingVines {
    fn resource_location(&self) -> &'static str { "minecraft:weeping_vines" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18155u32 }
}

impl BlockTrait<V1_20_0> for BlockWeepingVinesPlant {
    fn resource_location(&self) -> &'static str { "minecraft:weeping_vines_plant" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18181u32 }
}

impl BlockTrait<V1_20_0> for BlockTwistingVines {
    fn resource_location(&self) -> &'static str { "minecraft:twisting_vines" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18182u32 }
}

impl BlockTrait<V1_20_0> for BlockTwistingVinesPlant {
    fn resource_location(&self) -> &'static str { "minecraft:twisting_vines_plant" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18208u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonRoots {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_roots" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18209u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonPlanks {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_planks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18210u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedPlanks {
    fn resource_location(&self) -> &'static str { "minecraft:warped_planks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18211u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonSlab {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18212u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedSlab {
    fn resource_location(&self) -> &'static str { "minecraft:warped_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18218u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonPressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18224u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedPressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:warped_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18226u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonFence {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_fence" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18228u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedFence {
    fn resource_location(&self) -> &'static str { "minecraft:warped_fence" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18260u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonTrapdoor {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_trapdoor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18292u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedTrapdoor {
    fn resource_location(&self) -> &'static str { "minecraft:warped_trapdoor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18356u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonFenceGate {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_fence_gate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18420u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedFenceGate {
    fn resource_location(&self) -> &'static str { "minecraft:warped_fence_gate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18452u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonStairs {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18484u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedStairs {
    fn resource_location(&self) -> &'static str { "minecraft:warped_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18564u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonButton {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_button" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18644u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedButton {
    fn resource_location(&self) -> &'static str { "minecraft:warped_button" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18668u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonDoor {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_door" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18692u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedDoor {
    fn resource_location(&self) -> &'static str { "minecraft:warped_door" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18756u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonSign {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18820u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedSign {
    fn resource_location(&self) -> &'static str { "minecraft:warped_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18852u32 }
}

impl BlockTrait<V1_20_0> for BlockCrimsonWallSign {
    fn resource_location(&self) -> &'static str { "minecraft:crimson_wall_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18884u32 }
}

impl BlockTrait<V1_20_0> for BlockWarpedWallSign {
    fn resource_location(&self) -> &'static str { "minecraft:warped_wall_sign" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18892u32 }
}

impl BlockTrait<V1_20_0> for BlockStructureBlock {
    fn resource_location(&self) -> &'static str { "minecraft:structure_block" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18900u32 }
}

impl BlockTrait<V1_20_0> for BlockJigsaw {
    fn resource_location(&self) -> &'static str { "minecraft:jigsaw" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18904u32 }
}

impl BlockTrait<V1_20_0> for BlockComposter {
    fn resource_location(&self) -> &'static str { "minecraft:composter" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18916u32 }
}

impl BlockTrait<V1_20_0> for BlockTarget {
    fn resource_location(&self) -> &'static str { "minecraft:target" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18925u32 }
}

impl BlockTrait<V1_20_0> for BlockBeeNest {
    fn resource_location(&self) -> &'static str { "minecraft:bee_nest" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18941u32 }
}

impl BlockTrait<V1_20_0> for BlockBeehive {
    fn resource_location(&self) -> &'static str { "minecraft:beehive" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18965u32 }
}

impl BlockTrait<V1_20_0> for BlockHoneyBlock {
    fn resource_location(&self) -> &'static str { "minecraft:honey_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18989u32 }
}

impl BlockTrait<V1_20_0> for BlockHoneycombBlock {
    fn resource_location(&self) -> &'static str { "minecraft:honeycomb_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18990u32 }
}

impl BlockTrait<V1_20_0> for BlockNetheriteBlock {
    fn resource_location(&self) -> &'static str { "minecraft:netherite_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18991u32 }
}

impl BlockTrait<V1_20_0> for BlockAncientDebris {
    fn resource_location(&self) -> &'static str { "minecraft:ancient_debris" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18992u32 }
}

impl BlockTrait<V1_20_0> for BlockCryingObsidian {
    fn resource_location(&self) -> &'static str { "minecraft:crying_obsidian" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18993u32 }
}

impl BlockTrait<V1_20_0> for BlockRespawnAnchor {
    fn resource_location(&self) -> &'static str { "minecraft:respawn_anchor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 18994u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedCrimsonFungus {
    fn resource_location(&self) -> &'static str { "minecraft:potted_crimson_fungus" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 18999u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedWarpedFungus {
    fn resource_location(&self) -> &'static str { "minecraft:potted_warped_fungus" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 19000u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedCrimsonRoots {
    fn resource_location(&self) -> &'static str { "minecraft:potted_crimson_roots" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 19001u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedWarpedRoots {
    fn resource_location(&self) -> &'static str { "minecraft:potted_warped_roots" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 19002u32 }
}

impl BlockTrait<V1_20_0> for BlockLodestone {
    fn resource_location(&self) -> &'static str { "minecraft:lodestone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 19003u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackstone {
    fn resource_location(&self) -> &'static str { "minecraft:blackstone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 19004u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackstoneStairs {
    fn resource_location(&self) -> &'static str { "minecraft:blackstone_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 19005u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackstoneWall {
    fn resource_location(&self) -> &'static str { "minecraft:blackstone_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 19085u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackstoneSlab {
    fn resource_location(&self) -> &'static str { "minecraft:blackstone_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 19409u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedBlackstone {
    fn resource_location(&self) -> &'static str { "minecraft:polished_blackstone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 19415u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedBlackstoneBricks {
    fn resource_location(&self) -> &'static str { "minecraft:polished_blackstone_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 19416u32 }
}

impl BlockTrait<V1_20_0> for BlockCrackedPolishedBlackstoneBricks {
    fn resource_location(&self) -> &'static str { "minecraft:cracked_polished_blackstone_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 19417u32 }
}

impl BlockTrait<V1_20_0> for BlockChiseledPolishedBlackstone {
    fn resource_location(&self) -> &'static str { "minecraft:chiseled_polished_blackstone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 19418u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedBlackstoneBrickSlab {
    fn resource_location(&self) -> &'static str { "minecraft:polished_blackstone_brick_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 19419u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedBlackstoneBrickStairs {
    fn resource_location(&self) -> &'static str { "minecraft:polished_blackstone_brick_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 19425u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedBlackstoneBrickWall {
    fn resource_location(&self) -> &'static str { "minecraft:polished_blackstone_brick_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 19505u32 }
}

impl BlockTrait<V1_20_0> for BlockGildedBlackstone {
    fn resource_location(&self) -> &'static str { "minecraft:gilded_blackstone" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 19829u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedBlackstoneStairs {
    fn resource_location(&self) -> &'static str { "minecraft:polished_blackstone_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 19830u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedBlackstoneSlab {
    fn resource_location(&self) -> &'static str { "minecraft:polished_blackstone_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 19910u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedBlackstonePressurePlate {
    fn resource_location(&self) -> &'static str { "minecraft:polished_blackstone_pressure_plate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 19916u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedBlackstoneButton {
    fn resource_location(&self) -> &'static str { "minecraft:polished_blackstone_button" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 19918u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedBlackstoneWall {
    fn resource_location(&self) -> &'static str { "minecraft:polished_blackstone_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 19942u32 }
}

impl BlockTrait<V1_20_0> for BlockChiseledNetherBricks {
    fn resource_location(&self) -> &'static str { "minecraft:chiseled_nether_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 20266u32 }
}

impl BlockTrait<V1_20_0> for BlockCrackedNetherBricks {
    fn resource_location(&self) -> &'static str { "minecraft:cracked_nether_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 20267u32 }
}

impl BlockTrait<V1_20_0> for BlockQuartzBricks {
    fn resource_location(&self) -> &'static str { "minecraft:quartz_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 20268u32 }
}

impl BlockTrait<V1_20_0> for BlockCandle {
    fn resource_location(&self) -> &'static str { "minecraft:candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20269u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteCandle {
    fn resource_location(&self) -> &'static str { "minecraft:white_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20285u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeCandle {
    fn resource_location(&self) -> &'static str { "minecraft:orange_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20301u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaCandle {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20317u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueCandle {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20333u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowCandle {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20349u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeCandle {
    fn resource_location(&self) -> &'static str { "minecraft:lime_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20365u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkCandle {
    fn resource_location(&self) -> &'static str { "minecraft:pink_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20381u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayCandle {
    fn resource_location(&self) -> &'static str { "minecraft:gray_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20397u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayCandle {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20413u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanCandle {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20429u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleCandle {
    fn resource_location(&self) -> &'static str { "minecraft:purple_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20445u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueCandle {
    fn resource_location(&self) -> &'static str { "minecraft:blue_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20461u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownCandle {
    fn resource_location(&self) -> &'static str { "minecraft:brown_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20477u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenCandle {
    fn resource_location(&self) -> &'static str { "minecraft:green_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20493u32 }
}

impl BlockTrait<V1_20_0> for BlockRedCandle {
    fn resource_location(&self) -> &'static str { "minecraft:red_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20509u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackCandle {
    fn resource_location(&self) -> &'static str { "minecraft:black_candle" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20525u32 }
}

impl BlockTrait<V1_20_0> for BlockCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20541u32 }
}

impl BlockTrait<V1_20_0> for BlockWhiteCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:white_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20543u32 }
}

impl BlockTrait<V1_20_0> for BlockOrangeCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:orange_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20545u32 }
}

impl BlockTrait<V1_20_0> for BlockMagentaCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:magenta_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20547u32 }
}

impl BlockTrait<V1_20_0> for BlockLightBlueCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:light_blue_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20549u32 }
}

impl BlockTrait<V1_20_0> for BlockYellowCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:yellow_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20551u32 }
}

impl BlockTrait<V1_20_0> for BlockLimeCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:lime_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20553u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:pink_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20555u32 }
}

impl BlockTrait<V1_20_0> for BlockGrayCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:gray_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20557u32 }
}

impl BlockTrait<V1_20_0> for BlockLightGrayCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:light_gray_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20559u32 }
}

impl BlockTrait<V1_20_0> for BlockCyanCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:cyan_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20561u32 }
}

impl BlockTrait<V1_20_0> for BlockPurpleCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:purple_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20563u32 }
}

impl BlockTrait<V1_20_0> for BlockBlueCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:blue_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20565u32 }
}

impl BlockTrait<V1_20_0> for BlockBrownCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:brown_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20567u32 }
}

impl BlockTrait<V1_20_0> for BlockGreenCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:green_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20569u32 }
}

impl BlockTrait<V1_20_0> for BlockRedCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:red_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20571u32 }
}

impl BlockTrait<V1_20_0> for BlockBlackCandleCake {
    fn resource_location(&self) -> &'static str { "minecraft:black_candle_cake" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20573u32 }
}

impl BlockTrait<V1_20_0> for BlockAmethystBlock {
    fn resource_location(&self) -> &'static str { "minecraft:amethyst_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 20575u32 }
}

impl BlockTrait<V1_20_0> for BlockBuddingAmethyst {
    fn resource_location(&self) -> &'static str { "minecraft:budding_amethyst" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 20576u32 }
}

impl BlockTrait<V1_20_0> for BlockAmethystCluster {
    fn resource_location(&self) -> &'static str { "minecraft:amethyst_cluster" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20577u32 }
}

impl BlockTrait<V1_20_0> for BlockLargeAmethystBud {
    fn resource_location(&self) -> &'static str { "minecraft:large_amethyst_bud" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20589u32 }
}

impl BlockTrait<V1_20_0> for BlockMediumAmethystBud {
    fn resource_location(&self) -> &'static str { "minecraft:medium_amethyst_bud" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20601u32 }
}

impl BlockTrait<V1_20_0> for BlockSmallAmethystBud {
    fn resource_location(&self) -> &'static str { "minecraft:small_amethyst_bud" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20613u32 }
}

impl BlockTrait<V1_20_0> for BlockTuff {
    fn resource_location(&self) -> &'static str { "minecraft:tuff" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 20625u32 }
}

impl BlockTrait<V1_20_0> for BlockCalcite {
    fn resource_location(&self) -> &'static str { "minecraft:calcite" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 20626u32 }
}

impl BlockTrait<V1_20_0> for BlockTintedGlass {
    fn resource_location(&self) -> &'static str { "minecraft:tinted_glass" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 20627u32 }
}

impl BlockTrait<V1_20_0> for BlockPowderSnow {
    fn resource_location(&self) -> &'static str { "minecraft:powder_snow" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 20628u32 }
}

impl BlockTrait<V1_20_0> for BlockSculkSensor {
    fn resource_location(&self) -> &'static str { "minecraft:sculk_sensor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20629u32 }
}

impl BlockTrait<V1_20_0> for BlockCalibratedSculkSensor {
    fn resource_location(&self) -> &'static str { "minecraft:calibrated_sculk_sensor" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 20725u32 }
}

impl BlockTrait<V1_20_0> for BlockSculk {
    fn resource_location(&self) -> &'static str { "minecraft:sculk" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21109u32 }
}

impl BlockTrait<V1_20_0> for BlockSculkVein {
    fn resource_location(&self) -> &'static str { "minecraft:sculk_vein" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21110u32 }
}

impl BlockTrait<V1_20_0> for BlockSculkCatalyst {
    fn resource_location(&self) -> &'static str { "minecraft:sculk_catalyst" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21112u32 }
}

impl BlockTrait<V1_20_0> for BlockSculkShrieker {
    fn resource_location(&self) -> &'static str { "minecraft:sculk_shrieker" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21114u32 }
}

impl BlockTrait<V1_20_0> for BlockOxidizedCopper {
    fn resource_location(&self) -> &'static str { "minecraft:oxidized_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21122u32 }
}

impl BlockTrait<V1_20_0> for BlockWeatheredCopper {
    fn resource_location(&self) -> &'static str { "minecraft:weathered_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21123u32 }
}

impl BlockTrait<V1_20_0> for BlockExposedCopper {
    fn resource_location(&self) -> &'static str { "minecraft:exposed_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21124u32 }
}

impl BlockTrait<V1_20_0> for BlockCopperBlock {
    fn resource_location(&self) -> &'static str { "minecraft:copper_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21125u32 }
}

impl BlockTrait<V1_20_0> for BlockCopperOre {
    fn resource_location(&self) -> &'static str { "minecraft:copper_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21126u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateCopperOre {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_copper_ore" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21127u32 }
}

impl BlockTrait<V1_20_0> for BlockOxidizedCutCopper {
    fn resource_location(&self) -> &'static str { "minecraft:oxidized_cut_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21128u32 }
}

impl BlockTrait<V1_20_0> for BlockWeatheredCutCopper {
    fn resource_location(&self) -> &'static str { "minecraft:weathered_cut_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21129u32 }
}

impl BlockTrait<V1_20_0> for BlockExposedCutCopper {
    fn resource_location(&self) -> &'static str { "minecraft:exposed_cut_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21130u32 }
}

impl BlockTrait<V1_20_0> for BlockCutCopper {
    fn resource_location(&self) -> &'static str { "minecraft:cut_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21131u32 }
}

impl BlockTrait<V1_20_0> for BlockOxidizedCutCopperStairs {
    fn resource_location(&self) -> &'static str { "minecraft:oxidized_cut_copper_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21132u32 }
}

impl BlockTrait<V1_20_0> for BlockWeatheredCutCopperStairs {
    fn resource_location(&self) -> &'static str { "minecraft:weathered_cut_copper_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21212u32 }
}

impl BlockTrait<V1_20_0> for BlockExposedCutCopperStairs {
    fn resource_location(&self) -> &'static str { "minecraft:exposed_cut_copper_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21292u32 }
}

impl BlockTrait<V1_20_0> for BlockCutCopperStairs {
    fn resource_location(&self) -> &'static str { "minecraft:cut_copper_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21372u32 }
}

impl BlockTrait<V1_20_0> for BlockOxidizedCutCopperSlab {
    fn resource_location(&self) -> &'static str { "minecraft:oxidized_cut_copper_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21452u32 }
}

impl BlockTrait<V1_20_0> for BlockWeatheredCutCopperSlab {
    fn resource_location(&self) -> &'static str { "minecraft:weathered_cut_copper_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21458u32 }
}

impl BlockTrait<V1_20_0> for BlockExposedCutCopperSlab {
    fn resource_location(&self) -> &'static str { "minecraft:exposed_cut_copper_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21464u32 }
}

impl BlockTrait<V1_20_0> for BlockCutCopperSlab {
    fn resource_location(&self) -> &'static str { "minecraft:cut_copper_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21470u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedCopperBlock {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_copper_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21476u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedWeatheredCopper {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_weathered_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21477u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedExposedCopper {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_exposed_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21478u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedOxidizedCopper {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_oxidized_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21479u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedOxidizedCutCopper {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_oxidized_cut_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21480u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedWeatheredCutCopper {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_weathered_cut_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21481u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedExposedCutCopper {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_exposed_cut_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21482u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedCutCopper {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_cut_copper" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21483u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedOxidizedCutCopperStairs {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_oxidized_cut_copper_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21484u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedWeatheredCutCopperStairs {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_weathered_cut_copper_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21564u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedExposedCutCopperStairs {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_exposed_cut_copper_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21644u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedCutCopperStairs {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_cut_copper_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21724u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedOxidizedCutCopperSlab {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_oxidized_cut_copper_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21804u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedWeatheredCutCopperSlab {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_weathered_cut_copper_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21810u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedExposedCutCopperSlab {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_exposed_cut_copper_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21816u32 }
}

impl BlockTrait<V1_20_0> for BlockWaxedCutCopperSlab {
    fn resource_location(&self) -> &'static str { "minecraft:waxed_cut_copper_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21822u32 }
}

impl BlockTrait<V1_20_0> for BlockLightningRod {
    fn resource_location(&self) -> &'static str { "minecraft:lightning_rod" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21828u32 }
}

impl BlockTrait<V1_20_0> for BlockPointedDripstone {
    fn resource_location(&self) -> &'static str { "minecraft:pointed_dripstone" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21852u32 }
}

impl BlockTrait<V1_20_0> for BlockDripstoneBlock {
    fn resource_location(&self) -> &'static str { "minecraft:dripstone_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21872u32 }
}

impl BlockTrait<V1_20_0> for BlockCaveVines {
    fn resource_location(&self) -> &'static str { "minecraft:cave_vines" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21873u32 }
}

impl BlockTrait<V1_20_0> for BlockCaveVinesPlant {
    fn resource_location(&self) -> &'static str { "minecraft:cave_vines_plant" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21925u32 }
}

impl BlockTrait<V1_20_0> for BlockSporeBlossom {
    fn resource_location(&self) -> &'static str { "minecraft:spore_blossom" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21927u32 }
}

impl BlockTrait<V1_20_0> for BlockAzalea {
    fn resource_location(&self) -> &'static str { "minecraft:azalea" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21928u32 }
}

impl BlockTrait<V1_20_0> for BlockFloweringAzalea {
    fn resource_location(&self) -> &'static str { "minecraft:flowering_azalea" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21929u32 }
}

impl BlockTrait<V1_20_0> for BlockMossCarpet {
    fn resource_location(&self) -> &'static str { "minecraft:moss_carpet" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21930u32 }
}

impl BlockTrait<V1_20_0> for BlockPinkPetals {
    fn resource_location(&self) -> &'static str { "minecraft:pink_petals" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21931u32 }
}

impl BlockTrait<V1_20_0> for BlockMossBlock {
    fn resource_location(&self) -> &'static str { "minecraft:moss_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 21947u32 }
}

impl BlockTrait<V1_20_0> for BlockBigDripleaf {
    fn resource_location(&self) -> &'static str { "minecraft:big_dripleaf" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21948u32 }
}

impl BlockTrait<V1_20_0> for BlockBigDripleafStem {
    fn resource_location(&self) -> &'static str { "minecraft:big_dripleaf_stem" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21980u32 }
}

impl BlockTrait<V1_20_0> for BlockSmallDripleaf {
    fn resource_location(&self) -> &'static str { "minecraft:small_dripleaf" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 21988u32 }
}

impl BlockTrait<V1_20_0> for BlockHangingRoots {
    fn resource_location(&self) -> &'static str { "minecraft:hanging_roots" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 22004u32 }
}

impl BlockTrait<V1_20_0> for BlockRootedDirt {
    fn resource_location(&self) -> &'static str { "minecraft:rooted_dirt" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 22006u32 }
}

impl BlockTrait<V1_20_0> for BlockMud {
    fn resource_location(&self) -> &'static str { "minecraft:mud" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 22007u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslate {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 22008u32 }
}

impl BlockTrait<V1_20_0> for BlockCobbledDeepslate {
    fn resource_location(&self) -> &'static str { "minecraft:cobbled_deepslate" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 22011u32 }
}

impl BlockTrait<V1_20_0> for BlockCobbledDeepslateStairs {
    fn resource_location(&self) -> &'static str { "minecraft:cobbled_deepslate_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 22012u32 }
}

impl BlockTrait<V1_20_0> for BlockCobbledDeepslateSlab {
    fn resource_location(&self) -> &'static str { "minecraft:cobbled_deepslate_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 22092u32 }
}

impl BlockTrait<V1_20_0> for BlockCobbledDeepslateWall {
    fn resource_location(&self) -> &'static str { "minecraft:cobbled_deepslate_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 22098u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedDeepslate {
    fn resource_location(&self) -> &'static str { "minecraft:polished_deepslate" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 22422u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedDeepslateStairs {
    fn resource_location(&self) -> &'static str { "minecraft:polished_deepslate_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 22423u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedDeepslateSlab {
    fn resource_location(&self) -> &'static str { "minecraft:polished_deepslate_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 22503u32 }
}

impl BlockTrait<V1_20_0> for BlockPolishedDeepslateWall {
    fn resource_location(&self) -> &'static str { "minecraft:polished_deepslate_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 22509u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateTiles {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_tiles" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 22833u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateTileStairs {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_tile_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 22834u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateTileSlab {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_tile_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 22914u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateTileWall {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_tile_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 22920u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateBricks {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23244u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateBrickStairs {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_brick_stairs" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 23245u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateBrickSlab {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_brick_slab" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 23325u32 }
}

impl BlockTrait<V1_20_0> for BlockDeepslateBrickWall {
    fn resource_location(&self) -> &'static str { "minecraft:deepslate_brick_wall" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 23331u32 }
}

impl BlockTrait<V1_20_0> for BlockChiseledDeepslate {
    fn resource_location(&self) -> &'static str { "minecraft:chiseled_deepslate" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23655u32 }
}

impl BlockTrait<V1_20_0> for BlockCrackedDeepslateBricks {
    fn resource_location(&self) -> &'static str { "minecraft:cracked_deepslate_bricks" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23656u32 }
}

impl BlockTrait<V1_20_0> for BlockCrackedDeepslateTiles {
    fn resource_location(&self) -> &'static str { "minecraft:cracked_deepslate_tiles" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23657u32 }
}

impl BlockTrait<V1_20_0> for BlockInfestedDeepslate {
    fn resource_location(&self) -> &'static str { "minecraft:infested_deepslate" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23658u32 }
}

impl BlockTrait<V1_20_0> for BlockSmoothBasalt {
    fn resource_location(&self) -> &'static str { "minecraft:smooth_basalt" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23659u32 }
}

impl BlockTrait<V1_20_0> for BlockRawIronBlock {
    fn resource_location(&self) -> &'static str { "minecraft:raw_iron_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23660u32 }
}

impl BlockTrait<V1_20_0> for BlockRawCopperBlock {
    fn resource_location(&self) -> &'static str { "minecraft:raw_copper_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23661u32 }
}

impl BlockTrait<V1_20_0> for BlockRawGoldBlock {
    fn resource_location(&self) -> &'static str { "minecraft:raw_gold_block" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23662u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedAzaleaBush {
    fn resource_location(&self) -> &'static str { "minecraft:potted_azalea_bush" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23663u32 }
}

impl BlockTrait<V1_20_0> for BlockPottedFloweringAzaleaBush {
    fn resource_location(&self) -> &'static str { "minecraft:potted_flowering_azalea_bush" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23664u32 }
}

impl BlockTrait<V1_20_0> for BlockOchreFroglight {
    fn resource_location(&self) -> &'static str { "minecraft:ochre_froglight" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 23665u32 }
}

impl BlockTrait<V1_20_0> for BlockVerdantFroglight {
    fn resource_location(&self) -> &'static str { "minecraft:verdant_froglight" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 23668u32 }
}

impl BlockTrait<V1_20_0> for BlockPearlescentFroglight {
    fn resource_location(&self) -> &'static str { "minecraft:pearlescent_froglight" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 23671u32 }
}

impl BlockTrait<V1_20_0> for BlockFrogspawn {
    fn resource_location(&self) -> &'static str { "minecraft:frogspawn" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23674u32 }
}

impl BlockTrait<V1_20_0> for BlockReinforcedDeepslate {
    fn resource_location(&self) -> &'static str { "minecraft:reinforced_deepslate" }
    fn try_from_u32(_: u32) -> Option<Self> { Some(Self) }
    fn to_u32(&self) -> u32 { 23675u32 }
}

impl BlockTrait<V1_20_0> for BlockDecoratedPot {
    fn resource_location(&self) -> &'static str { "minecraft:decorated_pot" }
    fn try_from_u32(_id: u32) -> Option<Self> { Some(Self::default()) }
    fn to_u32(&self) -> u32 { 23676u32 }
}
