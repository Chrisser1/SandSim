use bevy::{
    core_pipeline::core_2d::graph::{Core2d, Node2d}, prelude::*, render::{
        Render, RenderApp, render_asset::RenderAssets, render_graph::{NodeRunError, RenderGraph, RenderGraphContext, RenderLabel}, render_resource::*, renderer::{RenderContext, RenderDevice}, storage::GpuShaderStorageBuffer, texture::GpuImage
    }
};
use crate::render::{SandGridGPU, GRID_WIDTH, GRID_HEIGHT};

/// Label for our node in the Render Graph
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct SandComputeLabel;

pub struct SandComputePlugin;

impl Plugin for SandComputePlugin {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return; };

        render_app.add_systems(Render, prepare_bind_group);
            
        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        
        if let Some(core_2d_graph) = render_graph.get_sub_graph_mut(Core2d) {
            core_2d_graph.add_node(SandComputeLabel, SandNode);
            core_2d_graph.add_node_edge(SandComputeLabel, Node2d::StartMainPass);
        }
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return; };
        render_app.init_resource::<SandPipeline>();
    }
}

/// The Pipeline Resource: Loads the shader and queues it for compilation
#[derive(Resource)]
struct SandPipeline {
    pipeline: CachedComputePipelineId,
    bind_group_layout: BindGroupLayout, // We store the constructed Bevy layout here!
}

impl FromWorld for SandPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        
        // Define our bindings once
        let entries = BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                binding_types::storage_buffer_read_only::<crate::render::GridConfig>(false),
                binding_types::storage_buffer_read_only::<crate::matter::Matter>(false),
                binding_types::storage_buffer::<crate::matter::Matter>(false),
                binding_types::texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::WriteOnly),
            ),
        );

        // 1. Create the Blueprint (Descriptor) for the Pipeline
        let layout_descriptor = BindGroupLayoutDescriptor {
            label: "sand_bind_group_layout".into(),
            entries: std::borrow::Cow::Owned::<[BindGroupLayoutEntry]>(entries.to_vec()).into(),
        };

        // 2. Create the Constructed Layout for the Bind Group
        let bind_group_layout = render_device.create_bind_group_layout(
            Some("sand_bind_group_layout"),
            &entries,
        );

        let shader = world.resource::<AssetServer>().load("shaders/sand.wgsl");
        let pipeline_cache = world.resource::<PipelineCache>();
        
        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("sand_pipeline".into()), 
            layout: vec![layout_descriptor], // Give the pipeline the Blueprint!
            shader,
            shader_defs: vec![],
            entry_point: Some("main".into()),
            push_constant_ranges: vec![],
            zero_initialize_workgroup_memory: false,
        });

        SandPipeline { pipeline, bind_group_layout }
    }
}

/// The actual Bind Group we create every frame from the swapped handles
#[derive(Resource)]
struct SandBindGroup(BindGroup);

fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<SandPipeline>,
    render_device: Res<RenderDevice>,
    gpu_data: Option<Res<SandGridGPU>>,
    buffers: Res<RenderAssets<GpuShaderStorageBuffer>>, 
    images: Res<RenderAssets<GpuImage>>,
) {
    let Some(gpu_data) = gpu_data else { return; };

    let (Some(config), Some(read), Some(write), Some(image)) = (
        buffers.get(&gpu_data.config_buffer),
        buffers.get(&gpu_data.read_buffer),
        buffers.get(&gpu_data.write_buffer),
        images.get(&gpu_data.texture),
    ) else {
        return; 
    };

    let bind_group = render_device.create_bind_group(
        Some("sand_bind_group"),
        &pipeline.bind_group_layout, // Use the proper Constructed Bevy Layout!
        &BindGroupEntries::sequential((
            config.buffer.as_entire_binding(),
            read.buffer.as_entire_binding(),
            write.buffer.as_entire_binding(),
            &image.texture_view,
        )),
    );

    commands.insert_resource(SandBindGroup(bind_group));
}

/// The Render Graph Node: This executes the shader
struct SandNode;

impl bevy::render::render_graph::Node for SandNode {
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();
        let sand_pipeline = world.resource::<SandPipeline>();
        let Some(bind_group) = world.get_resource::<SandBindGroup>() else { return Ok(()) };

        if let Some(pipeline) = pipeline_cache.get_compute_pipeline(sand_pipeline.pipeline) {
            let mut pass = render_context.command_encoder().begin_compute_pass(&ComputePassDescriptor::default());
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &bind_group.0, &[]);
            
            pass.dispatch_workgroups(GRID_WIDTH / 16, GRID_HEIGHT / 16, 1);
        }
        Ok(())
    }
}