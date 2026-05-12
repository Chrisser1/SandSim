mod camera;
mod matter;
mod simulation;
mod pipeline;

use bevy::{
    prelude::*,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, 
};
use crate::camera::CameraPlugin;
use crate::simulation::SimulationPlugin;
use crate::pipeline::SandComputePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SandSim - Native Bevy".into(),
                present_mode: bevy::window::PresentMode::AutoNoVsync, 
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default()) 
        .add_plugins(CameraPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(SandComputePlugin)
        .add_systems(Update, update_window_title) 
        .run();
}

// The system that updates the window title with the current FPS
fn update_window_title(
    diagnostics: Res<DiagnosticsStore>,
    mut windows: Query<&mut Window>,
) {
    // Look up the FPS diagnostic
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        // Get the smoothed average FPS
        if let Some(value) = fps.smoothed() {
            // Update the window title
            if let Ok(mut window) = windows.single_mut() {
                window.title = format!("SandSim | {:.1} FPS", value);
            }
        }
    }
}