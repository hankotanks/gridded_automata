use std::{
    iter,
    sync::{ Arc, Mutex },
    cell::Cell, 
    mem
};

use wgpu::util::DeviceExt;

use crate::{
    Vertex, 
    CLIP_SPACE_EXTREMA,
    automata
};

pub(crate) struct State {
    pub(crate) physical_size: winit::dpi::PhysicalSize<u32>,
    pub(crate) device: wgpu::Device,
    pub(crate) surface: wgpu::Surface,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) queue: wgpu::Queue,

    pub(crate) size_group: wgpu::BindGroup,
    pub(crate) automata: automata::Automata,
    pub(crate) cell_buffers: (wgpu::Buffer, wgpu::Buffer),
    pub(crate) cell_groups: (wgpu::BindGroup, wgpu::BindGroup),
    pub(crate) compute_texture_group: wgpu::BindGroup,
    pub(crate) compute_pipeline: wgpu::ComputePipeline,
    pub(crate) workgroup: u32,

    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) render_texture_group: wgpu::BindGroup,
    pub(crate) render_pipeline: wgpu::RenderPipeline,
}

impl State {
    pub(crate) async fn new(
        window: &winit::window::Window, 
        shader_descriptor: wgpu::ShaderModuleDescriptor<'static>,
        automata: automata::Automata
    ) -> Self {
        //
        // WGPU Mandatory State Information
        //

        let physical_size = window.inner_size();

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
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::Fifo
        };

        surface.configure(&device, &surface_config);

        //
        // GRAB SHADER
        //

        let shader_file = include_str!("render.wgsl");
        let shader = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(shader_file.into()),
            }
        );

        //
        // DIMENSION BUFFER AND BIND GROUPS
        //

        let size_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[automata.size]),
                usage: wgpu::BufferUsages::UNIFORM,
            }
        );

        let size_group_layout = device.create_bind_group_layout(
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

        let size_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: None,
                layout: &size_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: size_buffer.as_entire_binding()
                }]
            }
        );

        // 
        // OUTPUT TEXTURE CREATION
        //

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
                format: wgpu::TextureFormat::Rgba32Float,
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            }
        );

        let texture_view = texture.create_view(
            &wgpu::TextureViewDescriptor {
                label: None,
                format: Some(wgpu::TextureFormat::Rgba32Float),
                dimension: Some(wgpu::TextureViewDimension::D2),
                aspect: wgpu::TextureAspect::All,
                base_mip_level: 0,
                mip_level_count: std::num::NonZeroU32::new(1),
                base_array_layer: 0,
                array_layer_count: std::num::NonZeroU32::new(1),
                
            }
        );

        //
        // COMPUTE SHADER
        //

        let cell_buffers = (
            device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: &{
                        automata.data
                            .iter()
                            .map(|x| u32::to_ne_bytes(*x)) 
                            .flat_map(|f| f.into_iter())
                            .collect::<Vec<_>>()
                    },
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::MAP_READ
                }
            ), 
            device.create_buffer(
                &wgpu::BufferDescriptor {
                    label: None,
                    size: (automata.data.len() * 4) as wgpu::BufferAddress,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::MAP_READ,
                    mapped_at_creation: false,
                }  
            )
        );

        let cell_group_layout = device.create_bind_group_layout(
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
                    }
                ],
            }
        );

        let cell_groups = (
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &cell_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: cell_buffers.0.as_entire_binding()
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: cell_buffers.1.as_entire_binding()
                    }
                ]
            } ),
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &cell_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: cell_buffers.1.as_entire_binding()
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: cell_buffers.0.as_entire_binding()
                    }
                ]
            } )
        );

        let compute_texture_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            format: wgpu::TextureFormat::Rgba32Float,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    }
                ],
            }
        );

        let compute_texture_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: None,
                layout: &compute_texture_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
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
                            push_constant_ranges: &[],
                            bind_group_layouts: &[
                                &size_group_layout, 
                                &cell_group_layout, 
                                &compute_texture_group_layout
                            ]
                        } 
                    )
                } ),
                module: &device.create_shader_module(shader_descriptor),
                entry_point: "main_cs",
            }
        );

        let mut workgroup_size = 1u32;
        for i in 2..=16u32 {
            if automata.size.width % i == 0 && automata.size.height % i == 0 {
                workgroup_size = i;
            }
        }

        //
        // RENDER SHADER
        //

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(CLIP_SPACE_EXTREMA.as_slice()),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice([0, 1, 3, 0, 3, 2].as_slice()),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        let render_texture_group_layout = device.create_bind_group_layout(
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

        let render_texture_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: None,
                layout: &render_texture_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    }
                ],
            }
        );

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(
                    &device.create_pipeline_layout(
                        &wgpu::PipelineLayoutDescriptor {
                            label: None,
                            push_constant_ranges: &[],
                            bind_group_layouts: &[
                                &size_group_layout, 
                                &render_texture_group_layout
                            ]
                        }
                    )
                ),
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

        Self {
            physical_size,
            device,
            surface,
            surface_config,
            queue,
            size_group,
            automata,
            cell_buffers,
            cell_groups,
            compute_texture_group,
            compute_pipeline,
            workgroup: workgroup_size,
            vertex_buffer,
            index_buffer,
            render_texture_group,
            render_pipeline
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.physical_size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub(crate) fn tick(&mut self) {
        let desc = wgpu::CommandEncoderDescriptor { label: None };
        let mut encoder = self.device.create_command_encoder(&desc);

        {
            let desc = wgpu::ComputePassDescriptor { label: None };
            let mut compute_pass = encoder.begin_compute_pass(&desc);

            // Access to dimensions, cell arrays and...
            compute_pass.set_bind_group(0, &self.size_group, &[]);
            compute_pass.set_bind_group(1, &self.cell_groups.0, &[]);

            // ...output texture for the render attachment
            compute_pass.set_bind_group(2, &self.compute_texture_group, &[]);

            compute_pass.set_pipeline(&self.compute_pipeline);

            compute_pass.dispatch_workgroups(
                self.automata.size.width / self.workgroup, 
                self.automata.size.height / self.workgroup, 
                1
            );
        }

        // Wait for GPU to finish
        self.queue.submit(Some(encoder.finish()));

        // Get the updated buffer's data as a slice
        let buffer_slice = self.cell_buffers.1.slice(..);

        // Wait for the callback from map_async before proceeding
        let ready = Arc::new(Mutex::new(Cell::new(false)));
        let ready_ref = Arc::clone(&ready);
        buffer_slice.map_async(wgpu::MapMode::Read, move |_| { 
            ready_ref.lock().unwrap().set(true);
        } );

        // Wait until resources are cleaned up
        self.device.poll(wgpu::Maintain::Wait);

        // Read the cell data from the buffer slice
        if ready.lock().unwrap().get() {
            let data = buffer_slice.get_mapped_range();
            let result = data
                .chunks_exact(4)
                .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
                .collect::<Vec<_>>();
            
            drop(data);
            self.cell_buffers.1.unmap();

            self.automata.data = result;
        }

        // Swap the `current` and `updated` cell arrays for the next gen
        mem::swap(&mut self.cell_buffers.0, &mut self.cell_buffers.1);
        mem::swap(&mut self.cell_groups.0, &mut self.cell_groups.1);
    }
    
    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let desc = wgpu::CommandEncoderDescriptor { label: None };
        let mut encoder = self.device.create_command_encoder(&desc);
    
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

            // The render shader needs access to the field's dimensions...
            render_pass.set_bind_group(0, &self.size_group, &[]);

            // ...and the output texture from the compute shader
            render_pass.set_bind_group(1, &self.render_texture_group, &[]);

            // Setup the vertex and index buffers (which are constant)
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            
            // Finally, draw
            render_pass.draw_indexed(0..6, 0, 0..1); 
        }

        self.queue.submit(iter::once(encoder.finish()));
        
        output.present();

        Ok(())
    }
}