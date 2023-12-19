use classfile::ast::{GetFieldInsn, Insn, InvokeInsn, LdcInsn, LdcType, PutFieldInsn};
use json::JsonValue;
use strum::Display;
use tracing::{error, info, warn};

use crate::data::ModuleData;

use crate::modules::ModuleExt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockListModule;

impl BlockListModule {
    pub const CLASS_PATH: &'static str = "net/minecraft/class_2246";
    pub const CLASS_METHOD: &'static str = "<clinit>";

    pub const BLOCK_CLASS: &'static str = "net/minecraft/class_2248";
}

impl ModuleExt for BlockListModule {
    fn run(&self, data: &mut ModuleData) {
        let Some(class) = data.classmap.get_mut(Self::CLASS_PATH) else {
            error!("Could not find class {}", Self::CLASS_PATH);
            return;
        };

        let Some(method) = class
            .methods
            .iter_mut()
            .find(|method| method.name == Self::CLASS_METHOD)
        else {
            error!(
                "Could not find method {} in class {}",
                Self::CLASS_METHOD,
                Self::CLASS_PATH
            );
            return;
        };

        let Some(code) = method.code() else {
            error!(
                "Could not get code for method {} in class {}",
                Self::CLASS_METHOD,
                Self::CLASS_PATH
            );
            return;
        };

        let mut block_list = Vec::default();
        let mut insn_list = Vec::new();

        // Group insns for easier parsing
        for insn in code.insns.iter() {
            insn_list.push(insn.clone());

            if let Insn::PutField(field) = insn {
                if field.class == Self::CLASS_PATH
                    && field.descriptor == format!("L{};", Self::BLOCK_CLASS)
                {
                    let block = BlockInsns(std::mem::take(&mut insn_list));
                    block_list.push(block);
                }
            }
        }

        info!("Parsing block list...");

        // Parse blocks, passing in the list if the block copies from another block
        let mut blocks = Vec::with_capacity(block_list.len());
        for block in block_list {
            let block = block.parse(&blocks);
            blocks.push(block);
        }

        info!("Found {} blocks!", blocks.len());

        // Generate JSON
        let mut json_list = Vec::with_capacity(blocks.len());
        for (index, block) in blocks.into_iter().enumerate() {
            if &block.name == "unknown" || &block.field == "unknown" {
                warn!("Unknown block at index {index}");
            }

            data.output["blocks"]["field_map"][block.field.clone()] = block.name.clone().into();
            json_list.push(block.name.clone().into());

            data.output["blocks"]["data"][block.name.clone()] = json::object! {
                "id": index,
                // "block_type": block.block_type,
                // "map_color": block.map_color.value().1,
                "collidable": block.collidable,
                // "luminance": block.luminance,
                "resistance": block.resistance,
                "hardness": block.hardness,
                "tool_required": block.tool_required,
                "random_ticks": block.random_ticks,
                "slipperiness": block.slipperiness,
                "velocity_multiplier": block.velocity_multiplier,
                "jump_velocity_multiplier": block.jump_velocity_multiplier,
                // "loot_table_id": block.loot_table_id,
                "opaque": block.opaque,
                "is_air": block.is_air,
                "burnable": block.burnable,
                "liquid": block.liquid,
                "force_not_solid": block.force_not_solid,
                "force_solid": block.force_solid,
                "piston_behavior": block.piston_behavior,
                "block_break_particles": block.block_break_particles,
                // "instrument": block.instrument,
                "replaceable": block.replaceable,
                "dynamic_bounds": block.dynamic_bounds,
            };
        }
        data.output["blocks"]["list"] = JsonValue::Array(json_list);
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct BlockInsns(Vec<Insn>);

impl BlockInsns {
    fn parse(self, blocks: &[Block]) -> Block {
        let mut block = Block::default();

        let mut float_storage: Vec<f32> = Vec::with_capacity(2);
        let mut field_storage: Vec<String> = Vec::with_capacity(2);

        for (index, insn) in self.0.into_iter().enumerate() {
            if index == 0 {
                if let Insn::Ldc(LdcInsn {
                    constant: LdcType::String(constant),
                }) = insn
                {
                    block.name = constant;
                    continue;
                } else {
                    error!("Could not get name for block");
                    continue;
                }
            }

            match insn {
                // Store constants for later use
                Insn::Ldc(LdcInsn {
                    constant: LdcType::Float(constant),
                }) => float_storage.push(constant),
                // Store fields for later use
                Insn::GetField(GetFieldInsn { name, .. }) => field_storage.push(name),
                // Set the block field
                Insn::PutField(PutFieldInsn { name, .. }) => {
                    block.field = name;
                }
                // Match invoke methods to Block/Blocks/Extended methods
                Insn::Invoke(InvokeInsn { name, class, .. }) => match name.as_str() {
                    // --- Block class methods ---
                    //
                    "method_9618" => {
                        block.break_instantly();
                    }
                    "method_9624" => {
                        block.dynamic_bounds();
                    }
                    "method_9626" => {
                        // block.sound_group()
                    }
                    "method_9628" => {
                        let Some(slipperiness) = float_storage.pop() else {
                            error!("Could not get slipperiness constant for method_9628");
                            continue;
                        };

                        block.slipperiness(slipperiness);
                    }
                    "method_9629" => {
                        let Some(resistance) = float_storage.pop() else {
                            error!("Could not get resistance constant for method_9629");
                            continue;
                        };

                        let Some(hardness) = float_storage.pop() else {
                            error!("Could not get hardness constant for method_9629");
                            continue;
                        };

                        block.strength_hardness(hardness, resistance)
                    }
                    "method_9630" => {
                        let Some(field) = field_storage.pop() else {
                            error!("Could not get field name for method_9630");
                            continue;
                        };

                        let Some(other) = blocks.iter().find(|other| other.field == field) else {
                            error!("Could not find block with field name {}", field);
                            continue;
                        };

                        block.copy(other);
                    }
                    "method_9631" => {
                        // block.luminance()
                    }
                    "method_9632" => {
                        let Some(strength) = float_storage.pop() else {
                            error!("Could not get strength constant for method_9632");
                            continue;
                        };

                        block.strength(strength);
                    }
                    "method_9634" => {
                        block.no_collision();
                    }
                    "method_9640" => {
                        block.ticks_randomly();
                    }
                    "method_22488" => {
                        block.non_opaque();
                    }
                    "method_23351" => {
                        let Some(velocity_multiplier) = float_storage.pop() else {
                            error!("Could not get velocity_multiplier constant for method_23351");
                            continue;
                        };

                        block.velocity_multiplier(velocity_multiplier);
                    }
                    "method_23352" => {
                        let Some(jump_velocity_multiplier) = float_storage.pop() else {
                            error!(
                                "Could not get jump_velocity_multiplier constant for method_23352"
                            );
                            continue;
                        };

                        block.jump_velocity_multiplier(jump_velocity_multiplier);
                    }
                    "method_26250" => {
                        block.air();
                    }
                    "method_29292" => {
                        block.requires_tool();
                    }
                    "method_31710" => {
                        // block.map_color()
                    }
                    "method_36557" => {
                        let Some(hardness) = float_storage.pop() else {
                            error!("Could not get hardness constant for method_36557");
                            continue;
                        };

                        block.hardness(hardness);
                    }
                    "method_36558" => {
                        let Some(resistance) = float_storage.pop() else {
                            error!("Could not get resistance constant for method_36558");
                            continue;
                        };

                        block.resistance(resistance);
                    }
                    // dropsNothing()
                    "method_42327" => {}
                    "method_45477" => {
                        block.no_block_break_particles();
                    }
                    "method_50012" => {
                        // block.piston_behavior()
                    }
                    "method_50013" => {
                        block.burnable();
                    }
                    "method_51177" => {
                        block.liquid();
                    }
                    "method_51368" => {
                        // block.instrument()
                    }
                    "method_51369" => {
                        block.solid();
                    }
                    "method_51370" => {
                        block.not_solid();
                    }
                    "method_51371" => {
                        block.replaceable();
                    }
                    //
                    // --- Extended Class methods ---
                    //
                    // LeavesBlock
                    "method_26106" => {
                        block.block_type = BlockType::Leaves;
                        block.strength(0.2);
                        block.ticks_randomly();
                        // block.sound_group()
                        block.non_opaque();
                        block.burnable();
                        block.piston_behavior(PistonBehavior::Destroy);
                    }
                    // BedBlock
                    "method_26109" => {
                        block.block_type = BlockType::Bed;
                        // block.sound_group()
                        block.strength(0.2);
                        block.non_opaque();
                        block.burnable();
                        block.piston_behavior(PistonBehavior::Destroy);
                    }
                    // ShulkerBoxBlock
                    "method_26110" => {
                        block.block_type = BlockType::ShulkerBox;
                        block.solid();
                        block.strength(2.0);
                        block.dynamic_bounds();
                        block.non_opaque();
                        block.piston_behavior(PistonBehavior::Destroy);
                    }
                    // NetherStemBlock
                    "method_26115" => {
                        block.block_type = BlockType::NetherStem;
                        block.strength(2.0);
                        // block.sound_group()
                    }
                    // LogBlock
                    "method_26117" => {
                        block.block_type = BlockType::Log;
                        block.strength(2.0);
                        // block.sound_group()
                        block.burnable();
                    }
                    // PistonBlock
                    "method_26119" => {
                        block.block_type = BlockType::Piston;
                        block.strength(1.5);
                        block.piston_behavior(PistonBehavior::Block);
                    }
                    // StainedGlassBlock
                    "method_26120" => {
                        block.block_type = BlockType::StainedGlass;
                        // block.instrument()
                        block.strength(0.3);
                        // block.sound_group()
                        block.non_opaque();
                    }
                    // WoodenButtonBlock
                    "method_45451" => {
                        block.block_type = BlockType::WoodenButton;
                        block.no_collision();
                        block.strength(0.5);
                        block.piston_behavior(PistonBehavior::Destroy);
                    }
                    // StoneButtonBlock
                    "method_45453" => {
                        block.block_type = BlockType::StoneButton;
                        block.no_collision();
                        block.strength(0.5);
                        block.piston_behavior(PistonBehavior::Destroy);
                    }
                    // BambooBlock
                    "method_47375" => {
                        block.block_type = BlockType::Bamboo;
                        block.strength(2.0);
                        block.burnable();
                    }
                    // FlowerPotBlock 
                    "method_50000" => {
                        block.block_type = BlockType::FlowerPot;
                        block.break_instantly();
                        block.non_opaque();
                        block.piston_behavior(PistonBehavior::Destroy);
                    }
                    // CandleBlock
                    "method_50001" => {
                        block.block_type = BlockType::Candle;
                        block.non_opaque();
                        block.strength(0.1);
                        // block.sound_group()
                        block.piston_behavior(PistonBehavior::Destroy);
                    }
                    //
                    // --- Ignored methods ---
                    //
                    // register()
                    "method_9492"
                    // get_default_state()
                    | "method_9564"
                    // create()
                    | "method_9637"
                    // create_light_level_from_lit_block_state()
                    | "method_26107"
                    // allows_spawning()
                    | "method_26235"
                    // get_default_map_color()
                    | "method_26403"
                    // offset()
                    | "method_49229"
                    // <init>
                    | "<init>" => {}
                    _ => {
                        if matches!(class.as_str(), BlockListModule::CLASS_PATH | BlockListModule::BLOCK_CLASS) {
                            warn!("Unknown invoke method: `{class}` :: `{name}`");
                        }
                    }
                },
                _ => {}
            }
        }

        block
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Block {
    name: String,
    field: String,
    block_type: BlockType,

    map_color: MapColor,
    collidable: bool,
    sound_group: SoundGroup,
    // luminance: u8,
    resistance: f32,
    hardness: f32,
    tool_required: bool,
    random_ticks: bool,
    slipperiness: f32,
    velocity_multiplier: f32,
    jump_velocity_multiplier: f32,
    loot_table_id: Option<String>,
    opaque: bool,
    is_air: bool,
    burnable: bool,
    // @Deprecated
    liquid: bool,
    // @Deprecated
    force_not_solid: bool,
    force_solid: bool,
    piston_behavior: PistonBehavior,
    block_break_particles: bool,
    instrument: Instrument,
    replaceable: bool,
    dynamic_bounds: bool,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            name: String::from("unknown"),
            field: String::from("unknown"),
            block_type: BlockType::Block,

            map_color: MapColor::Clear,
            collidable: true,
            sound_group: SoundGroup::Stone,
            // luminance: 0,
            resistance: 0.0f32,
            hardness: 0.0f32,
            tool_required: false,
            random_ticks: false,
            slipperiness: 0.6f32,
            velocity_multiplier: 1.0f32,
            jump_velocity_multiplier: 1.0f32,
            loot_table_id: None,
            opaque: true,
            is_air: false,
            burnable: false,
            liquid: false,
            force_not_solid: false,
            force_solid: false,
            piston_behavior: PistonBehavior::Normal,
            block_break_particles: true,
            instrument: Instrument::Harp,
            replaceable: false,
            dynamic_bounds: false,
        }
    }
}

impl Block {
    fn no_collision(&mut self) {
        self.collidable = false;
        self.opaque = false;
    }

    fn non_opaque(&mut self) { self.opaque = false; }

    fn slipperiness(&mut self, slipperiness: f32) { self.slipperiness = slipperiness; }

    fn velocity_multiplier(&mut self, velocity_multiplier: f32) {
        self.velocity_multiplier = velocity_multiplier;
    }

    fn jump_velocity_multiplier(&mut self, jump_velocity_multiplier: f32) {
        self.jump_velocity_multiplier = jump_velocity_multiplier;
    }

    // fn luminance(&mut self, luminance: u8) { self.luminance = luminance; }

    fn strength_hardness(&mut self, hardness: f32, resistance: f32) {
        self.hardness = hardness;
        self.resistance = resistance;
    }

    fn break_instantly(&mut self) { self.strength_hardness(0.0, 0.0); }

    fn strength(&mut self, strength: f32) { self.strength_hardness(strength, strength) }

    fn ticks_randomly(&mut self) { self.random_ticks = true; }

    fn dynamic_bounds(&mut self) { self.dynamic_bounds = true; }

    fn burnable(&mut self) { self.burnable = true; }

    fn liquid(&mut self) { self.liquid = true; }

    fn solid(&mut self) { self.force_solid = true; }

    fn not_solid(&mut self) { self.force_not_solid = true; }

    fn piston_behavior(&mut self, piston_behavior: PistonBehavior) {
        self.piston_behavior = piston_behavior;
    }

    fn air(&mut self) { self.is_air = true; }

    fn requires_tool(&mut self) { self.tool_required = true; }

    fn hardness(&mut self, hardness: f32) { self.hardness = hardness; }

    fn resistance(&mut self, resistance: f32) { self.resistance = resistance; }

    fn no_block_break_particles(&mut self) { self.block_break_particles = false; }

    #[allow(dead_code)]
    fn instrument(&mut self, instrument: Instrument) { self.instrument = instrument; }

    fn replaceable(&mut self) { self.replaceable = true; }

    fn copy(&mut self, other: &Block) {
        self.hardness = other.hardness;
        self.resistance = other.resistance;
        self.collidable = other.collidable;
        self.random_ticks = other.random_ticks;
        // self.luminance = other.luminance;
        self.sound_group = other.sound_group;
        self.slipperiness = other.slipperiness;
        self.velocity_multiplier = other.velocity_multiplier;
        self.dynamic_bounds = other.dynamic_bounds;
        self.opaque = other.opaque;
        self.is_air = other.is_air;
        self.burnable = other.burnable;
        self.liquid = other.liquid;
        self.force_not_solid = other.force_not_solid;
        self.force_solid = other.force_solid;
        self.piston_behavior = other.piston_behavior;
        self.tool_required = other.tool_required;
        self.block_break_particles = other.block_break_particles;
        self.instrument = other.instrument;
        self.replaceable = other.replaceable;
    }
}

#[derive(Debug, Display, Default, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub(crate) enum BlockType {
    #[default]
    Block,
    Leaves,
    Bed,
    ShulkerBox,
    NetherStem,
    Log,
    Piston,
    StainedGlass,
    WoodenButton,
    StoneButton,
    Bamboo,
    FlowerPot,
    Candle,
}

impl From<BlockType> for JsonValue {
    fn from(value: BlockType) -> Self { JsonValue::String(value.to_string()) }
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
#[allow(dead_code)]
pub(crate) enum PistonBehavior {
    Normal,
    Destroy,
    Block,
    Ignore,
    PushOnly,
}

impl From<PistonBehavior> for JsonValue {
    fn from(value: PistonBehavior) -> Self { JsonValue::String(value.to_string()) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SoundGroup {
    Stone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Instrument {
    Harp,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MapColor {
    Clear,
    PaleGreen,
    PaleYellow,
    WhiteGray,
    BrightRed,
    PalePurple,
    IronGray,
    DarkGreen,
    White,
    LightBlueGray,
    DirtBrown,
    StoneGray,
    WaterBlue,
    OakTan,
    OffWhite,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
    Gold,
    DiamondBlue,
    LapisBlue,
    EmeraldGreen,
    SpruceBrown,
    DarkRed,
    TerracottaWhite,
    TerracottaOrange,
    TerracottaMagenta,
    TerracottaLightBlue,
    TerracottaYellow,
    TerracottaLime,
    TerracottaPink,
    TerracottaGray,
    TerracottaLightGray,
    TerracottaCyan,
    TerracottaPurple,
    TerracottaBlue,
    TerracottaBrown,
    TerracottaGreen,
    TerracottaRed,
    TerracottaBlack,
    DullRed,
    DullPink,
    DarkCrimson,
    Teal,
    DarkAqua,
    DarkDullPink,
    BrightTeal,
    DeepslateGray,
    RawIronPink,
    LichenGreen,
}

#[allow(dead_code)]
impl MapColor {
    fn value(&self) -> (u32, u32) {
        match self {
            MapColor::Clear => (0, 0),
            MapColor::PaleGreen => (1, 8368696),
            MapColor::PaleYellow => (2, 16247203),
            MapColor::WhiteGray => (3, 0xC7C7C7),
            MapColor::BrightRed => (4, 0xFF0000),
            MapColor::PalePurple => (5, 0xA0A0FF),
            MapColor::IronGray => (6, 0xA7A7A7),
            MapColor::DarkGreen => (7, 31744),
            MapColor::White => (8, 0xFFFFFF),
            MapColor::LightBlueGray => (9, 10791096),
            MapColor::DirtBrown => (10, 9923917),
            MapColor::StoneGray => (11, 0x707070),
            MapColor::WaterBlue => (12, 0x4040FF),
            MapColor::OakTan => (13, 9402184),
            MapColor::OffWhite => (14, 0xFFFCF5),
            MapColor::Orange => (15, 14188339),
            MapColor::Magenta => (16, 11685080),
            MapColor::LightBlue => (17, 6724056),
            MapColor::Yellow => (18, 0xE5E533),
            MapColor::Lime => (19, 8375321),
            MapColor::Pink => (20, 15892389),
            MapColor::Gray => (21, 0x4C4C4C),
            MapColor::LightGray => (22, 0x999999),
            MapColor::Cyan => (23, 5013401),
            MapColor::Purple => (24, 8339378),
            MapColor::Blue => (25, 3361970),
            MapColor::Brown => (26, 6704179),
            MapColor::Green => (27, 6717235),
            MapColor::Red => (28, 0x993333),
            MapColor::Black => (29, 0x191919),
            MapColor::Gold => (30, 16445005),
            MapColor::DiamondBlue => (31, 6085589),
            MapColor::LapisBlue => (32, 4882687),
            MapColor::EmeraldGreen => (33, 55610),
            MapColor::SpruceBrown => (34, 8476209),
            MapColor::DarkRed => (35, 0x700200),
            MapColor::TerracottaWhite => (36, 13742497),
            MapColor::TerracottaOrange => (37, 10441252),
            MapColor::TerracottaMagenta => (38, 9787244),
            MapColor::TerracottaLightBlue => (39, 7367818),
            MapColor::TerracottaYellow => (40, 12223780),
            MapColor::TerracottaLime => (41, 6780213),
            MapColor::TerracottaPink => (42, 10505550),
            MapColor::TerracottaGray => (43, 0x392923),
            MapColor::TerracottaLightGray => (44, 8874850),
            MapColor::TerracottaCyan => (45, 0x575C5C),
            MapColor::TerracottaPurple => (46, 8014168),
            MapColor::TerracottaBlue => (47, 4996700),
            MapColor::TerracottaBrown => (48, 4993571),
            MapColor::TerracottaGreen => (49, 5001770),
            MapColor::TerracottaRed => (50, 9321518),
            MapColor::TerracottaBlack => (51, 2430480),
            MapColor::DullRed => (52, 12398641),
            MapColor::DullPink => (53, 9715553),
            MapColor::DarkCrimson => (54, 6035741),
            MapColor::Teal => (55, 1474182),
            MapColor::DarkAqua => (56, 3837580),
            MapColor::DarkDullPink => (57, 5647422),
            MapColor::BrightTeal => (58, 1356933),
            MapColor::DeepslateGray => (59, 0x646464),
            MapColor::RawIronPink => (60, 14200723),
            MapColor::LichenGreen => (61, 8365974),
        }
    }
}
