use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OrthographicCamera::default())
           .add_systems(Startup, setup_camera)
           .add_systems(Update, move_camera);
    }
}

#[derive(Resource)]
pub struct OrthographicCamera {
    pub pos: Vec2,
    pub scale: f32,
}

impl Default for OrthographicCamera {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            scale: 1.0,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    // Spawn the actual Bevy 2D camera entity
    commands.spawn(Camera2d::default());
}

fn move_camera(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_state: ResMut<OrthographicCamera>,
    // Query the camera entity to update its actual transform
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut move_vec = Vec2::ZERO;
    let speed = 500.0 * camera_state.scale; // Adjust speed based on zoom

    if keyboard_input.pressed(KeyCode::KeyW) { move_vec.y += 1.0; }
    if keyboard_input.pressed(KeyCode::KeyS) { move_vec.y -= 1.0; }
    if keyboard_input.pressed(KeyCode::KeyA) { move_vec.x -= 1.0; }
    if keyboard_input.pressed(KeyCode::KeyD) { move_vec.x += 1.0; }

    if move_vec != Vec2::ZERO {
        camera_state.pos += move_vec.normalize() * speed * time.delta_secs();
    }

    // Update the actual Bevy Transform
    if let Ok(mut transform) = query.single_mut() {
        transform.translation = camera_state.pos.extend(0.0);
        transform.scale = Vec3::splat(camera_state.scale);
    }
}