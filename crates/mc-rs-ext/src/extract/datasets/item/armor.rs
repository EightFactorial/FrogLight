use std::mem;

use classfile::ast::{Insn, LdcType};
use json::JsonValue;
use log::error;

use crate::{
    extract::datasets::round_float,
    types::{ClassMap, Manifest, Version},
};

use crate::extract::{Dataset, Datasets};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Armor;

impl Dataset for Armor {
    fn min(&self) -> &'static Option<Version> {
        &Some(Version::Release {
            major: 1,
            minor: 19,
            patch: 0,
        })
    }

    fn deps(&self) -> &'static [Datasets] { &[] }

    fn parse(
        &self,
        _version: &Version,
        _manifest: &Manifest,
        classmap: &ClassMap,
        data: &mut JsonValue,
    ) {
        let Some(class) = classmap.get("net/minecraft/class_1740") else {
            error!("Failed to find ArmorMaterial class");
            return;
        };

        let Some(method) = class.methods.iter().find(|m| m.name == "<clinit>") else {
            error!("Failed to find ArmorMaterial.<clinit>");
            return;
        };

        let mut method = method.clone();
        let Some(code) = method.code() else {
            error!("Failed to find ArmorMaterial.<clinit> code");
            return;
        };

        let mut material = Material::default();
        let mut materials: Vec<Material> = Vec::new();

        for insn in code.insns.iter() {
            match &insn {
                Insn::Ldc(insn) => match &insn.constant {
                    LdcType::String(s) => {
                        if material.constant.is_empty() {
                            material.constant = s.clone();
                            continue;
                        }

                        if material.name.is_empty() {
                            material.name = s.clone();
                            continue;
                        }
                    }
                    LdcType::Int(i) => {
                        if material.name.is_empty() {
                            continue;
                        }

                        if material.durability_multiplier == i32::MIN {
                            material.durability_multiplier = *i;
                            continue;
                        }

                        // match material.protection_amounts {
                        //     [i32::MIN, i32::MIN, i32::MIN, i32::MIN] => {
                        //         material.protection_amounts[0] = *i;
                        //         continue;
                        //     }
                        //     [_, i32::MIN, i32::MIN, i32::MIN] => {
                        //         material.protection_amounts[1] = *i;
                        //         continue;
                        //     }
                        //     [_, _, i32::MIN, i32::MIN] => {
                        //         material.protection_amounts[2] = *i;
                        //         continue;
                        //     }
                        //     [_, _, _, i32::MIN] => {
                        //         material.protection_amounts[3] = *i;
                        //         continue;
                        //     }
                        //     _ => {}
                        // }

                        if material.enchantability == i32::MIN {
                            material.enchantability = *i;
                            continue;
                        }
                    }
                    LdcType::Float(f) => {
                        if material.toughness == f64::MIN {
                            material.toughness = round_float(*f as f64);
                            continue;
                        }

                        if material.knockback_resistance == f64::MIN {
                            material.knockback_resistance = round_float(*f as f64);
                            continue;
                        }
                    }
                    _ => {}
                },
                Insn::PutField(insn) => {
                    if insn.class == "net/minecraft/class_1740"
                        && insn.descriptor == "Lnet/minecraft/class_1740;"
                    {
                        materials.push(mem::take(&mut material));
                    }
                }
                _ => {}
            }
        }

        data["items"]["armor"]["types"] = materials
            .iter()
            .map(|m| m.constant.clone())
            .collect::<Vec<_>>()
            .into();

        for material in materials {
            data["items"]["armor"]["stats"][material.constant] = json::object! {
                "name": material.name,
                "durability_multiplier": material.durability_multiplier,
                "enchantability": material.enchantability,
                "toughness": material.toughness,
                "knockback_resistance": material.knockback_resistance,
            };
        }
    }
}

//     private ArmorMaterials(String name, int durabilityMultiplier, int[] protectionAmounts, int
// enchantability, SoundEvent equipSound, float toughness, float knockbackResistance,
// Supplier<Ingredient> repairIngredientSupplier) { .. }

#[derive(Debug, Clone)]
struct Material {
    constant: String,
    name: String,
    durability_multiplier: i32,
    // protection_amounts: [i32; 4],
    enchantability: i32,
    // equip_sound: String,
    toughness: f64,
    knockback_resistance: f64,
    // repair_ingredient: String,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            constant: Default::default(),
            name: Default::default(),
            durability_multiplier: i32::MIN,
            // protection_amounts: [i32::MIN; 4],
            enchantability: i32::MIN,
            // equip_sound: Default::default(),
            toughness: f64::MIN,
            knockback_resistance: f64::MIN,
            // repair_ingredient: Default::default(),
        }
    }
}
