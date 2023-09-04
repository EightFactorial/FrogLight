use std::collections::BTreeMap;

use classfile::ast::{GetFieldInsn, Insn, InvokeInsn, LdcInsn, LdcType, PutFieldInsn};
use json::{object, JsonValue};
use log::{debug, error};

use crate::types::{ClassMap, Manifest, Version};

use crate::extract::{Dataset, Datasets};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Blocks;

impl Blocks {
    pub const CLASS: &'static str = "net/minecraft/class_2246";
    pub const METHOD: &'static str = "<clinit>";
}

impl Dataset for Blocks {
    fn min(&self) -> &'static Option<Version> { &None }

    fn deps(&self) -> &'static [Datasets] { &[] }

    fn parse(
        &self,
        _version: &Version,
        _manifest: &Manifest,
        classmap: &ClassMap,
        data: &mut JsonValue,
    ) {
        let Some(insns) = Datasets::get_code(Self::METHOD, Self::CLASS, classmap) else {
            error!(
                "Could not get code for method {} in class {}",
                Self::METHOD,
                Self::CLASS
            );
            return;
        };

        let mut blocks = BTreeMap::new();
        let mut block_names = Vec::with_capacity(1024);

        let mut block = Block::default();
        for (index, insn) in insns.iter().enumerate() {
            match insn {
                Insn::PutField(PutFieldInsn { name, .. }) => {
                    block.id = blocks.len() as u32;

                    block_names.push(block.name.clone());
                    blocks.insert(name.to_string(), std::mem::take(&mut block));
                }
                Insn::Ldc(LdcInsn {
                    constant: LdcType::String(name),
                }) => {
                    block.name = name.to_string();
                }
                Insn::Invoke(InvokeInsn { name, .. }) => match name.as_str() {
                    // Copy block properties from another block
                    "method_9630" => {
                        if let Some(Insn::GetField(GetFieldInsn { name, .. })) =
                            insns.get(index - 1)
                        {
                            if let Some(other_block) = blocks.get(name) {
                                let name = block.name.clone();

                                block = other_block.clone();
                                block.name = name;
                            } else {
                                error!(
                                    "Unable to copy block properties from {name} for {}",
                                    block.name
                                );
                            }
                        } else {
                            error!("Unable to copy block properties for {}", block.name)
                        }
                    }
                    // Hardness and Resistance
                    "method_9629" => {
                        if let Some(Insn::Ldc(LdcInsn {
                            constant: LdcType::Float(resistance),
                        })) = insns.get(index - 1)
                        {
                            block.resistance = *resistance;
                        } else {
                            error!("Unable to get resistance for {}", block.name);
                        }

                        if let Some(Insn::Ldc(LdcInsn {
                            constant: LdcType::Float(hardness),
                        })) = insns.get(index - 2)
                        {
                            block.hardness = *hardness;
                        } else {
                            error!("Unable to get hardness for {}", block.name);
                        }
                    }
                    // Break instantly
                    "method_9618" => {
                        block.resistance = 0.0;
                        block.hardness = 0.0;
                    }
                    // Hardness
                    "method_9632" => {
                        if let Some(Insn::Ldc(LdcInsn {
                            constant: LdcType::Float(hardness),
                        })) = insns.get(index - 1)
                        {
                            block.hardness = *hardness;
                        } else {
                            error!("Unable to get hardness for {}", block.name);
                        }
                    }
                    // Friction (slipperiness)
                    "method_9628" => {
                        if let Some(Insn::Ldc(LdcInsn {
                            constant: LdcType::Float(friction),
                        })) = insns.get(index - 1)
                        {
                            block.friction = *friction;
                        } else {
                            error!("Unable to get friction for {}", block.name);
                        }
                    }
                    // No collision
                    "method_9634" => {
                        block.collidable = false;
                        block.opaque = false;
                    }
                    // Ticks randomly
                    "method_9640" => {
                        block.random_ticks = true;
                    }
                    // Velocity multiplier
                    "method_23351" => {
                        if let Some(Insn::Ldc(LdcInsn {
                            constant: LdcType::Float(velocity_multiplier),
                        })) = insns.get(index - 1)
                        {
                            block.velocity_multiplier = *velocity_multiplier;
                        } else {
                            error!("Unable to get velocity multiplier for {}", block.name);
                        }
                    }
                    // Jump velocity multiplier
                    "method_23352" => {
                        if let Some(Insn::Ldc(LdcInsn {
                            constant: LdcType::Float(jump_velocity_multiplier),
                        })) = insns.get(index - 1)
                        {
                            block.jump_velocity_multiplier = *jump_velocity_multiplier;
                        } else {
                            error!("Unable to get jump velocity multiplier for {}", block.name);
                        }
                    }
                    // Non-opaque
                    "method_22488" => {
                        block.opaque = false;
                    }
                    // Air
                    "method_26250" => {
                        block.is_air = true;
                    }
                    // Burnable
                    "method_50013" => {
                        block.burnable = true;
                    }
                    // Solid block
                    "method_51369" => {
                        block.collidable = true;
                    }
                    // Non-solid block
                    "method_51370" => {
                        block.collidable = false;
                    }
                    // Liquid
                    "method_51177" => {
                        block.is_fluid = true;
                    }
                     // createLogBlock
                    "method_26117" => {
                        block.hardness = 2.0;
                        block.resistance = 2.0;
                        block.burnable = true;
                    }
                    // createBambooBlock
                    "method_47375" => {
                        block.hardness = 2.0;
                        block.resistance = 2.0;
                        block.burnable = true;
                    }
                    // createLeavesBlock
                    "method_26106" => {
                        block.hardness = 0.2;
                        block.resistance = 0.2;
                        block.burnable = true;
                        block.opaque = false;
                        block.random_ticks = true;
                    }
                    // createBedBlock
                    "method_26109"  => {
                        block.hardness = 0.2;
                        block.resistance = 0.2;
                        block.opaque = false;
                    }
                    // createPistonBlock
                    "method_26119"  => {
                        block.hardness = 1.5;
                        block.resistance = 1.5;
                    }
                    // createShulkerBoxBlock
                    "method_26110" => {
                        block.hardness = 2.0;
                        block.resistance = 2.0;
                        block.opaque = false;
                    }
                    // createWoodenButtonBlock
                    "method_45451" => {
                        block.hardness = 0.5;
                        block.resistance = 0.5;
                        block.collidable = false;
                    }
                    // createStoneButtonBlock
                    "method_45453" => {
                        block.hardness = 0.5;
                        block.resistance = 0.5;
                        block.collidable = false;
                    }
                    // createStainedGlassBlock
                    "method_26120" => {
                        block.hardness = 0.3;
                        block.resistance = 0.3;
                        block.opaque = false;
                    }
                    // createFlowerPotBlock
                    "method_50000" => {
                        block.hardness = 0.0;
                        block.resistance = 0.0;
                        block.opaque = false;
                    }
                    // createCandleBlock
                    "method_50001" => {
                        block.hardness = 0.1;
                        block.resistance = 0.1;
                        block.opaque = false;
                    }

                    // Ignore these methods, we don't care about them
                    // TODO: Document what these methods do
                    "method_45476" | "method_9626" | "method_9492" | "method_9639"
                    | "method_9637" | "method_29292" | "method_9564" | "method_26243"
                    | "method_26236" | "method_26235" | "method_31710" | "method_49229"
                    | "method_37362" | "method_9624" | "method_26249" | "method_26245" | "method_9617" | "method_37364"
                    | "method_9595" | "method_11662" | "method_10205" | "method_26200"
                    | "method_9631"  // Luminance
                                     // 1.19.4
                    | "method_42327" // Drops nothing
                    | "method_35017" // create?
                    | "method_26107" // createLightLevelFromLitBlockState
                    | "method_26247" // postProcess
                    | "method_16228" // dropsLike
                    | "method_26162" // getLootTableId
                    | "method_26403" // getDefaultMapColor
                    | "method_32913" // getMaxHorizontalModelOffset
                    | "method_36555" // getHardness
                    | "method_37247" // getVerticalModelOffsetMultiplier
                    | "method_45477" // noBlockBreakParticles
                    | "method_26115" // createNetherStemBlock
                                     // 1.20.0
                    | "method_51368" // instrument
                    | "method_50012" // pistonBehavior
                    | "method_51371" // replaceable
                    | "method_51517" // mapColor
                    | "<init>" => {}
                    _ => {
                        if name.starts_with("method") {
                            debug!("Found unknown invoke getting block properties: {name}");
                        }
                    }
                },
                _ => {}
            }
        }

        // Add a field mapping for the block names
        blocks.iter().for_each(|(key, block)| {
            data["blocks"]["blocks"]["fields"][key.clone()] = block.name.clone().into();
        });

        // Add the block name list
        data["blocks"]["blocks"]["list"] = block_names.into();

        // Add the block data
        blocks.values().for_each(|block| {
            data["blocks"]["blocks"]["blocks"][block.name.clone()] = object! {
                "id" => block.id,
                "hardness" => Datasets::round_float_f32(block.hardness),
                "resistance" => Datasets::round_float_f32(block.resistance),
                "friction" => Datasets::round_float_f32(block.friction),
                "velocity_multiplier" => Datasets::round_float_f32(block.velocity_multiplier),
                "jump_velocity_multiplier" => Datasets::round_float_f32(block.jump_velocity_multiplier),
                "random_ticks" => block.random_ticks,
                "burnable" => block.burnable,
                "collidable" => block.collidable,
                "opaque" => block.opaque,
                "is_air" => block.is_air,
                "is_fluid" => block.is_fluid,
            };
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub name: String,
    pub id: u32,
    pub hardness: f32,
    pub resistance: f32,
    pub friction: f32,
    pub velocity_multiplier: f32,
    pub jump_velocity_multiplier: f32,
    pub random_ticks: bool,
    pub burnable: bool,
    pub collidable: bool,
    pub opaque: bool,
    pub is_air: bool,
    pub is_fluid: bool,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: u32::MAX,
            hardness: 0.0,
            resistance: 0.0,
            friction: 0.6,
            velocity_multiplier: 1.0,
            jump_velocity_multiplier: 1.0,
            random_ticks: false,
            burnable: false,
            collidable: true,
            opaque: true,
            is_air: false,
            is_fluid: false,
        }
    }
}
