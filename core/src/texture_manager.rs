// use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Device, Queue, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, Texture, TextureSampleType, TextureView, TextureViewDescriptor, TextureViewDimension};

// pub struct TextureManager {
//     pub textures: Vec<Texture>,
//     pub texture_views: Vec<TextureView>,
//     pub samplers: Vec<Sampler>,
//     pub bind_group: BindGroup,
//     pub bind_group_layout: BindGroupLayout,
// }

// impl TextureManager {
//     pub fn new(device: &Device) -> Self {
//         let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
//             label: Some("Texture Bind Group Layout"),
//             entries: &[
//                 BindGroupLayoutEntry {
//                     binding: 0,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Texture {
//                         sample_type: TextureSampleType::Float { filterable: true },
//                         view_dimension: TextureViewDimension::D2,
//                         multisampled: false,
//                     },
//                     count: None,
//                 },
//                 BindGroupLayoutEntry {
//                     binding: 1,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Sampler(SamplerBindingType::Filtering),
//                     count: None,
//                 }
//             ]
//         });
//         let bind_group = device.create_bind_group(&BindGroupDescriptor {
//             label: Some("Empty Texture Bind Group"),
//             layout: &bind_group_layout,
//             entries: &[],
//         });

//         Self {
//             textures: Vec::new(),
//             texture_views: Vec::new(),
//             samplers: Vec::new(),
//             bind_group,
//             bind_group_layout
//         }
//     }

//     pub fn update_textures(&mut self, device: &Device, queue: &Queue, new_textures: Vec<Texture>) {
//         self.textures = new_textures;
//         self.texture_views = self.textures.iter()
//             .map(|tex| {
//                 tex.create_view(&TextureViewDescriptor::default())
//             })
//             .collect();
//         self.samplers = self.textures.iter()
//             .map(|_| {
//                 device.create_sampler(&SamplerDescriptor::default())
//             })
//             .collect();
//         let bind_group_entities: Vec<BindGroupEntry> = self.texture_views.iter().zip(&self.samplers)
//             .enumerate()
//             .flat_map(|(idx, (view, sampler))| {
//                 vec![
//                     BindGroupEntry {
//                         binding: (idx * 2) as u32,
//                         resource: wgpu::BindingResource::TextureView(view),
//                     },
//                     BindGroupEntry {
//                         binding: (idx * 2 + 1) as u32,
//                         resource: wgpu::BindingResource::Sampler(sampler),
//                     },
//                 ]
//             })
//             .collect();
//         self.bind_group = device.create_bind_group(&BindGroupDescriptor {
//             label: Some("Updated Texture Bind Group"),
//             layout: &self.bind_group_layout,
//             entries: &bind_group_entities
//         });
//     }
// }