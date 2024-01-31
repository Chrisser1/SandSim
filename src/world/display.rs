mod utils;
use utils::get_offset;

use egui::{Ui, Vec2, Pos2, Rect};
use super::world::{material::Material, material::MaterialTypes, World};

/// Handles rendering of the world state onto the UI.
pub struct WorldDisplay {
    // Fields related to display properties can be added here
}

impl WorldDisplay {
    /// Creates a new WorldDisplay instance.
    pub fn new() -> Self {
        Self {
            // Initialization of display properties
        }
    }

    /// Renders the world onto the given UI context.
    ///
    /// # Arguments
    ///
    /// * `ui` - The `egui::Ui` reference to draw the interface.
    pub fn render(ui: &mut Ui, world: &World) {
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

    // Additional display-related functionalities can be added here
}

// Function for handling clicks in the UI
pub fn clicked(world: &mut World, size_of_the_window: Vec2, pos: Option<Pos2>, central_panel_position: Pos2) {
    if let Some(mouse_position) = pos {
        let window_size = size_of_the_window;
        let offset = get_offset(window_size, world.width, world.height) + central_panel_position.to_vec2();

        // println!("window_size.x: {} window_size.y: {} offset.x: {} offset.y: {} cell_size: {}", window_size.x, window_size.y, offset.x, offset.y, cell_size);

        for index_x in 0..world.width {
            for index_y in 0..world.height {
                let x = offset.x + index_x as f32 * world.cell_size;
                let y = offset.y + index_y as f32 * world.cell_size;

                // Check if the mouse position is within the cell bounds
                if mouse_position.x >= x && mouse_position.x < x + world.cell_size &&
                    mouse_position.y >= y && mouse_position.y < y + world.cell_size {
                    // println!("Cell clicked at ({}, {})", index_x, index_y);
                    let rect = Rect::from_min_size(
                        Pos2 { x, y },
                        egui::Vec2::splat(world.cell_size),
                    );
                    world.data[index_x][index_y] = match world.selected_material {
                        MaterialTypes::Air => Material::air(),
                        MaterialTypes::Sand => Material::sand(rect),
                        MaterialTypes::Stone => Material::stone(rect)
                    };
                }
            }
        }
    }
}