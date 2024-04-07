use strum_macros::EnumIter;

use crate::{
    matter::{Direction, MatterCharacteristic, MatterState},
    utils::{grey_scale_u32, u32_rgba_to_u8_rgba, u8_rgba_to_u32_rgba},
    EMPTY_COLOR, GREY_SCALE,
};

pub const MAX_TRANSITIONS: u32 = 5;

pub struct MatterReaction {
    pub reacts: MatterCharacteristic,
    pub direction: Direction,
    pub probability: f32,
    pub becomes: u32,
}

impl MatterReaction {
    pub fn zero() -> Self {
        MatterReaction {
            reacts: MatterCharacteristic::empty(),
            direction: Direction::NONE,
            probability: 0.0,
            becomes: 0,
        }
    }

    pub fn dies(p: f32, empty_matter: u32) -> Self {
        MatterReaction {
            reacts: MatterCharacteristic::empty(),
            direction: Direction::ALL,
            probability: p,
            becomes: empty_matter,
        }
    }

    pub fn becomes_on_touch(
        p: f32,
        touch_characteristic: MatterCharacteristic,
        becomes_matter: u32,
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
        becomes_matter: u32,
    ) -> Self {
        MatterReaction {
            reacts: touch_characteristic,
            direction: (Direction::DOWN
                | Direction::DOWN_LEFT
                | Direction::DOWN_RIGHT
                | Direction::RIGHT
                | Direction::LEFT),
            probability: p,
            becomes: becomes_matter,
        }
    }
}

pub struct MatterDefinition {
    pub id: u32,
    pub name: String,
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
            id: 0,
            name: "Empty".to_string(),
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
}

/// Matter Id representing matter that we simulate
#[repr(u8)]
#[derive(EnumIter, Debug, Copy, Clone, Eq, PartialEq)]
pub enum MatterId {
    Empty = 0,
    Sand = 1,
    Stone = 2,
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

impl MatterId {
    fn color_rgba_u8(&self) -> [u8; 4] {
        let color = match *self {
            MatterId::Empty => EMPTY_COLOR,
            MatterId::Sand => 0xc2b280ff,
            MatterId::Stone => 0x787a79ff,
            MatterId::Water => 0x0f5e9cff,
        };
        if GREY_SCALE {
            u32_rgba_to_u8_rgba(grey_scale_u32(color))
        } else {
            u32_rgba_to_u8_rgba(color)
        }
    }
}

/// Matter data where first 3 bytes are saved for color and last 4th byte is saved for matter id
#[derive(Default, Copy, Clone)]
pub struct MatterWithColor {
    pub value: u32,
}

impl MatterWithColor {
    /// Creates a new matter with color from matter id giving it a slightly randomized color
    pub fn new(matter_id: MatterId) -> MatterWithColor {
        let color = matter_id.color_rgba_u8();
        MatterWithColor {
            value: u8_rgba_to_u32_rgba(color[0], color[1], color[2], matter_id as u8),
        }
    }

    pub fn matter_id(&self) -> MatterId {
        ((self.value & 255) as u8).into()
    }
}

impl From<u32> for MatterWithColor {
    fn from(item: u32) -> Self {
        Self {
            value: item,
        }
    }
}