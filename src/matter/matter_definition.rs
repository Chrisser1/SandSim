use strum_macros::EnumIter;
use serde::{Deserialize, Serialize};

use crate::{
    matter::{Direction, MatterCharacteristic, MatterState},
    utils::{grey_scale_u32, u32_rgba_to_u8_rgba, u8_rgba_to_u32_rgba},
    EMPTY_COLOR, GREY_SCALE,
};

use super::{MATTER_EMPTY, MATTER_ROCK, MATTER_SAND, MATTER_WATER};

pub const MAX_TRANSITIONS: u8 = 5;

/// Matter Id representing matter that we simulate
#[repr(u8)]
#[derive(Serialize, Deserialize, EnumIter, Debug, Copy, Clone, Eq, PartialEq)]
pub enum MatterId {
    Empty = 0,
    Sand = 1,
    Rock = 2,
    Water = 3,
}

impl Default for MatterId {
    fn default() -> Self {
        MatterId::Empty
    }
}

impl From<u8> for MatterId {
    fn from(item: u8) -> Self {
        unsafe { std::mem::transmute(item) }
    }
}

impl Into<u8> for MatterId {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct MatterReaction {
    pub reacts: MatterCharacteristic,
    pub direction: Direction,
    pub probability: f32,
    pub becomes: MatterId,
}

impl MatterReaction {
    pub const fn zero() -> Self {
        MatterReaction {
            reacts: MatterCharacteristic::empty(),
            direction: Direction::NONE,
            probability: 0.0,
            becomes: MatterId::Empty,
        }
    }

    pub fn dies(p: f32) -> Self {
        MatterReaction {
            reacts: MatterCharacteristic::empty(),
            direction: Direction::ALL,
            probability: p,
            becomes: MatterId::Empty,
        }
    }

    pub const fn becomes_on_touch(
        p: f32,
        touch_characteristic: MatterCharacteristic,
        becomes_matter: MatterId,
    ) -> Self {
        MatterReaction {
            reacts: touch_characteristic,
            direction: Direction::ALL,
            probability: p,
            becomes: becomes_matter,
        }
    }

    pub fn becomes_on_touch_below(
        p: f32,
        touch_characteristic: MatterCharacteristic,
        becomes_matter: MatterId,
    ) -> Self {
        MatterReaction {
            reacts: touch_characteristic,
            direction: (Direction::DOWN
                | Direction::DOWN_LEFT
                | Direction::DOWN_RIGHT
                | Direction::RIGHT
                | Direction::LEFT),
            probability: p,
            becomes: MatterId::from(becomes_matter),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatterDefinition {
    pub id: MatterId,
    pub color: u32,
    pub weight: f32,
    /// MatterState defines what state the matter is in
    /// - Liquid: behaves like a liquid
    /// - Powder: behaves like a powder
    pub state: MatterState,
    /// Characteristics defines what the matter "does to others"
    /// - Water: "Cools", "Rusts"
    /// - Acid: "Corrodes".
    pub characteristics: MatterCharacteristic,
    /// Reactions defines how the matter reacts with it's neighbors and their chararesticts'
    /// - Example: "Water becomes ice on probability x if touches one that freezes".
    /// - Example: "Acid might become empty on probability x if touches a material it corroded (corroding)".
    pub reactions: [MatterReaction; MAX_TRANSITIONS as usize],
}

impl MatterDefinition {
    pub fn zero() -> Self {
        MatterDefinition {
            id: MatterId::Empty,
            color: 0x0,
            weight: 0.0,
            state: MatterState::Empty,
            characteristics: MatterCharacteristic::empty(),
            reactions: [
                MatterReaction::zero(),
                MatterReaction::zero(),
                MatterReaction::zero(),
                MatterReaction::zero(),
                MatterReaction::zero(),
            ],
        }
    }

    fn color_rgba_u8(&self) -> [u8; 4] {
        if GREY_SCALE {
            u32_rgba_to_u8_rgba(grey_scale_u32(self.color))
        } else {
            u32_rgba_to_u8_rgba(self.color)
        }
    }

    /// Creates a new matter from matter id
    pub fn new(matter_id: MatterId) -> MatterDefinition {
        match matter_id {
            MatterId::Empty => MATTER_EMPTY,
            MatterId::Sand => MATTER_SAND,
            MatterId::Rock => MATTER_ROCK,
            MatterId::Water => MATTER_WATER,
        }
    }

    pub fn to_matter_with_color(&self) -> u32 {
        let color = self.color_rgba_u8();
        u8_rgba_to_u32_rgba(color[0], color[1], color[2], self.id.into())
    }

    pub fn get_id_from_u32(color_and_id: u32) -> MatterId {
        MatterId::from((color_and_id & 255) as u8)
    }
}


// /// Matter data where first 3 bytes are saved for color and last 4th byte is saved for matter id
// #[derive(Default, Copy, Clone)]
// pub struct MatterWithColor {
//     pub value: u32,
// }

// impl MatterWithColor {
//     /// Creates a new matter with color from matter id giving it a slightly randomized color
//     pub fn new(matter_id: MatterId) -> MatterWithColor {
//         let color = matter_id.color_rgba_u8();
//         MatterWithColor {
//             value: u8_rgba_to_u32_rgba(color[0], color[1], color[2], matter_id as u8),
//         }
//     }

//     pub fn matter_id(&self) -> MatterId {
//         ((self.value & 255) as u8).into()
//     }
// }

// impl From<u32> for MatterWithColor {
//     fn from(item: u32) -> Self {
//         Self {
//             value: item,
//         }
//     }
// }