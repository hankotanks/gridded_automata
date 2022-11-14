mod state;

mod vertex;
pub(crate) use vertex::Vertex;

pub mod automata;

use std::{borrow::Cow, time};

use winit::{
    window::WindowBuilder,
    event,
    event::WindowEvent,
    event_loop
};

pub async fn run(automata: automata::Automata, compute_shader_file: Cow<'static, str>, fps: u32) {
    let event_loop = event_loop::EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    let compute = wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(compute_shader_file)
    };

    let mut state = state::State::new(
        &window,
        compute,
        automata
    ).await;

    let fps = (fps as f32).recip();
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
                /* TODO */ let render_time = time::Instant::now();

                match state.render() {
                    Ok(_) => {  },
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.physical_size), 
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = event_loop::ControlFlow::Exit, 
                    Err(wgpu::SurfaceError::Timeout) => {  },
                }

                /* TODO */ dbg!(render_time.elapsed());
            }
            event::Event::MainEventsCleared => { 
                if accumulated_time >= fps {

                    /* TODO */ let automata_time = time::Instant::now();
                    
                    state.tick();

                    /* TODO */ dbg!(automata_time.elapsed());

                    accumulated_time -= fps;
                }


                window.request_redraw();
            },
            _ => {}
        }
    });
}
