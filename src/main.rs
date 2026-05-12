mod matter;
mod ui;
mod render;
mod input;

use bevy::{
    prelude::*,
    diagnostic::FrameTimeDiagnosticsPlugin, 
};
use crate::render::{SimulationPlugin, SandComputePlugin};
use crate::ui::{CameraPlugin, GuiPlugin};
use crate::input::MousePlugin;

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
        .add_plugins((
            FrameTimeDiagnosticsPlugin::default(), 
            CameraPlugin, 
            SimulationPlugin, 
            SandComputePlugin, 
            GuiPlugin, 
            MousePlugin,
        ))
        .run();
}