mod state;

mod vertex;
pub(crate) use vertex::Vertex;
pub(crate) use vertex::CLIP_SPACE_EXTREMA;

pub mod automata;

use std::collections::HashMap;
use std::{
    borrow::Cow, 
    time
};

use winit::{
    window::WindowBuilder,
    event,
    event::WindowEvent,
    event_loop
};

pub struct Config {
    pub title: Option<Cow<'static, str>>,
    pub fps: u32,
    pub state_shader: Cow<'static, str>,
    pub coloring: ColoringScheme
}

pub enum ColoringScheme {
    Lerp { start: [f32; 3], end: [f32; 3], num_states: u32 },
    Living([f32; 3]),
    Dictionary(HashMap<u32, [f32; 3]>)
}

impl ColoringScheme {
    fn get_color(&self) -> Cow<'static, str> {
        match &self {
            Self::Lerp { start, end, num_states } => {
                format!(
                    "fn get_color(state: u32) -> vec3<f32> {{
                        if(state == 0u) {{ return vec3<f32>(0.0, 0.0, 0.0); }}
                        let s = f32(state) / f32({});
                        return mix({}, {}, vec3<f32>(s, s, s));
                    }}", 
                    num_states, 
                    format!(
                        "vec3<f32>({:?}, {:?}, {:?})", 
                        start[0], 
                        start[1], 
                        start[2]
                    ), 
                    format!(
                        "vec3<f32>({:?}, {:?}, {:?})", 
                        end[0], 
                        end[1], 
                        end[2]
                    )
                )
            }
            Self::Living(alive) => {
                format!(
                    "fn get_color(state: u32) -> vec3<f32> {{
                        if(state == 0u) {{ return vec3<f32>(0.0, 0.0, 0.0); }}
                        return {};
                    }}",
                    format!(
                        "vec3<f32>({:?}, {:?}, {:?})", 
                        alive[0], 
                        alive[1], 
                        alive[2]
                    ), 
                )
            },
            Self::Dictionary(color_map) => { 
                format!(        
                    "fn get_color(state: u32) -> vec3<f32> {{
                        {}
                        return vec3<f32>(0.0, 0.0, 0.0);
                    }}",
                    {
                        let mut conditionals = "".to_string();
                        for (state, color) in color_map.iter() {
                            conditionals.push_str(
                                &format!(
                                    "if(state == {}u) {{
                                        return {};
                                    }}",
                                    state,
                                    format!(
                                        "vec3<f32>({:?}, {:?}, {:?})", 
                                        color[0], 
                                        color[1], 
                                        color[2]
                                    )
                                )
                            );
                        }
                        conditionals
                    }
                )
            },
        }.into()
    }
}

pub async fn run(automata: automata::Automata, config: Config) {
    let event_loop = event_loop::EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(config.title.clone().unwrap_or_default())
        .build(&event_loop)
        .unwrap();

    let shader_descriptor = wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(
            vec![
                include_str!("ca_header.wgsl"),
                &config.coloring.get_color(),
                &config.state_shader,
                include_str!("ca_caller.wgsl")
            ].join("\n").into()
        )
    };

    let mut state = state::State::new(
        &window,
        shader_descriptor,
        automata
    ).await;

    let fps = (config.fps as f32).recip();
    let mut accumulated_time = 0.0;
    let mut current = time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        accumulated_time += current.elapsed().as_secs_f32();
        current = time::Instant::now();
        
        match event {
            event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = event_loop::ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => state.resize(*physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => state.resize(**new_inner_size),
                    _ => {}
                }
            }
            event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                
                /* TODO */ //let render_time = time::Instant::now();

                match state.render() {
                    Ok(_) => {  },
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.physical_size), 
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = event_loop::ControlFlow::Exit, 
                    Err(wgpu::SurfaceError::Timeout) => {  },
                }

                /* TODO */ //dbg!(render_time.elapsed());

            }
            event::Event::MainEventsCleared => { 
                if accumulated_time >= fps {

                    /* TODO */ //let automata_time = time::Instant::now();
                    
                    state.tick();

                    /* TODO */ //dbg!(automata_time.elapsed());

                    accumulated_time -= fps;
                }


                window.request_redraw();
            },
            _ => {}
        }
    });
}
