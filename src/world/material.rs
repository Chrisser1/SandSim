use egui::{Color32, Pos2, Rect};
use super::utils::random_color;

#[derive(Copy, Clone)]
pub struct Material {
    pub color: Color32,
    pub is_static: bool,
    pub rect: Rect,
    pub material_type: MaterialTypes,
}

impl Material {
    const  fn new(color: Color32, is_static: bool, rect: Rect, material_type: MaterialTypes) -> Self {
        Self {
            color: color,
            is_static: is_static,
            rect: rect,
            material_type: material_type
        }
    }

    /// Creates a new Material instance with the specified position.
    pub fn new_with_position(material: Material, position: Pos2, cell_size: f32) -> Self {
        match material.material_type {
            MaterialTypes::Air => Material::air().unwrap(),
            MaterialTypes::Sand => Material::sand(Rect::from_min_size(position, egui::Vec2::splat(cell_size))).unwrap(),
            MaterialTypes::RedSand => Material::red_sand(Rect::from_min_size(position, egui::Vec2::splat(cell_size))).unwrap(),
            MaterialTypes::RainbowSand => Material::rainbow_sand(Rect::from_min_size(position, egui::Vec2::splat(cell_size))).unwrap(),
            MaterialTypes::Stone => Material::stone(Rect::from_min_size(position, egui::Vec2::splat(cell_size))).unwrap(),
            MaterialTypes::BlackHole => Material::black_hole(Rect::from_min_size(position, egui::Vec2::splat(cell_size))).unwrap(),

            // ... handle other material types similarly
        }
    }

    pub const fn sand(rect: Rect) -> Option<Material>  {
        Some(Material::new(Color32::YELLOW, false, rect, MaterialTypes::Sand))
    }
    pub const fn red_sand(rect: Rect) -> Option<Material>  {
        Some(Material::new(Color32::DARK_RED, false, rect, MaterialTypes::RedSand))
    }
    // Funny Glitchy sand
    // pub const fn red_sand(rect: Rect) -> Option<Material>  {
    //     Some(Material::new(Color32::DARK_RED, false, rect, MaterialTypes::Stone))
    // }
    pub const fn black_hole(rect: Rect) -> Option<Material> {
        Some(Material::new(Color32::BLACK, true, rect, MaterialTypes::BlackHole))
    }
    pub const fn stone(rect: Rect) -> Option<Material> {
        Some(Material::new(Color32::GRAY, true, rect, MaterialTypes::Stone))
    }
    pub const fn air() -> Option<Material>  {
        None
    }
    pub const fn rainbow_sand(rect: Rect) -> Option<Material> {
        Some(Material::new(random_color(), false, rect, MaterialTypes::RainbowSand))
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MaterialTypes {
    Air,
    Sand,
    RedSand,
    RainbowSand,
    Stone,
    BlackHole
}
