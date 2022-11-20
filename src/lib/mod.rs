mod state;

mod vertex;
use color::color_shader;
pub(crate) use vertex::Vertex;
pub(crate) use vertex::CLIP_SPACE_EXTREMA;

pub mod automata;
pub mod color;

use std::{
    time,
    borrow::Cow
};

use winit::{
    dpi,
    window::WindowBuilder,
    event,
    event::WindowEvent,
    event_loop
};

pub struct Config<'a> {
    pub title: Option<Cow<'static, str>>,
    pub fps: u32,
    pub state_shader: Cow<'static, str>,
    pub coloring: &'a [color::Coloring]
}

pub async fn run<'a>(automata: automata::Automata, config: Config<'a>) {
    let event_loop = event_loop::EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(config.title.clone().unwrap_or_default())
        .with_inner_size::<dpi::Size>(automata.size.into())
        .build(&event_loop)
        .unwrap();

    // The shader is built at runtime to support any given coloring scheme
    let shader_descriptor = wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(
            vec![
                include_str!("./compute/header.wgsl"),
                &color_shader(config.coloring.to_vec()),
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
