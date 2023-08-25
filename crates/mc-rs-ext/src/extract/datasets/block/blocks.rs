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
        let Some(class) = classmap.get(Self::CLASS) else {
            error!("Could not find class {}", Self::CLASS);
            return;
        };

        let Some(method) = class.methods.iter().find(|&m| m.name == Self::METHOD) else {
            error!("Could not find method {}", Self::METHOD);
            return;
        };
        let mut method = method.clone();

        let Some(code) = method.code() else {
            error!("Could not get code for method {}", Self::METHOD);
            return;
        };
        let insns = &code.insns.insns;

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
                    // Reset hardness and resistance
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
                    // Fluid?
                    "method_9634" => {
                        block.is_fluid = true;
                        block.collidable = false;
                        block.opaque = false;
                    }
                    // Growable?
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
                    // Transparent?
                    "method_22488" => {
                        block.opaque = false;
                    }
                    // Air
                    "method_25250" => {
                        block.is_air = true;
                    }
                    // 1.20 - Solid block
                    "method_51369" => {
                        block.collidable = true;
                    }
                    // 1.20 - Non-solid block
                    "method_51370" => {
                        block.collidable = false;
                    }

                    // Ignore these methods, we don't care about them
                    // TODO: Document what these methods do
                    "method_45476" | "method_9626" | "method_9492" | "method_9639"
                    | "method_9637" | "method_29292" | "method_9564" | "method_26243"
                    | "method_26236" | "method_26235" | "method_31710" | "method_49229"
                    | "method_37362" | "method_9624" | "method_26249" | "method_26245" | "method_9617" | "method_37364"
                    | "method_9631"  // Luminance
                                     // 1.19.4
                    | "method_42327" // Drops nothing
                    | "method_26117" // createLogBlock
                    | "method_47375" // createBambooBlock
                    | "method_26106" // createLeavesBlock
                    | "method_35017" // create?
                    | "method_26109" // createBedBlock
                    | "method_26119" // createPistonBlock
                    | "method_26107" // createLightLevelFromLitBlockState
                    | "method_26247" // postProcess
                    | "method_16228" // dropsLike
                    | "method_26162" // getLootTableId
                    | "method_26403" // getDefaultMapColor
                    | "method_32913" // getMaxHorizontalModelOffset
                    | "method_36555" // getHardness
                    | "method_37247" // getVerticalModelOffsetMultiplier
                    | "method_26110" // createShulkerBoxBlock
                    | "method_45451" // createWoodenButtonBlock
                    | "method_45453" // createStoneButtonBlock
                    | "method_26120" // createStainedGlassBlock
                    | "method_45477" // noBlockBreakParticles
                    | "method_26115" // createNetherStemBlock
                                     // 1.20.0
                    | "method_51177" // liquid
                    | "method_51368" // instrument
                    | "method_50012" // pistonBehavior
                    | "method_51371" // replaceable
                    | "method_51517" // mapColor
                    | "method_50001" // createCandleBlock
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
            data["blocks"]["fields"][key.clone()] = block.name.clone().into();
        });

        // Add the block name list
        data["blocks"]["list"] = block_names.into();

        // Add the block data
        blocks.values().for_each(|block| {
            data["blocks"]["blocks"][block.name.clone()] = object! {
                "id" => block.id,
                "hardness" => block.hardness,
                "resistance" => block.resistance,
                "friction" => block.friction,
                "velocity_multiplier" => block.velocity_multiplier,
                "jump_velocity_multiplier" => block.jump_velocity_multiplier,
                "random_ticks" => block.random_ticks,
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
            friction: 0.0,
            velocity_multiplier: 0.0,
            jump_velocity_multiplier: 0.0,
            random_ticks: false,
            collidable: true,
            opaque: true,
            is_air: false,
            is_fluid: false,
        }
    }
}
