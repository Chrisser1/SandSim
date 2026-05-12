use bevy::{prelude::*, input::mouse::MouseWheel, ecs::message::MessageReader};
use crate::render::{GridConfig, SimulationStats, SimulationSettings, GRID_WIDTH, GRID_HEIGHT};
use crate::matter::MatterId;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_controls);
    }
}

/// Track the currently selected brush globally
#[derive(Default)]
struct ActiveBrush {
    id: MatterId,
    color: u32,
    radius: u32,
}

fn handle_controls(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut scroll_evr: MessageReader<MouseWheel>,
    mut config: ResMut<GridConfig>,
    mut stats: ResMut<SimulationStats>,
    mut settings: ResMut<SimulationSettings>,
    time: Res<Time>,
    mut brush: Local<ActiveBrush>, // Local state stores our selected material between frames
) {
    // Initialize brush on first frame
    if brush.radius == 0 {
        brush.id = MatterId::Sand;
        brush.color = 0xE2C280FF; // Sand Color
        brush.radius = 8;
    }

    // --- KEYBOARD CONTROLS ---
    if keys.just_pressed(KeyCode::Space) { settings.is_paused = !settings.is_paused; }
    
    // Material Swapping!
    if keys.just_pressed(KeyCode::Digit1) {
        brush.id = MatterId::Sand; brush.color = 0xE2C280FF; brush.radius = 8;
    }
    if keys.just_pressed(KeyCode::Digit2) {
        brush.id = MatterId::Water; brush.color = 0x2266CCFF; brush.radius = 12; // Blue Water
    }
    if keys.just_pressed(KeyCode::Digit3) {
        brush.id = MatterId::Rock; brush.color = 0x555555FF; brush.radius = 15; // Dark Grey Rock
    }
    if keys.just_pressed(KeyCode::Digit4) {
        brush.id = MatterId::Empty; brush.color = 0x00000000; brush.radius = 20; // Eraser
    }

    for ev in scroll_evr.read() {
        settings.updates_per_second += ev.y * 15.0;
    }
    settings.updates_per_second = settings.updates_per_second.clamp(1.0, 1000.0);
    
    // --- MOUSE DRAWING ---
    let Ok(window) = windows.single() else { return; };
    let Ok((camera, camera_transform)) = camera_q.single() else { return; };

    config.is_drawing = 0;
    config.time = (time.elapsed_secs() * 1000.0) as u32; 

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            
            let grid_x = world_pos.x + (GRID_WIDTH as f32 / 2.0);
            let grid_y = -world_pos.y + (GRID_HEIGHT as f32 / 2.0);

            if grid_x >= 0.0 && grid_x < GRID_WIDTH as f32 && grid_y >= 0.0 && grid_y < GRID_HEIGHT as f32 {
                config.mouse_x = grid_x as u32;
                config.mouse_y = grid_y as u32;

                let area = (std::f32::consts::PI * (brush.radius as f32).powi(2)) as usize;

                // Draw the active material!
                if buttons.pressed(MouseButton::Left) {
                    config.is_drawing = 1;
                    config.brush_matter = brush.id as u32; 
                    config.brush_color = brush.color; 
                    config.brush_radius = brush.radius; 
                    if brush.id != MatterId::Empty { stats.particle_count += area / 15; }
                } 
                // Right click always erases
                else if buttons.pressed(MouseButton::Right) {
                    config.is_drawing = 1;
                    config.brush_matter = MatterId::Empty as u32; 
                    config.brush_color = 0x00000000; 
                    config.brush_radius = 20; 
                    stats.particle_count = stats.particle_count.saturating_sub(area / 10);
                }
            }
        }
    }
}