mod utils;
use utils::get_cell_size;
#[path = "./material.rs"] pub(crate) mod material;
use material::{Material, MaterialTypes};

use egui::{Pos2, Vec2};

/// Represents a 2D grid world.
///
/// The world is represented as a flat vector of booleans, where each boolean
/// indicates the state of a cell in the world. The size of the world is
/// determined by `width` and `height`, which represent the width and height
/// of the grid, respectively.
pub struct World {
    pub data: Vec<Vec<Option<Material>>>,
    pub width: usize,
    pub height: usize,
    pub cell_size: f32,
    pub selected_material: MaterialTypes,
}

impl Default for World {
    fn default() -> Self {
        let width = 100;
        let height = 100;
        let available_window_size = Vec2 { x: (872.0), y: (960.0) };
        let central_panel_position = Pos2 { x: (112.0), y: (36.0) };
        World::new(width, height, available_window_size, central_panel_position)
    }
}

impl World {
    /// Creates a new World with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the world.
    /// * `height` - The height of the world.
    /// * `available_window_size` - The current window size available
    ///
    /// # Examples
    ///
    /// ```
    /// let world = World::new(10, 10, ui.available_size());
    /// ```
    pub fn new(width: usize, height: usize, available_window_size: Vec2, _central_panel_position: Pos2) -> Self {
        let mut materials = Vec::with_capacity(width);
        let cell_size = get_cell_size(available_window_size, width, height);

        for _index_x in 0..width {
            let mut material_row = Vec::with_capacity(height);
            for _index_y in 0..height {
                // let cell = rects[index_x][index_y];
                material_row.push(Material::air());
            }
            materials.push(material_row);
        }
        Self {
            data: materials,
            width: width,
            height: height,
            cell_size: cell_size,
            selected_material: MaterialTypes::Sand
        }
    }

    /// Resizes the world to new dimensions.
    ///
    /// Note that this method creates a new world and does not preserve
    /// the state of the existing cells.
    ///
    /// # Arguments
    ///
    /// * `available_window_size` - The size of the current window.
    pub fn resize(&self, available_window_size: Vec2, central_panel_position: Pos2) -> Self {
        World::new(self.data.len(), self.data[0].len(), available_window_size, central_panel_position)
    }
}