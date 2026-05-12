use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    time::common_conditions::on_timer,
};
use std::time::Duration;
use crate::render::{SimulationStats, SimulationSettings};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_diagnostics_ui)
           .add_systems(Update, update_diagnostics_ui.run_if(on_timer(Duration::from_millis(150))));
    }
}

// Marker components so we can find exactly which text to update
#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct ParticleText;

#[derive(Component)]
struct SpeedText;

fn setup_diagnostics_ui(mut commands: Commands) {
    // Root window node: Absolute positioned top-left with a dark glass background
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(15.0),
            left: Val::Px(15.0),
            padding: UiRect::all(Val::Px(15.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            border: UiRect::all(Val::Px(1.0)),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        })
        .insert(BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.85)))
        .insert(BorderColor::all(Color::srgba(0.3, 0.3, 0.3, 0.9)))
        .with_children(|parent| {
            // Title Header
            parent.spawn((
                Text::new("Engine Diagnostics"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgba(0.7, 0.7, 0.7, 1.0)),
            ));

            // Visual Divider Line
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(1.0),
                    margin: UiRect::axes(Val::Px(0.0), Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
            ));

            // FPS Counter
            parent.spawn((
                Text::new("FPS: --"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgba(0.2, 0.9, 0.2, 1.0)), // Green
                FpsText,
            ));

            // Particle Counter
            parent.spawn((
                Text::new("Particles: --"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgba(0.9, 0.7, 0.2, 1.0)), // Orange
                ParticleText,
            ));

            // Speed display
            parent.spawn((
                Text::new("Speed: --"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgba(0.2, 0.7, 0.9, 1.0)), // Cyan
                SpeedText,
            ));
        });
}

fn update_diagnostics_ui(
    diagnostics: Res<DiagnosticsStore>,
    stats: Option<Res<SimulationStats>>,
    settings: Option<Res<SimulationSettings>>, // <-- Added Settings
    mut fps_query: Query<&mut Text, (With<FpsText>, Without<ParticleText>, Without<SpeedText>)>,
    mut particle_query: Query<&mut Text, (With<ParticleText>, Without<FpsText>, Without<SpeedText>)>,
    mut speed_query: Query<(&mut Text, &mut TextColor), (With<SpeedText>, Without<FpsText>, Without<ParticleText>)>,
) {
    if let Ok(mut text) = fps_query.single_mut() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.0 = format!("FPS: {:.0}", value);
            }
        }
    }

    if let Ok(mut text) = particle_query.single_mut() {
        let count = stats.map(|s| s.particle_count).unwrap_or(0);
        text.0 = format!("Particles: {}", count);
    }

    if let Ok((mut text, mut color)) = speed_query.single_mut() {
        if let Some(settings) = settings {
            if settings.is_paused {
                text.0 = "Speed: PAUSED".into();
                *color = TextColor(Color::srgba(0.9, 0.2, 0.2, 1.0)); // Turn Red
            } else {
                text.0 = format!("Speed: {:.0} UPS", settings.updates_per_second);
                *color = TextColor(Color::srgba(0.2, 0.7, 0.9, 1.0)); // Turn Cyan
            }
        }
    }
}