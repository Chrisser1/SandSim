use bevy::{
    prelude::*,
    asset::RenderAssetUsages,
    image::{ImageSampler, ImageSamplerDescriptor},
    render::{
        render_resource::*,
        storage::ShaderStorageBuffer, // Moved here
        extract_resource::{ExtractResource, ExtractResourcePlugin},
    },
};
use crate::matter::{Pixel, MatterId};

pub const GRID_WIDTH: u32 = 1024;
pub const GRID_HEIGHT: u32 = 1024;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<SandGridGPU>::default())
           .add_systems(Startup, setup_gpu_buffers)
           .add_systems(Update, swap_buffers);
    }
}

#[derive(ShaderType, Clone, Copy)]
pub struct GridConfig {
    pub width: u32,
    pub height: u32,
}

/// SandGridGPU stores the Handles to our GPU data here so the CPU can pass them to the Compute Pipeline
#[derive(Resource, ExtractResource, Clone)]
pub struct SandGridGPU {
    pub config_buffer: Handle<ShaderStorageBuffer>,
    pub read_buffer: Handle<ShaderStorageBuffer>,
    pub write_buffer: Handle<ShaderStorageBuffer>,
    pub texture: Handle<Image>,
}

fn setup_gpu_buffers(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    let total_pixels = (GRID_WIDTH * GRID_HEIGHT) as usize;

    let config = GridConfig { width: GRID_WIDTH, height: GRID_HEIGHT };
    let config_buffer = buffers.add(ShaderStorageBuffer::from(config));

    // Spawn a block of sand in the middle!
    let mut initial_data = vec![Pixel::new(MatterId::Empty, 0x00000000); total_pixels];
    let center_x = GRID_WIDTH / 2;
    let center_y = GRID_HEIGHT / 2;
    let block_size = 100;
    for y in (center_y - block_size / 2)..(center_y + block_size / 2) {
        for x in (center_x - block_size / 2)..(center_x + block_size / 2) {
            let idx = (y * GRID_WIDTH + x) as usize;
            initial_data[idx] = Pixel::new(MatterId::Sand, 0xE2C280FF);
        }
    }

    let read_buffer = buffers.add(ShaderStorageBuffer::from(initial_data.clone()));
    let write_buffer = buffers.add(ShaderStorageBuffer::from(
        vec![Pixel::new(MatterId::Empty, 0x00000000); total_pixels]
    ));

    // Create the Texture the GPU will draw to
    let mut image = Image::new_fill(
        Extent3d { width: GRID_WIDTH, height: GRID_HEIGHT, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm, 
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    
    // Tell Bevy this image is going to be written to by a Compute Shader
    image.texture_descriptor.usage |= TextureUsages::STORAGE_BINDING;
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());
    let texture = images.add(image);

    commands.insert_resource(SandGridGPU { 
        config_buffer,
        read_buffer, 
        write_buffer, 
        texture: texture.clone() 
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