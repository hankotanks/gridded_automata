use gridded_automata::{
    run,
    automata, 
    Config, 
    ColorScheme
};

fn main() {
    let automata = automata::rand_automata(automata::Size {
        width: 640,
        height: 512
    } );

    let config = Config {
        title: Some("Conway's Game of Life".into()),
        fps: 60,
        state_shader: include_str!("cgol.wgsl").into(),
        coloring: ColorScheme::Lerp { start: [1.0, 0.0, 0.0], end: [0.0, 0.1, 1.0], states: 6u32 }
    };
    
    pollster::block_on(run(automata, config));
}