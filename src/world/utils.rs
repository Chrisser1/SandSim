use egui::{Color32, Pos2, Vec2}; // Assuming Vec2 is from egui, import it

/// Function for calculating cell size based on window size and world dimensions
pub fn get_cell_size(available_window_size: Vec2, width: usize, height: usize) -> f32 {
    let cell_width_size = available_window_size.x as f32 / width as f32;
    let cell_height_size = available_window_size.y as f32 / height as f32;

    // Set the cell size based on the current space available
    cell_width_size.min(cell_height_size)
}

/// Function for calculating offset for UI elements
pub fn get_offset(available_window_size: Vec2, width: usize, height: usize) -> Vec2 {
    let cellsize = get_cell_size(available_window_size, width, height);

    let total_grid_size = Vec2 {
        x: width as f32 * cellsize,
        y: height as f32 * cellsize,
    };

    // Calculate the offset to center the grid
    let offset_x = (available_window_size.x - total_grid_size.x) / 2.0;
    let offset_y = (available_window_size.y - total_grid_size.y) / 2.0;
    let hardcored_margin = 8.0;
    // Center the grid with a margin
    Vec2{ x: (offset_x + hardcored_margin), y: (offset_y - hardcored_margin)}
}

pub fn rect_point_collision(mouse_position: Pos2, cell_position: Pos2, cell_size: f32) -> bool {
    mouse_position.x >= cell_position.x && mouse_position.x < cell_position.x + cell_size &&
    mouse_position.y >= cell_position.y && mouse_position.y < cell_position.y + cell_size
}

/// Checks if the given x and y coordinates are within the specified bounds.
pub fn is_in_bounds_array(x: i32, y: i32, width: usize, height: usize) -> bool {
    x < width as i32 && y < height as i32 && x >= 0 && y >= 0
}

pub const fn random_color() -> Color32 {
    //this is a random number that I picked
    Color32::from_rgb(255,255,255)
}
