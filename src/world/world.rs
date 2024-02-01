mod utils;
use utils::get_cell_size;
#[path = "./material.rs"] pub(crate) mod material;
use material::{Material, MaterialTypes};

use egui::{Pos2, Vec2, Rect};

use self::utils::is_in_bounds_array;

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
        let width = 200;
        let height = 200;
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

    /// Places a rectangle in the world grid based on material and position.
    pub fn place_rect(&mut self, x: f32, y: f32, index_x: usize, index_y: usize) {
        let rect = Rect::from_min_size(
            Pos2 { x, y },
            egui::Vec2::splat(self.cell_size),
        );
        self.data[index_x][index_y] = match self.selected_material {
            MaterialTypes::Air => Material::air(),
            MaterialTypes::Sand => Material::sand(rect),
            MaterialTypes::Stone => Material::stone(rect)
        };
    }

    /// Moves a material from one position to another in the grid.
    pub fn move_material(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool{
        let mut something_changed = false;
        if is_in_bounds_array(from_x as i32, from_y as i32, self.width, self.height) && 
            is_in_bounds_array(to_x as i32, to_y as i32, self.width, self.height) {
            if let Some(mut material) = &self.data[from_x][from_y] {
                if self.data[to_x][to_y].is_none() {
                    // Update the position of the material
                    let new_rect_pos_x = material.rect.left_top().x as f32 + self.cell_size * (to_x as f32 - from_x as f32);
                    let new_rect_pos_y = material.rect.left_top().y as f32 + self.cell_size * (to_y as f32 - from_y as f32);
                    self.data[to_x][to_y] = Some(Material::new_with_position(
                        material, 
                        Pos2 { x: (new_rect_pos_x), y: (new_rect_pos_y) }, 
                        self.cell_size
                    ));
                    self.data[from_x][from_y] = None;
                    something_changed = true;
                }
            }
        }
        return something_changed;
    }    
}