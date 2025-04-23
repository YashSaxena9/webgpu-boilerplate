mod utils;
mod gpu_context;
// mod texture_manager;
mod buffer_manager;
mod pipeline_manager;
mod renderer;

// use texture_manager::TextureManager;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use gpu_context::GpuContext;
use buffer_manager::BufferManager;
use pipeline_manager::PipelineManager;
use renderer::Renderer;
use log::info;
use wgpu::TextureFormat;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct RequestedInput {
    pub mouse_position: bool,
    pub elapsed_time: bool,
    pub delta_time: bool,
}

#[wasm_bindgen]
pub struct App {
    pub requested_input: RequestedInput,
    gpu: GpuContext,
    buffers: BufferManager,
    // textures: TextureManager,
    pipeline: PipelineManager,
    renderer: Renderer,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen]
    pub async fn setup(canvas: HtmlCanvasElement) -> App {
        console_log::init().expect("Could not init logger!!!");
        info!("Setting up webgpu");
        let width = canvas.width();
        let height = canvas.height();

        let gpu = GpuContext::new(canvas, width, height).await;

        let buffers = BufferManager::fullscreen_buffer(&gpu.device, width as f32, height as f32);
        // let textures = TextureManager::new(&gpu.device);
        // let pipeline = PipelineManager::new(&gpu.device, TextureFormat::Bgra8Unorm, &buffers, &textures);
        let pipeline = PipelineManager::new(&gpu.device, TextureFormat::Bgra8Unorm, &buffers);
        let renderer = Renderer::new();
        App {
            gpu,
            buffers,
            // textures,
            pipeline,
            renderer,
            requested_input: RequestedInput {
                mouse_position: true,
                elapsed_time: true,
                delta_time: true,
            },
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self, time: Option<f32>, delta_time: Option<f32>, mouse_pos_x: Option<f32>, mouse_pos_y: Option<f32>) {
        self.buffers.uniforms.update(&self.gpu.queue, time, delta_time, mouse_pos_x, mouse_pos_y);
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        self.renderer.render(&self.gpu, &self.buffers, &self.pipeline);
        // self.renderer.render(&self.gpu, &self.buffers, &self.textures, &self.pipeline);
    }
}