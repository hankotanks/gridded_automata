use gridded_automata::{
    run,
    automata, 
    Config, 
    ColoringScheme
};

fn main() {
    let automata = automata::rand_automata(automata::Size {
        height: 512,
        width: 512
    } );

    let config = Config {
        title: Some("Conway's Game of Life".into()),
        fps: 60,
        state_shader: include_str!("cgol.wgsl").into(),
        coloring: ColoringScheme::Lerp { start: [1.0, 0.0, 0.0], end: [0.0, 0.1, 1.0], num_states: 6u32 }
    };
    
    pollster::block_on(run(
        automata,
        config
    ));
}