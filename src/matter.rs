use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

/// The fundamental unit of our simulation. 
/// We keep it at exactly 8 bytes (two u32s) for optimal GPU alignment.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Serialize, Deserialize)]
pub struct Pixel {
    /// Stores the MatterId and potentially 8-bit metadata (like life-time or temperature)
    pub matter_info: u32,
    /// The current color of the pixel, stored as 0xRRGGBBAA
    pub color: u32,
}

/// Matter Id representing matter types
#[repr(u32)]
#[derive(Serialize, Deserialize, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum MatterId {
    #[default]
    Empty = 0,
    Sand = 1,
    Rock = 2,
    Water = 3,
}

/// Physical states for different matter types
#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum MatterState {
    Empty = 0,
    Powder = 1,
    Liquid = 2,
    Solid = 3,
    Gas = 4,
}

impl Pixel {
    pub fn new(id: MatterId, color: u32) -> Self {
        Self {
            matter_info: id as u32,
            color,
        }
    }
}