mod utils;
use utils::get_offset;

use egui::{Ui, Vec2, Pos2, Rect};
use self::utils::{is_in_bounds_array, rect_point_collision};

use super::world::World;

/// Handles rendering of the world state onto the UI.
pub struct WorldDisplay {
    pub click_radius: i32,
}

impl WorldDisplay {
    /// Creates a new WorldDisplay instance.
    pub fn new() -> Self {
        Self {
            click_radius: 1,
        }
    }

    /// Renders the world onto the given UI context.
    ///
    /// # Arguments
    ///
    /// * `ui` - The `egui::Ui` reference to draw the interface.
    pub fn render(&self, ui: &mut Ui, world: &World) {
        for (_index_x, row) in world.data.iter().enumerate() {
            for (_index_y, cell) in row.iter().enumerate() {
                if cell.is_none() {continue;}
                let rect = cell.unwrap().rect; // Assuming cell has a `rect` field
                let color = cell.unwrap().color; // Assuming cell has a `color` field
    
                let stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
                ui.painter().rect(rect, 0.0, color, stroke);
            }
        }
    }

    // Function for handling clicks in the UI
    pub fn clicked(&self, world: &mut World, size_of_the_window: Vec2, pos: Option<Pos2>, central_panel_position: Pos2) {
        if let Some(mouse_position) = pos {
            let offset = get_offset(size_of_the_window, world.width, world.height) + central_panel_position.to_vec2();
            
            for index_x in 0..world.width {
                for index_y in 0..world.height {
                    let centrum_x = offset.x + index_x as f32 * world.cell_size;
                    let centrum_y = offset.y + index_y as f32 * world.cell_size;
                    
                    // Check if the mouse position is within the cell bounds
                    if rect_point_collision(mouse_position, Pos2{ x: (centrum_x), y: (centrum_y) }, world.cell_size) {
                        for radius_x in -self.click_radius..self.click_radius {
                            for radius_y in -self.click_radius..self.click_radius {
                                let x = centrum_x + radius_x as f32 * world.cell_size;
                                let y = centrum_y + radius_y as f32 * world.cell_size;
                                if is_in_bounds_array(index_x as i32 + radius_x, index_y as i32 + radius_y, world.width, world.height){
                                    world.place_rect(x, y, (index_x as i32 + radius_x) as usize, (index_y as i32 + radius_y) as usize);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
