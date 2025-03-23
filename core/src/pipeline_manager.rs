use log::info;
use wgpu::{BufferAddress, Device, FragmentState, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexBufferLayout, VertexState};

use crate::{
    buffer_manager::BufferManager,
    // texture_manager::TextureManager
};

pub struct PipelineManager {
    pub shader: ShaderModule,
    pub pipeline: RenderPipeline,
}

impl PipelineManager {
    pub fn new(device: &Device, swapchain_format: TextureFormat, buffers: &BufferManager) -> Self {
    // pub fn new(device: &Device, swapchain_format: TextureFormat, buffers: &BufferManager, textures: &TextureManager) -> Self {
        let shader_code = include_str!("./shader/default.wgsl");
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: ShaderSource::Wgsl(shader_code.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&buffers.uniforms.bind_group_layout, /*&textures.bind_group_layout*/],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[
                    VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 2]>() as BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x2] // matches @position(0)
                    }
                ],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[
                    Some(swapchain_format.into())
                ],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        info!("Pipeline created successfully!!!");

        Self { shader, pipeline }
    }
}
