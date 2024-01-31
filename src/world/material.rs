use egui::{Color32, Rect};



#[derive(Copy, Clone)]
pub struct Material {
    pub color: Color32,
    pub is_static: bool,
    pub rect: Rect,
}


impl Material {
    const  fn new(color: Color32, is_static: bool, rect: Rect) -> Self {
        Self {
            color: color,
            is_static: is_static,
            rect: rect
        }
    }

    pub const fn sand(rect: Rect) -> Option<Material>  {
        Some(Material::new(Color32::YELLOW, false, rect))
    }
    pub const fn stone(rect: Rect) -> Option<Material>  {
        Some(Material::new(Color32::GRAY, true, rect))
    }
    pub const fn air() -> Option<Material>  {
        None
    }
}

pub enum MaterialTypes {
    Air,
    Sand,
    Stone
}