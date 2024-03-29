use strum_macros::EnumIter;

use crate::{
    utils::{grey_scale_u32, u32_rgba_to_u8_rgba, u8_rgba_to_u32_rgba},
    EMPTY_COLOR, GREY_SCALE,
};

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