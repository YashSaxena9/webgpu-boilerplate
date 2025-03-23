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
pub struct App {
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
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self, time: f32) {
        self.buffers.uniforms.update_time(&self.gpu.queue, time);
    }

    #[wasm_bindgen]
    pub fn render(&self) {
        self.renderer.render(&self.gpu, &self.buffers, &self.pipeline);
        // self.renderer.render(&self.gpu, &self.buffers, &self.textures, &self.pipeline);
    }
}