use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
// use web_sys::GpuCanvasContext;
use wgpu::{util::DeviceExt, PipelineCompilationOptions, SurfaceTarget};
use log::info;

#[repr(C)]
struct ScreenSize {
    width: f32,
    height: f32,
}

#[wasm_bindgen]
pub async fn run(canvas: HtmlCanvasElement) {
    console_log::init().expect("Could not init logger!!!");
    info!("Running webgpu!!!");
    let width = canvas.width();
    let height = canvas.height();
    info!("window::width: {width}, window::height: {height}");
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());
    let surface_target = SurfaceTarget::Canvas(canvas);
    let surface = instance.create_surface(surface_target)
        .inspect_err(|err| info!("Failed to use canvas as webgpu surface: {:?}", err))
        .unwrap();

    let adaptor = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .ok_or_else(|| info!("No suitable GPU adaptor found"))
        .unwrap();

    let (device, queue) = adaptor
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("Device request"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults(),
            memory_hints: wgpu::MemoryHints::MemoryUsage,
        }, None)
        .await
        .inspect_err(|err| info!("Failed to create device: {:?}", err))
        .unwrap();

    let config = surface
        .get_default_config(&adaptor, width, height)
        .ok_or_else(|| info!("Unable to get default config for surface"))
        .unwrap();
    surface.configure(&device, &config);

    let vertex_data: [[f32; 2]; 4] = [
        [-1.0, -1.0],
        [1.0, -1.0],
        [-1.0, 1.0],
        [1.0, 1.0],
    ];
    let index_data: [u16; 6] = [
        0, 1, 2,
        3, 1, 2,
    ];
    let screen_size = ScreenSize {
        width: width as f32,
        height: height as f32,
    };
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertex_data),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&index_data),
        usage: wgpu::BufferUsages::INDEX,
    });
    let uniform_contents: [u8; 8] = unsafe { std::mem::transmute(screen_size) };
    info!("UNIFORM: {:?}", uniform_contents);
    let scren_size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Screen Size Buffer"),
        contents: &uniform_contents,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let screen_size_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Screen Size Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None
            },
            count: None,
        }]
    });
    let screen_size_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Screen Size Bind Group"),
        layout: &screen_size_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: scren_size_buffer.as_entire_binding(),
            }
        ]
    });

    let shader_code = include_str!("./shader/default.wgsl");
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Triangle Shader"),
        source: wgpu::ShaderSource::Wgsl(shader_code.into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline layout"),
        bind_group_layouts: &[&screen_size_bind_group_layout],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adaptor);
    let swapchain_format = swapchain_capabilities.formats[0];

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x2]
            }],
            compilation_options: PipelineCompilationOptions::default()
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: PipelineCompilationOptions::default(),
            targets: &[ Some(swapchain_format.into()) ]
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None
    });

    let frame = surface
        .get_current_texture()
        .inspect_err(|err| info!("Unable to create frame: {:?}", err))
        .unwrap();
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[
                Some(
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    }
                )
            ],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        render_pass.set_pipeline(&pipeline);
        render_pass.set_bind_group(0, &screen_size_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..index_data.len() as u32, 0, 0..1);
    }
    queue.submit(Some(encoder.finish()));
    frame.present();
}