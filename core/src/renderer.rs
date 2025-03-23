use log::{info, error};
use wgpu::{Color, CommandEncoderDescriptor, IndexFormat, LoadOp, Operations, Queue, RenderPassColorAttachment, RenderPassDescriptor, StoreOp, TextureViewDescriptor};
use crate::buffer_manager::BufferManager;
use crate::gpu_context::GpuContext;
use crate::pipeline_manager::PipelineManager;
// use crate::texture_manager::TextureManager;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        info!("Renderer created successfully!!!");
        Self {}
    }

    pub fn render(
        &self,
        gpu: &GpuContext,
        buffers: &BufferManager,
        // textures: &TextureManager,
        pipeline: &PipelineManager
    ) {
        let frame = gpu.surface.get_current_texture()
            .inspect_err(|err| error!("Failed to acquire frame: {:?}", err))
            .unwrap();
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = gpu.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            // change of scope as encoder is borrowed here
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(Color::BLACK),
                            store: StoreOp::Store,
                        },
                    })
                ],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&pipeline.pipeline);
            render_pass.set_vertex_buffer(0, buffers.vertex_buffer.slice(..));
            render_pass.set_index_buffer(buffers.index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.set_bind_group(0, &buffers.uniforms.bind_group, &[]); // matches @group(0)
            // if !textures.textures.is_empty() {
            //     render_pass.set_bind_group(1, &textures.bind_group, &[]);
            // }
            render_pass.draw_indexed(0..buffers.index_length as u32, 0, 0..1);
        }

        gpu.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
