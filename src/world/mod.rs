use std::vec;

use egui::{Color32, Pos2, Rect, Vec2};

/// Represents a 2D grid world.
///
/// The world is represented as a flat vector of booleans, where each boolean
/// indicates the state of a cell in the world. The size of the world is
/// determined by `width` and `height`, which represent the width and height
/// of the grid, respectively.
pub struct World {
    data: Vec<Vec<Option<Material>>>,
    width: usize,
    height: usize, 
}

impl Default for World {
    fn default() -> Self {
        let width = 100;
        let height = 100;
        let available_window_size = Vec2 { x: (1000.0), y: (1000.0) };
        
        World::new(width, height, available_window_size)
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
    pub fn new(width: usize, height: usize, available_window_size: Vec2) -> Self {
        let rects = World::create_rects(available_window_size, width, height);
        let mut materials = Vec::with_capacity(width);
    
        for index_x in 0..width {
            let mut material_row = Vec::with_capacity(height);
            for index_y in 0..height {
                let cell = rects[index_x][index_y];
                material_row.push(Material::air(cell));
            }
            materials.push(material_row);
        }
        Self {
            data: materials,
            width: width,
            height: height,
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
    pub fn resize(&self, available_window_size: Vec2) -> Self {
        World::new(self.data.len(), self.data[0].len(), available_window_size)
    }

    pub fn create_rects(available_window_size: Vec2, width: usize, height: usize) -> Vec<Vec<Rect>> {
        let cell_width_size = available_window_size.x as f32 / width as f32;
        let cell_height_size = available_window_size.y as f32 / height as f32;
        
        // Set the cell size based on the current space available 
        let cell_size = cell_width_size.min(cell_height_size);

        let total_grid_size = (cell_size * width as f32, cell_size * height as f32);
        
        // Calculate the offset to center the grid
        let offset_x = (available_window_size.x - total_grid_size.0) / 2.0;
        let offset_y = (available_window_size.y - total_grid_size.1) / 2.0;
        println!("Offset x: {} Offset y; {}", offset_x, offset_y);
        
        let mut output = Vec::with_capacity(width);
    
        for index_x in 0..width {
            let mut row = Vec::with_capacity(height);
            for index_y in 0..height {
                let x = offset_x + index_x as f32 * cell_size;
                let y = offset_y + index_y as f32 * cell_size;
    
                row.push(Rect::from_min_size(
                    Pos2 { x, y },
                    egui::Vec2::splat(cell_size),
                ));
            }
            output.push(row);
        }
        
        output
    }
}

pub trait Display {
    /// Displays the world in an `egui` interface.
    ///
    /// # Arguments
    ///
    /// * `ui` - The `egui::Ui` reference to draw the interface.
    fn show(&mut self, ui: &mut egui::Ui);

    /// Update the world using sand simulation. Returns true if anything was changed
    fn update(&mut self) -> bool;
}

impl Display for World {
    fn show(&mut self, ui: &mut egui::Ui) {
        let mut clicked_cell = None;

        for (index_x, row) in self.data.iter().enumerate() {
            for (index_y, cell) in row.iter().enumerate() {
                if cell.is_none() {continue;}
                let rect = cell.unwrap().rect; // Assuming cell has a `rect` field
                let color = cell.unwrap().color; // Assuming cell has a `color` field

                if ui.interact(rect, ui.id().with((index_x, index_y)), egui::Sense::click()).clicked() {
                    clicked_cell = Some((index_x, index_y));
                }

                let stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
                ui.painter().rect(rect, 0.0, color, stroke);
            }
        }

        // Apply changes after iteration
        if let Some((x, y)) = clicked_cell {
            if let Some(material) = self.data.get_mut(x).and_then(|row| row.get_mut(y)) {
                // Modify the material here, e.g., change its state
                *material = Material::sand(material.unwrap().rect); // Example modification
            }
        }
    }
    
    fn update(&mut self) -> bool{
        let mut something_changed = false;
        // Go through each cell and check if sand needs to be moved
        for x in 0..self.width {
            for y in (0..self.height).rev() { // Iterate in reverse to handle sand falling down
                if self.data[x][y].is_none() {
                    continue;
                }
                let tile = self.data[x][y].unwrap();
                if tile.gravity{
                    // Check if the cell below is within bounds and empty
                    if y < self.height - 1 && !self.data[x][y + 1].unwrap().gravity {
                        self.data[x][y + 1] = Material::sand(self.data[x][y + 1].unwrap().rect); // Move sand down
                        self.data[x][y] = Material::air(self.data[x][y].unwrap().rect);
                        something_changed = true;
                    } else if x > 0 && y < self.height - 1 && !self.data[x - 1][y + 1].unwrap().gravity {
                        // Check if the cell down-left is within bounds and empty
                        self.data[x - 1][y + 1] = Material::sand(self.data[x - 1][y + 1].unwrap().rect); // Move sand down-left
                        self.data[x][y] = Material::air(self.data[x][y].unwrap().rect);
                        something_changed = true;
                    } else if x < self.width - 1 && y < self.height - 1 && !self.data[x + 1][y + 1].unwrap().gravity {
                        // Check if the cell down-right is within bounds and empty
                        self.data[x + 1][y + 1] = Material::sand(self.data[x + 1][y + 1].unwrap().rect); // Move sand down-right
                        self.data[x][y] = Material::air(self.data[x][y].unwrap().rect);
                        something_changed = true;
                    }
                }
            }
        }
        return something_changed;
    }
}

#[derive(Copy, Clone)]
struct Material {
    color: Color32,
    gravity: bool,
    rect: Rect,
}


impl Material {
    fn new(color: Color32, gravity: bool, rect: Rect) -> Self {
        Self {
            color: color,
            gravity: gravity,
            rect: rect
        }
    }

    fn update_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn sand(rect: Rect) -> Option<Material>  {
        Some(Material::new(Color32::YELLOW, true, rect))
    }
    pub fn air(_rect: Rect) -> Option<Material>  {
        None
    }
}