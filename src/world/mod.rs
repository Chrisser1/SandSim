use std::vec;

use egui::{ Color32, Pos2, Rect, Vec2};
use rand::Rng;
/// Represents a 2D grid world.
///
/// The world is represented as a flat vector of booleans, where each boolean
/// indicates the state of a cell in the world. The size of the world is
/// determined by `width` and `height`, which represent the width and height
/// of the grid, respectively.
pub struct World {
    pub data: Vec<Vec<Option<Material>>>,
    width: usize,
    height: usize,
    cell_size: f32,
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
    pub fn new(width: usize, height: usize, available_window_size: Vec2, central_panel_position: Pos2) -> Self {
        let rects = World::create_rects(available_window_size, width, height, central_panel_position);
        let mut materials = Vec::with_capacity(width);
        let cell_size = World::get_cell_size(available_window_size, width, height);
    
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
            cell_size: cell_size, 
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

    pub fn create_rects(available_window_size: Vec2, width: usize, height: usize, central_panel_position: Pos2) -> Vec<Vec<Rect>> {
        let cell_size = World::get_cell_size(available_window_size, width, height);
        
        // Calculate the offset to center the grid
        let offset = World::get_offset(available_window_size, width, height) + central_panel_position.to_vec2();
        
        let mut output = Vec::with_capacity(width);
    
        for index_x in 0..width {
            let mut row = Vec::with_capacity(height);
            for index_y in 0..height {
                let x = offset.x + index_x as f32 * cell_size;
                let y = offset.y + index_y as f32 * cell_size;
                row.push(Rect::from_min_size(
                    Pos2 { x, y },
                    egui::Vec2::splat(cell_size),
                ));
            }
            output.push(row);
        }
        output
    }

    pub fn clicked(&mut self, size_of_the_window: Vec2, pos: Option<Pos2>, central_panel_position: Pos2) {
        if let Some(mouse_position) = pos {
            let window_size = size_of_the_window;
            let offset = World::get_offset(window_size, self.width, self.height) + central_panel_position.to_vec2();
            
            // println!("window_size.x: {} window_size.y: {} offset.x: {} offset.y: {} cell_size: {}", window_size.x, window_size.y, offset.x, offset.y, cell_size);

            for index_x in 0..self.width {
                for index_y in 0..self.height {
                    let x = offset.x + index_x as f32 * self.cell_size;
                    let y = offset.y + index_y as f32 * self.cell_size;
                    
                    // Check if the mouse position is within the cell bounds
                    if mouse_position.x >= x && mouse_position.x < x + self.cell_size &&
                       mouse_position.y >= y && mouse_position.y < y + self.cell_size {
                        // println!("Cell clicked at ({}, {})", index_x, index_y);
                        self.data[index_x][index_y] = Material::sand(Rect::from_min_size(
                            Pos2 { x, y },
                            egui::Vec2::splat(self.cell_size),
                        ));
                    }
                }
            }
        }
    }
    

    fn get_cell_size(available_window_size: Vec2, width: usize, height: usize) -> f32 {
        let cell_width_size = available_window_size.x as f32 / width as f32;
        let cell_height_size = available_window_size.y as f32 / height as f32;
        
        // Set the cell size based on the current space available 
        cell_width_size.min(cell_height_size)
    }




    fn get_offset(available_window_size: Vec2, width: usize, height: usize) -> Vec2 {
        let cellsize = World::get_cell_size(available_window_size, width, height);
        
        let total_grid_size = Vec2 {
            x: width as f32 * cellsize,
            y: height as f32 * cellsize,
        };

        // Calculate the offset to center the grid
        let offset_x = (available_window_size.x - total_grid_size.x) / 2.0;
        let offset_y = (available_window_size.y - total_grid_size.y) / 2.0;
        let hardcored_margin = 8.0;
        /// https://github.com/emilk/egui/discussions/2297
        Vec2{ x: (offset_x + hardcored_margin), y: (offset_y - hardcored_margin)}
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
        for (index_x, row) in self.data.iter().enumerate() {
            for (index_y, cell) in row.iter().enumerate() {
                if cell.is_none() {continue;}
                let rect = cell.unwrap().rect; // Assuming cell has a `rect` field
                let color = cell.unwrap().color; // Assuming cell has a `color` field

                let stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
                ui.painter().rect(rect, 0.0, color, stroke);
                
            }
        }
    }

    fn update(&mut self) -> bool {
        let mut something_changed = false;

        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(current) = self.data[x][y] {
                    if y < self.height - 1 && self.data[x][y + 1].is_none() {
                        // Move down
                        self.data[x][y + 1] = Material::sand(Rect::from_min_size(
                            Pos2 { x: current.rect.left_top().x, y: current.rect.left_top().y + self.cell_size },
                            egui::Vec2::splat(self.cell_size),
                        ));
                        self.data[x][y] = None;
                        something_changed = true;
                    } else {
                        let move_left = rand::thread_rng().gen_range(0..2) == 0;
                        if move_left && x > 0 && y < self.height - 1 && self.data[x - 1][y + 1].is_none() {
                            // Move left
                            self.data[x - 1][y + 1] = Material::sand(Rect::from_min_size(
                                Pos2 { x: current.rect.left_top().x - self.cell_size, y: current.rect.left_top().y + self.cell_size },
                                egui::Vec2::splat(self.cell_size),
                            ));
                            self.data[x][y] = None;
                            something_changed = true;
                        } else if !move_left && x < self.width - 1 && y < self.height - 1 && self.data[x + 1][y + 1].is_none() {
                            // Move right
                            self.data[x + 1][y + 1] = Material::sand(Rect::from_min_size(
                                Pos2 { x: current.rect.left_top().x + self.cell_size, y: current.rect.left_top().y + self.cell_size },
                                egui::Vec2::splat(self.cell_size),
                            ));
                            self.data[x][y] = None;
                            something_changed = true;
                        }
                    }
                }
            }
        }
        something_changed
    }
}

#[derive(Copy, Clone)]
pub struct Material {
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

#[cfg(test)]
mod tests {
    use super::*;
    use egui::Vec2;

    #[test]
    fn test_cell_size_and_offset() {
        // Define a test window size
        let test_window_size = Vec2::new(967.0, 800.0);
        let grid_width = 10;
        let grid_height = 10;

        // Expected values
        let expected_cell_size = 80.0; // Assuming cells are square and fit perfectly in the window
        let expected_offset = Vec2::new((967.0-800.0)/2.0, 0.0); // Assuming centered horizontally, no vertical offset

        // Actual values from your functions
        let actual_cell_size = World::get_cell_size(test_window_size, grid_width, grid_height);
        let actual_offset = World::get_offset(test_window_size, grid_width, grid_height);

        println!("actual_cell_size: {} - actual_offset.x: {} - actual_offset.y: {}", actual_cell_size, actual_offset.x, actual_offset.y);

        // Assertions
        assert_eq!(actual_cell_size, expected_cell_size, "Cell size calculation is incorrect.");
        assert_eq!(actual_offset, expected_offset, "Offset calculation is incorrect.");
    }
}