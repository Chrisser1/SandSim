mod matter;
mod camera;

use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::EguiPlugin;

use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SandSim - Native Bevy".into(),
                resolution: WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        
        // Add Egui for our UI
        .add_plugins(EguiPlugin::default())
        .add_plugins(CameraPlugin)
        
        // Our Initial Logic
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    
    info!("SandSim Window Initialized!");
}

fn close_on_esc(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}