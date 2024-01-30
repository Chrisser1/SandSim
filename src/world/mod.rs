use egui::{Pos2, Rect, Vec2};

/// Represents a 2D grid world.
///
/// The world is represented as a flat vector of booleans, where each boolean
/// indicates the state of a cell in the world. The size of the world is
/// determined by `width` and `height`, which represent the width and height
/// of the grid, respectively.
pub struct World {
    data: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Default for World {
    fn default() -> Self {
        Self {
            data: vec![vec![false; 10];10],
            width: 10,
            height: 10,
        }
    }
}

impl World {
    /// Creates a new World with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the world.
    /// * `height` - The height of the world.
    ///
    /// # Examples
    ///
    /// ```
    /// let world = World::new(10, 10);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![vec![false; height]; width],
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
    /// * `width` - The new width of the world.
    /// * `height` - The new height of the world.
    ///
    /// # Examples
    ///
    /// ```
    /// let world = World::new(10, 10);
    /// let resized_world = World::resize(20, 20);
    /// ```
    pub fn resize(width: usize, height: usize) -> Self {
        World::new(width, height)
    }
}

pub trait Display {
    /// Displays the world in an `egui` interface.
    ///
    /// # Arguments
    ///
    /// * `ui` - The `egui::Ui` reference to draw the interface.
    fn show(&self, ui: &mut egui::Ui);
}

impl Display for World {
    fn show(&self, ui: &mut egui::Ui) {
        
        let available_size = ui.available_size();


        let cell_width_size = available_size.x as f32 / (self.width * 2) as f32;
        let cell_height_size = available_size.y as f32 / (self.height * 2) as f32;
        
        // Set the cell size based on the current space available 
        let cell_size = if cell_width_size > cell_height_size {
            cell_width_size
        } else {
            cell_height_size
        };
        
        let total_grid_size = (cell_size * self.width as f32, cell_size * self.height as f32);

        // Calculate the offset to center the grid
        let offset_x = (available_size.x - total_grid_size.0 as f32) / 2.0;
        let offset_y = (available_size.y - total_grid_size.1 as f32) / 2.0;
        println!("offset_x: {} ", offset_x);
        print!(" offset_y: {} ", offset_y);
        for (index_x, row) in self.data.iter().enumerate() {
            for (index_y, _) in row.iter().enumerate() {
                let x = offset_x + index_x as f32 * cell_size;
                let y = offset_y + index_y as f32 * cell_size;

                let rect = Rect::from_min_size(
                    Pos2 { x, y },
                    egui::Vec2::splat(cell_size as f32),
                );

                if ui.interact(rect, ui.id().with((index_x, index_y)), egui::Sense::click()).clicked() {
                    *cell = true;  // Set the cell to true on click
                }

                // Define the frame style
                let stroke = egui::Stroke::new(2.0, egui::Color32::WHITE); // Width and color of the border
                ui.painter().rect(
                    rect,
                    0.0,
                    if self.data[index_x][index_y] { egui::Color32::YELLOW } else { egui::Color32::BLACK }, // Cell color
                    stroke,
                );
            }
        }
    }
}
