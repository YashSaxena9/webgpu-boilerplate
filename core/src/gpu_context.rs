use wasm_bindgen::UnwrapThrowExt;
use log::{error, info};
use web_sys::HtmlCanvasElement;
use wgpu::{Adapter, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, MemoryHints, Queue, RequestAdapterOptions, Surface, SurfaceTarget};

pub struct GpuContext {
    pub instance: Instance,
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub adapter: Adapter,
}

impl GpuContext {
    pub async fn new(canvas: HtmlCanvasElement, width: u32, height: u32) -> Self {
        let instance = Instance::new(&InstanceDescriptor::from_env_or_default());
        let surface_target = SurfaceTarget::Canvas(canvas);
        let surface = instance.create_surface(surface_target)
            .inspect_err(|err| error!("Unable to create render surface!!! {:?}", err))
            .unwrap();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect_throw("No GPU adaptor found!");
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("WebGPU Device"),
                required_features: Features::default(),
                required_limits: Limits::downlevel_defaults(),
                memory_hints: MemoryHints::MemoryUsage
            }, None)
            .await
            .expect_throw("Failed to create WebGPU device!");
        let surface_config = surface
            .get_default_config(&adapter, width, height)
            .expect_throw("Unable to get default config for surface!");
        surface.configure(&device, &surface_config);
        info!("WebGPU initialized successfully!");
        Self {
            instance,
            device,
            queue,
            surface,
            adapter
        }
    }
}