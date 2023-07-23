use std::mem;

use classfile::ast::{Insn, LdcType};
use json::JsonValue;
use log::error;

use crate::types::{ClassMap, Manifest, Version};

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
                        } else {
                            material.name = s.clone();
                        }
                    }
                    LdcType::Int(i) => {
                        if material.name.is_empty() {
                            continue;
                        }

                        if material.durability_multiplier == 0 {
                            material.durability_multiplier = *i;
                            continue;
                        }

                        // match material.protection_amounts {
                        //     [0, 0, 0, 0] => {
                        //         material.protection_amounts[0] = *i;
                        //         continue;
                        //     }
                        //     [_, 0, 0, 0] => {
                        //         material.protection_amounts[1] = *i;
                        //         continue;
                        //     }
                        //     [_, _, 0, 0] => {
                        //         material.protection_amounts[2] = *i;
                        //         continue;
                        //     }
                        //     [_, _, _, 0] => {
                        //         material.protection_amounts[3] = *i;
                        //         continue;
                        //     }
                        //     _ => {}
                        // }

                        if material.enchantability == 0 {
                            material.enchantability = *i;
                            continue;
                        }
                    }
                    LdcType::Float(f) => {
                        if material.toughness == 0.0 {
                            material.toughness = *f;
                            continue;
                        }

                        if material.knockback_resistance == 0.0 {
                            material.knockback_resistance = *f;
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

        data["items"]["armor"]["types"] = JsonValue::Array(
            materials
                .iter()
                .map(|m| JsonValue::String(m.constant.clone()))
                .collect::<Vec<_>>(),
        );

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

#[derive(Debug, Default, Clone)]
struct Material {
    constant: String,
    name: String,
    durability_multiplier: i32,
    // protection_amounts: [i32; 4],
    enchantability: i32,
    // equip_sound: String,
    toughness: f32,
    knockback_resistance: f32,
    // repair_ingredient: String,
}
