use std::{
    iter,
    sync::{Arc, Mutex},
    cell::Cell
};

use wgpu::util::DeviceExt;

use crate::{
    Vertex, 
    automata::Automata
};

pub(crate) struct State {
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    pub(crate) surface: wgpu::Surface,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) render_pipeline: wgpu::RenderPipeline,
    pub(crate) automata: Automata,
    pub(crate) dim_bind_group: wgpu::BindGroup,
    pub(crate) a_buffer: wgpu::Buffer,
    pub(crate) b_buffer: wgpu::Buffer,
    pub(crate) a_bind_group: wgpu::BindGroup,
    pub(crate) b_bind_group: wgpu::BindGroup,
    pub(crate) texture_bind_group: wgpu::BindGroup,
    pub(crate) compute_pipeline: wgpu::ComputePipeline
}

impl State {
    pub(crate) async fn new(
        window: &winit::window::Window, 
        compute: wgpu::ShaderModuleDescriptor<'static>,
        automata: Automata
    ) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None
        ).await.unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo
        };

        surface.configure(&device, &surface_config);

        let shader_file = include_str!("shader.wgsl");
        let shader = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(shader_file.into()),
            }
        );

        let dim_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[automata.size]),
                usage: wgpu::BufferUsages::UNIFORM,
            }
        );

        let dim_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
                    count: None,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    }
                }]
            }
        );

        let dim_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: None,
                layout: &dim_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: dim_buffer.as_entire_binding()
                }]
            }
        );

        let extent = wgpu::Extent3d {
            width: automata.size.width,
            height: automata.size.height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: None,
                size: extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R32Float,
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            }
        );

        let texture_view = texture.create_view(
            &wgpu::TextureViewDescriptor {
                label: None,
                format: Some(wgpu::TextureFormat::R32Float),
                dimension: Some(wgpu::TextureViewDimension::D2),
                aspect: wgpu::TextureAspect::All,
                base_mip_level: 0,
                mip_level_count: std::num::NonZeroU32::new(1),
                base_array_layer: 0,
                array_layer_count: std::num::NonZeroU32::new(1),
                
            }
        );

        let texture_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None
                    }
                ],
            }
        );

        let texture_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: None,
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    }
                ],
            }
        );

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&dim_bind_group_layout, &texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::description()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_config.format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            }
        );

        let a_bytes = automata.data
            .iter()
            .map(|x| u32::to_ne_bytes(*x)) 
            .flat_map(|f| f.into_iter())
            .collect::<Vec<_>>();

        let a_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &a_bytes,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::MAP_READ
            }
        );

        let b_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: None,
                size: a_bytes.len() as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }  
        );

        let layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        count: None,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            has_dynamic_offset: false,
                            min_binding_size: None,
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                        }
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        count: None,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            has_dynamic_offset: false,
                            min_binding_size: None,
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                        }
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            format: wgpu::TextureFormat::R32Float,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    }
                ],
            }
        );

        let a_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: a_buffer.as_entire_binding()
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: b_buffer.as_entire_binding()
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&texture_view)
                    }
                ],
            }  
        );

        let b_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: b_buffer.as_entire_binding()
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: a_buffer.as_entire_binding()
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&texture_view)
                    }
                ],
            }  
        );

        let compute_pipeline = device.create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                label: None,
                layout: Some(&{
                    device.create_pipeline_layout(
                        &wgpu::PipelineLayoutDescriptor {
                            label: None,
                            bind_group_layouts: &[&dim_bind_group_layout, &layout],
                            push_constant_ranges: &[]
                        } 
                    )
                } ),
                module: &device.create_shader_module(compute),
                entry_point: "main_cs",
            }
        );

        Self {
            size,
            surface,
            surface_config,
            device,
            queue,
            render_pipeline,
            automata,
            dim_bind_group,
            a_buffer,
            b_buffer,
            a_bind_group,
            b_bind_group,
            texture_bind_group,
            compute_pipeline
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub(crate) fn tick(&mut self) {
        let descriptor = wgpu::CommandEncoderDescriptor { label: None };
        let mut encoder = self.device.create_command_encoder(&descriptor);

        {
            let mut compute_pass = encoder.begin_compute_pass(
                &wgpu::ComputePassDescriptor { label: None }
            );

            // TODO
            compute_pass.set_bind_group(0, &self.dim_bind_group, &[]);
            compute_pass.set_bind_group(1, &self.a_bind_group, &[]);
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.dispatch_workgroups(
                self.automata.size.width / 16, 
                self.automata.size.height / 16, 
                1
            );
        }

        // Wait for GPU to finish
        self.queue.submit(Some(encoder.finish()));

        let buffer_slice = self.b_buffer.slice(..);

        let ready = Arc::new(Mutex::new(Cell::new(false)));
        let ready_ref = Arc::clone(&ready);
        buffer_slice.map_async(wgpu::MapMode::Read, move |_| { 
            ready_ref.lock().unwrap().set(true);
        } );

        self.device.poll(wgpu::Maintain::Wait);

        if ready.lock().unwrap().get() {
            let data = buffer_slice.get_mapped_range();
            let result = data
                .chunks_exact(4)
                .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
                .collect::<Vec<_>>();
            
            drop(data);
            self.b_buffer.unmap();

            self.automata.data = result;
        }

        std::mem::swap(&mut self.a_buffer, &mut self.b_buffer);
        std::mem::swap(&mut self.a_bind_group, &mut self.b_bind_group);
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        
        let vertices = vec![
            Vertex { position: [-1.0, -1.0, 0.0], tex: [0.0; 2] },
            Vertex { position: [1.0, -1.0, 0.0], tex: [1.0, 0.0] },
            Vertex { position: [-1.0, 1.0, 0.0], tex: [0.0, 1.0] },
            Vertex { position: [1.0, 1.0, 0.0], tex: [1.0; 2] },
        ];

        let indices = vec![0, 1, 3, 0, 3, 2];

        let vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let index_count = indices.len() as u32;

        let index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let descriptor = wgpu::CommandEncoderDescriptor { label: None };
        let mut encoder = self.device.create_command_encoder(&descriptor);
    
        {
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: true,
                            },
                        } )
                    ],
                    depth_stencil_attachment: None,
                }
            );

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.dim_bind_group, &[]);
            render_pass.set_bind_group(1, &self.texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                index_buffer.slice(..), 
                wgpu::IndexFormat::Uint32
            );

            render_pass.draw_indexed(0..index_count, 0, 0..1);
            
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}