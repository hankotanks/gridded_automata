mod state;

mod vertex;
pub(crate) use vertex::Vertex;
pub(crate) use vertex::CLIP_SPACE_EXTREMA;

pub mod automata;

use std::{
    time,
    borrow::Cow
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
    pub coloring: ColorScheme
}

pub enum ColorScheme {
    Lerp { start: [f32; 3], end: [f32; 3], num_states: u32 },
    Living([f32; 3]),
    Map(Vec<(u32, [f32; 3])>)
}

impl ColorScheme {
    fn get_color(&self) -> Cow<'static, str> {
        // Returns the shader code that determines how cells are colored
        // This is inserted between the ca_header and ca_tail blocks
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
            },
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
            Self::Map(color_map) => { 
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
            }
        }.into()
    }
}

pub async fn run(automata: automata::Automata, config: Config) {
    let event_loop = event_loop::EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(config.title.clone().unwrap_or_default())
        .build(&event_loop)
        .unwrap();

    // The shader is built at runtime to support any given coloring scheme
    let shader_descriptor = wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(
            vec![
                include_str!("./compute/header.wgsl"),
                &config.coloring.get_color(),
                &config.state_shader,
                include_str!("./compute/tail.wgsl")
            ].join("\n").into()
        )
    };

    // The State struct holds all of the programs mutable state
    let mut state = state::State::new(
        &window,
        shader_descriptor,
        automata
    ).await;

    // A few variables to keep frame-time consistent when performance allows
    let fps = (config.fps as f32).recip();
    let mut accumulated_time = 0.0;
    let mut current = time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Increment accumulated time for this pass
        accumulated_time += current.elapsed().as_secs_f32();
        current = time::Instant::now();
        
        match event {
            // Basic window behavior: resizing and closing
            event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = event_loop::ControlFlow::Exit; },
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size); },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size); },
                    _ => {}
                }
            },

            // Called after resizes and after simulation updates
            event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                match state.render() {
                    Ok(_) => {  },
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.physical_size); }, 
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        *control_flow = event_loop::ControlFlow::Exit; }, 
                    Err(wgpu::SurfaceError::Timeout) => {  },
                }
            },

            // Simulation updates occur when
            // the accumulated time exceeds the time-per-frame
            event::Event::MainEventsCleared => { 
                if accumulated_time >= fps {
                    state.tick();
                    accumulated_time -= fps;
                }
                
                window.request_redraw();
            },

            // No other events are caught
            _ => {}
        }
    } );
}
