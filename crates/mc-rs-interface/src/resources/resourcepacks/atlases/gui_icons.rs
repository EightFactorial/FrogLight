use bevy::math::{Rect, Vec2};
use mc_rs_core::ResourceLocation;

use crate::traits::interface::AtlasData;

use super::AtlasKind;

pub struct GuiIcons;

impl GuiIcons {
    pub const CROSSHAIR: usize = 1;

    pub const PLAYER_HEART_OUTLINE_0: usize = 2;
    pub const PLAYER_HEART_OUTLINE_1: usize = 3;
    pub const PLAYER_HEART_OUTLINE_2: usize = 4;
    pub const PLAYER_HEART_OUTLINE_3: usize = 5;

    pub const PLAYER_HEART_0_FULL: usize = 6;
    pub const PLAYER_HEART_0_HALF: usize = 7;
    pub const PLAYER_HEART_1_FULL: usize = 8;
    pub const PLAYER_HEART_1_HALF: usize = 9;
    pub const PLAYER_HEART_2_FULL: usize = 10;
    pub const PLAYER_HEART_2_HALF: usize = 11;
    pub const PLAYER_HEART_3_FULL: usize = 12;
    pub const PLAYER_HEART_3_HALF: usize = 13;
    pub const PLAYER_HEART_4_FULL: usize = 14;
    pub const PLAYER_HEART_4_HALF: usize = 15;
    pub const PLAYER_HEART_5_FULL: usize = 16;
    pub const PLAYER_HEART_5_HALF: usize = 17;
    pub const PLAYER_HEART_6_FULL: usize = 18;
    pub const PLAYER_HEART_6_HALF: usize = 19;
    pub const PLAYER_HEART_7_FULL: usize = 20;
    pub const PLAYER_HEART_7_HALF: usize = 21;

    pub const PLAYER_ARMOR_EMPTY: usize = 22;
    pub const PLAYER_ARMOR_HALF: usize = 23;
    pub const PLAYER_ARMOR_FULL_0: usize = 24;
    pub const PLAYER_ARMOR_FULL_1: usize = 25;

    pub const ANIMAL_HEART_OUTLINE_0: usize = 26;
    pub const ANIMAL_HEART_OUTLINE_1: usize = 27;
    pub const ANIMAL_HEART_OUTLINE_2: usize = 28;
    pub const ANIMAL_HEART_OUTLINE_3: usize = 29;

    pub const ANIMAL_HEART_0_FULL: usize = 30;
    pub const ANIMAL_HEART_0_HALF: usize = 31;
    pub const ANIMAL_HEART_1_FULL: usize = 32;
    pub const ANIMAL_HEART_1_HALF: usize = 33;

    pub const PLAYER_BREATH_BUBBLE: usize = 34;
    pub const PLAYER_BREATH_POP: usize = 35;
    // Huh?
    pub const PLAYER_ARMOR_WATER_HALF: usize = 36;
    pub const PLAYER_ARMOR_WATER: usize = 37;

    pub const PLAYER_FOOD_OUTLINE_0: usize = 38;
    pub const PLAYER_FOOD_OUTLINE_1: usize = 39;
    pub const PLAYER_FOOD_OUTLINE_2: usize = 40;
    pub const PLAYER_FOOD_OUTLINE_3: usize = 41;

    pub const PLAYER_FOOD_0_FULL: usize = 42;
    pub const PLAYER_FOOD_0_HALF: usize = 43;
    pub const PLAYER_FOOD_1_FULL: usize = 44;
    pub const PLAYER_FOOD_1_HALF: usize = 45;
    pub const PLAYER_FOOD_2_FULL: usize = 46;
    pub const PLAYER_FOOD_2_HALF: usize = 47;
    pub const PLAYER_FOOD_3_FULL: usize = 48;
    pub const PLAYER_FOOD_3_HALF: usize = 49;

    pub const PLAYER_FOOD_OUTLINE_4: usize = 50;
    pub const PLAYER_FOOD_OUTLINE_5: usize = 51;

    pub const PLAYER_FOOD_FLIPPED: usize = 52;

    pub const PLAYER_HARDCORE_HEART_OUTLINE_0: usize = 53;
    pub const PLAYER_HARDCORE_HEART_OUTLINE_1: usize = 54;
    pub const PLAYER_HARDCORE_HEART_OUTLINE_2: usize = 55;
    pub const PLAYER_HARDCORE_HEART_OUTLINE_3: usize = 56;

    pub const PLAYER_HARDCORE_HEART_0_FULL: usize = 57;
    pub const PLAYER_HARDCORE_HEART_0_HALF: usize = 58;
    pub const PLAYER_HARDCORE_HEART_1_FULL: usize = 59;
    pub const PLAYER_HARDCORE_HEART_1_HALF: usize = 60;
    pub const PLAYER_HARDCORE_HEART_2_FULL: usize = 61;
    pub const PLAYER_HARDCORE_HEART_2_HALF: usize = 62;
    pub const PLAYER_HARDCORE_HEART_3_FULL: usize = 63;
    pub const PLAYER_HARDCORE_HEART_3_HALF: usize = 64;
    pub const PLAYER_HARDCORE_HEART_4_FULL: usize = 65;
    pub const PLAYER_HARDCORE_HEART_4_HALF: usize = 66;
    pub const PLAYER_HARDCORE_HEART_5_FULL: usize = 67;
    pub const PLAYER_HARDCORE_HEART_5_HALF: usize = 68;
    pub const PLAYER_HARDCORE_HEART_6_FULL: usize = 69;
    pub const PLAYER_HARDCORE_HEART_6_HALF: usize = 70;
    pub const PLAYER_HARDCORE_HEART_7_FULL: usize = 71;
    pub const PLAYER_HARDCORE_HEART_7_HALF: usize = 72;

    pub const CONNECTION_STRENGTH_5: usize = 73;
    pub const CONNECTION_STRENGTH_4: usize = 74;
    pub const CONNECTION_STRENGTH_3: usize = 75;
    pub const CONNECTION_STRENGTH_2: usize = 76;
    pub const CONNECTION_STRENGTH_1: usize = 77;
    pub const CONNECTION_STRENGTH_0: usize = 78;

    pub const XP_BAR_BACKGROUND: usize = 79;
    pub const XP_BAR_FOREGROUND: usize = 80;

    pub const BOSS_BAR_BACKGROUND: usize = 81;
    pub const BOSS_BAR_FOREGROUND: usize = 82;

    pub const HORSE_JUMP_BAR_BACKGROUND: usize = 83;
    pub const HORSE_JUMP_BAR_FOREGROUND: usize = 84;

    pub const WEAPON_SWING_0: usize = 85;
    pub const WEAPON_SWING_1: usize = 86;

    pub const WEAPON_SWING_2: usize = 87;
    pub const WEAPON_SWING_3: usize = 88;
    pub const WEAPON_SWING_4: usize = 89;

    pub const SERVER_PING_STRENGTH_5: usize = 90;
    pub const SERVER_PING_STRENGTH_4: usize = 91;
    pub const SERVER_PING_STRENGTH_3: usize = 92;
    pub const SERVER_PING_STRENGTH_2: usize = 93;
    pub const SERVER_PING_STRENGTH_1: usize = 94;
    pub const SERVER_PING_STRENGTH_0: usize = 95;

    pub const SERVER_PING_ANIM_0: usize = 96;
    pub const SERVER_PING_ANIM_1: usize = 97;
    pub const SERVER_PING_ANIM_2: usize = 98;
    pub const SERVER_PING_ANIM_3: usize = 99;
    pub const SERVER_PING_ANIM_4: usize = 100;
}

impl AtlasData for GuiIcons {
    fn atlas_kind() -> AtlasKind { AtlasKind::GuiIcons }
    fn path() -> ResourceLocation { ResourceLocation::from("minecraft:gui/icons") }

    fn coords() -> Vec<Rect> {
        vec![
            // Crosshair
            Rect::from_corners(Vec2::new(0., 0.), Vec2::new(15., 15.)),
            // Player Heart Outline
            Rect::from_corners(Vec2::new(15., 0.), Vec2::new(25., 9.)),
            Rect::from_corners(Vec2::new(25., 0.), Vec2::new(34., 9.)),
            Rect::from_corners(Vec2::new(34., 0.), Vec2::new(43., 9.)),
            Rect::from_corners(Vec2::new(43., 0.), Vec2::new(52., 9.)),
            // Player Hearts
            Rect::from_corners(Vec2::new(52., 0.), Vec2::new(61., 9.)),
            Rect::from_corners(Vec2::new(61., 0.), Vec2::new(70., 9.)),
            Rect::from_corners(Vec2::new(70., 0.), Vec2::new(79., 9.)),
            Rect::from_corners(Vec2::new(79., 0.), Vec2::new(88., 9.)),
            Rect::from_corners(Vec2::new(88., 0.), Vec2::new(97., 9.)),
            Rect::from_corners(Vec2::new(97., 0.), Vec2::new(106., 9.)),
            Rect::from_corners(Vec2::new(106., 0.), Vec2::new(115., 9.)),
            Rect::from_corners(Vec2::new(115., 0.), Vec2::new(124., 9.)),
            Rect::from_corners(Vec2::new(124., 0.), Vec2::new(133., 9.)),
            Rect::from_corners(Vec2::new(133., 0.), Vec2::new(142., 9.)),
            Rect::from_corners(Vec2::new(142., 0.), Vec2::new(151., 9.)),
            Rect::from_corners(Vec2::new(151., 0.), Vec2::new(160., 9.)),
            Rect::from_corners(Vec2::new(160., 0.), Vec2::new(169., 9.)),
            Rect::from_corners(Vec2::new(169., 0.), Vec2::new(178., 9.)),
            Rect::from_corners(Vec2::new(178., 0.), Vec2::new(187., 9.)),
            Rect::from_corners(Vec2::new(187., 0.), Vec2::new(196., 9.)),
            // Player Armor
            Rect::from_corners(Vec2::new(15., 9.), Vec2::new(25., 18.)),
            Rect::from_corners(Vec2::new(25., 9.), Vec2::new(34., 18.)),
            Rect::from_corners(Vec2::new(34., 9.), Vec2::new(43., 18.)),
            Rect::from_corners(Vec2::new(43., 9.), Vec2::new(52., 18.)),
            // Animal Heart Outline
            Rect::from_corners(Vec2::new(52., 9.), Vec2::new(61., 18.)),
            Rect::from_corners(Vec2::new(61., 9.), Vec2::new(70., 18.)),
            Rect::from_corners(Vec2::new(70., 9.), Vec2::new(79., 18.)),
            Rect::from_corners(Vec2::new(79., 9.), Vec2::new(88., 18.)),
            // Animal Hearts
            Rect::from_corners(Vec2::new(88., 9.), Vec2::new(97., 18.)),
            Rect::from_corners(Vec2::new(97., 9.), Vec2::new(106., 18.)),
            Rect::from_corners(Vec2::new(106., 9.), Vec2::new(115., 18.)),
            Rect::from_corners(Vec2::new(115., 9.), Vec2::new(124., 18.)),
            // Player Breath
            Rect::from_corners(Vec2::new(15., 18.), Vec2::new(25., 27.)),
            Rect::from_corners(Vec2::new(25., 18.), Vec2::new(34., 27.)),
            // Player Armor Water
            Rect::from_corners(Vec2::new(34., 18.), Vec2::new(43., 27.)),
            Rect::from_corners(Vec2::new(43., 18.), Vec2::new(52., 27.)),
            // Player Food Outline
            Rect::from_corners(Vec2::new(15., 27.), Vec2::new(25., 36.)),
            Rect::from_corners(Vec2::new(25., 27.), Vec2::new(34., 36.)),
            Rect::from_corners(Vec2::new(34., 27.), Vec2::new(43., 36.)),
            Rect::from_corners(Vec2::new(43., 27.), Vec2::new(52., 36.)),
            // Player Food
            Rect::from_corners(Vec2::new(52., 27.), Vec2::new(61., 36.)),
            Rect::from_corners(Vec2::new(61., 27.), Vec2::new(70., 36.)),
            Rect::from_corners(Vec2::new(70., 27.), Vec2::new(79., 36.)),
            Rect::from_corners(Vec2::new(79., 27.), Vec2::new(88., 36.)),
            Rect::from_corners(Vec2::new(88., 27.), Vec2::new(97., 36.)),
            Rect::from_corners(Vec2::new(97., 27.), Vec2::new(106., 36.)),
            Rect::from_corners(Vec2::new(106., 27.), Vec2::new(115., 36.)),
            Rect::from_corners(Vec2::new(115., 27.), Vec2::new(124., 36.)),
            // Player Food Outline Again
            Rect::from_corners(Vec2::new(124., 27.), Vec2::new(133., 36.)),
            Rect::from_corners(Vec2::new(133., 27.), Vec2::new(142., 36.)),
            // Player Food Flipped
            Rect::from_corners(Vec2::new(15., 36.), Vec2::new(24., 45.)),
            // Player Hardcore Heart Outline
            Rect::from_corners(Vec2::new(15., 45.), Vec2::new(25., 54.)),
            Rect::from_corners(Vec2::new(25., 45.), Vec2::new(34., 54.)),
            Rect::from_corners(Vec2::new(34., 45.), Vec2::new(43., 54.)),
            Rect::from_corners(Vec2::new(43., 45.), Vec2::new(52., 54.)),
            // Player Hardcore Hearts
            Rect::from_corners(Vec2::new(52., 45.), Vec2::new(61., 54.)),
            Rect::from_corners(Vec2::new(61., 45.), Vec2::new(70., 54.)),
            Rect::from_corners(Vec2::new(70., 45.), Vec2::new(79., 54.)),
            Rect::from_corners(Vec2::new(79., 45.), Vec2::new(88., 54.)),
            Rect::from_corners(Vec2::new(88., 45.), Vec2::new(97., 54.)),
            Rect::from_corners(Vec2::new(97., 45.), Vec2::new(106., 54.)),
            Rect::from_corners(Vec2::new(106., 45.), Vec2::new(115., 54.)),
            Rect::from_corners(Vec2::new(115., 45.), Vec2::new(124., 54.)),
            Rect::from_corners(Vec2::new(124., 45.), Vec2::new(133., 54.)),
            Rect::from_corners(Vec2::new(133., 45.), Vec2::new(142., 54.)),
            Rect::from_corners(Vec2::new(142., 45.), Vec2::new(151., 54.)),
            Rect::from_corners(Vec2::new(151., 45.), Vec2::new(160., 54.)),
            Rect::from_corners(Vec2::new(160., 45.), Vec2::new(169., 54.)),
            Rect::from_corners(Vec2::new(169., 45.), Vec2::new(178., 54.)),
            Rect::from_corners(Vec2::new(178., 45.), Vec2::new(187., 54.)),
            Rect::from_corners(Vec2::new(187., 45.), Vec2::new(196., 54.)),
            // Connection Strength
            Rect::from_corners(Vec2::new(0., 16.), Vec2::new(10., 24.)),
            Rect::from_corners(Vec2::new(0., 24.), Vec2::new(10., 32.)),
            Rect::from_corners(Vec2::new(0., 32.), Vec2::new(10., 40.)),
            Rect::from_corners(Vec2::new(0., 40.), Vec2::new(10., 48.)),
            Rect::from_corners(Vec2::new(0., 48.), Vec2::new(10., 56.)),
            Rect::from_corners(Vec2::new(0., 56.), Vec2::new(10., 64.)),
            // XP Bar
            Rect::from_corners(Vec2::new(0., 64.), Vec2::new(182., 69.)),
            Rect::from_corners(Vec2::new(0., 69.), Vec2::new(182., 74.)),
            // Boss Bar
            Rect::from_corners(Vec2::new(0., 74.), Vec2::new(182., 79.)),
            Rect::from_corners(Vec2::new(0., 79.), Vec2::new(182., 84.)),
            // Horse Jump Bar
            Rect::from_corners(Vec2::new(0., 84.), Vec2::new(182., 89.)),
            Rect::from_corners(Vec2::new(0., 89.), Vec2::new(182., 94.)),
            // Weapon Swing
            Rect::from_corners(Vec2::new(0., 94.), Vec2::new(18., 112.)),
            Rect::from_corners(Vec2::new(18., 94.), Vec2::new(36., 112.)),
            Rect::from_corners(Vec2::new(36., 94.), Vec2::new(52., 102.)),
            Rect::from_corners(Vec2::new(52., 94.), Vec2::new(68., 102.)),
            Rect::from_corners(Vec2::new(68., 94.), Vec2::new(84., 102.)),
            // Server Ping Strength
            Rect::from_corners(Vec2::new(0., 176.), Vec2::new(10., 184.)),
            Rect::from_corners(Vec2::new(0., 184.), Vec2::new(10., 192.)),
            Rect::from_corners(Vec2::new(0., 192.), Vec2::new(10., 200.)),
            Rect::from_corners(Vec2::new(0., 200.), Vec2::new(10., 208.)),
            Rect::from_corners(Vec2::new(0., 208.), Vec2::new(10., 216.)),
            Rect::from_corners(Vec2::new(0., 216.), Vec2::new(10., 224.)),
            // Server Ping Anim
            Rect::from_corners(Vec2::new(10., 176.), Vec2::new(20., 184.)),
            Rect::from_corners(Vec2::new(10., 184.), Vec2::new(20., 192.)),
            Rect::from_corners(Vec2::new(10., 192.), Vec2::new(20., 200.)),
            Rect::from_corners(Vec2::new(10., 200.), Vec2::new(20., 208.)),
            Rect::from_corners(Vec2::new(10., 208.), Vec2::new(20., 216.)),
        ]
    }
}
