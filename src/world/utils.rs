use egui::Vec2; // Assuming Vec2 is from egui, import it

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