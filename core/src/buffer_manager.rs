use log::info;
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Device, Queue, ShaderStages};
use crate::utils;

// Uniforms
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct StaticUniform {
    pub screen_width: f32,
    pub screen_height: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DynamicUniform {
    pub time: f32,
    pub delta_time: f32,
    pub mouse_position: [f32; 2],
}

pub struct UniformManager {
    pub static_uniform_data: StaticUniform,
    pub static_uniform_buffer: Buffer,
    pub dynamic_uniform_data: DynamicUniform,
    pub dynamic_uniform_buffer: Buffer,
    pub bind_group: BindGroup,
    pub bind_group_layout: BindGroupLayout,
}

impl UniformManager {
    pub fn new(device: &Device, width: f32, height: f32) -> Self {
        let static_uniform_data = StaticUniform {
            screen_width: width,
            screen_height: height
        };
        let dynamic_uniform_data = DynamicUniform {
            time: 0.0,
            delta_time: 0.0,
            mouse_position: [0.0, 0.0],
        };

        let static_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Static Uniform Buffer"),
            contents: utils::as_byte_array(&static_uniform_data),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let dynamic_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Dynamic Uniform Buffer"),
            contents: utils::as_byte_array(&dynamic_uniform_data),
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
                    resource: static_uniform_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: dynamic_uniform_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            static_uniform_data,
            static_uniform_buffer,
            dynamic_uniform_data,
            dynamic_uniform_buffer,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn update(
        &mut self,
        queue: &Queue,
        time: Option<f32>,
        delta_time: Option<f32>,
        mouse_pos_x: Option<f32>,
        mouse_pos_y: Option<f32>
    ) {
        let mouse_position = if let (Some(x), Some(y)) = (mouse_pos_x, mouse_pos_y) {
            [x, y]
        } else {
            self.dynamic_uniform_data.mouse_position
        };
        let dynamic_uniform_data = DynamicUniform {
            time: time.unwrap_or(self.dynamic_uniform_data.time),
            delta_time: delta_time.unwrap_or(self.dynamic_uniform_data.delta_time),
            mouse_position
        };
        self.dynamic_uniform_data = dynamic_uniform_data;
        queue.write_buffer(
            &self.dynamic_uniform_buffer,
            0,
            utils::as_byte_array(&dynamic_uniform_data)
        );
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

