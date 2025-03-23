use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device, Queue, ShaderStages};
use crate::utils;

// Uniforms
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ScreenSize {
    pub width: f32,
    pub height: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TimeUniform {
    pub time: f32,
}

pub struct UniformManager {
    pub screen_size_buffer: Buffer,
    pub time_buffer: Buffer,
    pub bind_group: BindGroup,
    pub bind_group_layout: BindGroupLayout,
}

impl UniformManager {
    pub fn new(device: &Device, width: f32, height: f32) -> Self {
        let screen_size = ScreenSize { width, height };
        let time = TimeUniform { time: 0.0 };

        let screen_size_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Screen Size Buffer"),
            contents: utils::as_byte_array(&screen_size),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let time_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Time Buffer"),
            contents: utils::as_byte_array(&time),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[
                // screen size buffer
                BindGroupLayoutEntry {
                    binding: 0, // matches @binding(0)
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None,
                },
                // time buffer
                BindGroupLayoutEntry {
                    binding: 1, // matches @binding(1)
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: screen_size_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: time_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            screen_size_buffer,
            time_buffer,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn update_time(&self, queue: &Queue, time: f32) {
        let time_data = TimeUniform { time };
        queue.write_buffer(&self.time_buffer, 0, utils::as_byte_array(&time_data));
    }
}

// Buffers
pub struct BufferManager {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub index_length: u32,
    pub uniforms: UniformManager,
}
impl BufferManager {
    pub fn fullscreen_buffer(device: &Device, width: f32, height: f32) -> Self {

        let vertices_data: [[f32; 2]; 4] = [
            [-1.0, -1.0],
            [1.0, -1.0],
            [-1.0, 1.0],
            [1.0, 1.0],
        ];
        let index_data: [[u16; 3]; 2] = [
            [0, 1, 2],
            [3, 1, 2],
        ];
        let index_length = index_data.len() as u32 * 3; // flattened size of index_data

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: utils::as_byte_array(&vertices_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: utils::as_byte_array(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        let uniforms = UniformManager::new(device, width, height);

        Self {
            vertex_buffer,
            index_buffer,
            index_length,
            uniforms,
        }
    }
}

