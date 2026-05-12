use bevy::{
    prelude::*,
    asset::RenderAssetUsages,
    image::{ImageSampler, ImageSamplerDescriptor},
    render::{
        render_resource::*,
        storage::ShaderStorageBuffer,
        extract_resource::{ExtractResource, ExtractResourcePlugin},
    },
};
use crate::matter::{Matter, MatterId};

pub const GRID_WIDTH: u32 = 2048;
pub const GRID_HEIGHT: u32 = 1024;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<SandGridGPU>::default())
            .insert_resource(SimulationSettings::default()) // <-- Added
            .insert_resource(GridConfig {
                width: GRID_WIDTH, height: GRID_HEIGHT,
                mouse_x: 0, mouse_y: 0, brush_radius: 0, brush_matter: 0, brush_color: 0, is_drawing: 0,
                time: 0,
                run_physics: 1, // <-- Added
            })
            .add_systems(Startup, setup_gpu_buffers)
            .add_systems(Update, 
                (
                    step_simulation, 
                    swap_buffers, 
                    sync_config_to_gpu
                ).chain()
            );
    }
}

// --- NEW RESOURCE ---
#[derive(Resource)]
pub struct SimulationSettings {
    pub updates_per_second: f32,
    pub accumulator: f32,
    pub is_paused: bool,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            updates_per_second: 60.0,
            accumulator: 0.0,
            is_paused: false,
        }
    }
}

#[derive(Resource, ShaderType, Clone, Copy)]
pub struct GridConfig {
    pub width: u32,
    pub height: u32,
    pub mouse_x: u32,
    pub mouse_y: u32,
    pub brush_radius: u32,
    pub brush_matter: u32,
    pub brush_color: u32,
    pub is_drawing: u32,
    pub time: u32,
    pub run_physics: u32,
}

#[derive(Resource, Default)]
pub struct SimulationStats {
    pub particle_count: usize,
}

#[derive(Resource, ExtractResource, Clone)]
pub struct SandGridGPU {
    pub config_buffer: Handle<ShaderStorageBuffer>,
    pub read_buffer: Handle<ShaderStorageBuffer>,
    pub write_buffer: Handle<ShaderStorageBuffer>,
    pub texture: Handle<Image>,
}

fn step_simulation(
    time: Res<Time>,
    mut settings: ResMut<SimulationSettings>,
    mut config: ResMut<GridConfig>,
) {
    if settings.is_paused {
        config.run_physics = 0;
        return;
    }

    settings.accumulator += time.delta_secs();
    let step_size = 1.0 / settings.updates_per_second;

    // If enough time has passed, tell the GPU to advance the physics!
    if settings.accumulator >= step_size {
        config.run_physics = 1;
        settings.accumulator -= step_size;
        
        // Prevent a spiral of death if the window is dragged or frozen
        if settings.accumulator > step_size * 2.0 {
            settings.accumulator = 0.0;
        }
    } else {
        config.run_physics = 0;
    }
}

fn setup_gpu_buffers(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    config: Res<GridConfig>,
) {
    let total_pixels = (GRID_WIDTH * GRID_HEIGHT) as usize;
    let config_buffer = buffers.add(ShaderStorageBuffer::from(*config));
    commands.insert_resource(SimulationStats { particle_count: 0 });

    let initial_data = vec![Matter::new(MatterId::Empty, 0x00000000); total_pixels];
    let read_buffer = buffers.add(ShaderStorageBuffer::from(initial_data));
    let write_buffer = buffers.add(ShaderStorageBuffer::from(
        vec![Matter::new(MatterId::Empty, 0x00000000); total_pixels]
    ));

    let mut image = Image::new_fill(
        Extent3d { width: GRID_WIDTH, height: GRID_HEIGHT, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm, 
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    
    image.texture_descriptor.usage |= TextureUsages::STORAGE_BINDING;
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());
    let texture = images.add(image);

    commands.insert_resource(SandGridGPU { 
        config_buffer, read_buffer, write_buffer, texture: texture.clone() 
    });

    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn swap_buffers(mut gpu_data: ResMut<SandGridGPU>) {
    let data = &mut *gpu_data;
    std::mem::swap(&mut data.read_buffer, &mut data.write_buffer);
}

fn sync_config_to_gpu(
    config: Res<GridConfig>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    gpu_data: Res<SandGridGPU>,
) {
    if config.is_changed() {
        buffers.insert(gpu_data.config_buffer.id(), ShaderStorageBuffer::from(*config)).expect("Failed to insert into config");
    }
}