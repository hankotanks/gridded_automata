use gridded_automata::{
    run,
    automata, 
    Config, 
    color
};

fn main() {
    let automata = automata::random_automata(
        automata::Size { width: 512, height: 512 },
        &[0, 1],
        0
    );

    let config = Config {
        title: Some("Conway's Game of Life".into()),
        fps: 60,
        state_shader: include_str!("cgol.wgsl").into(),
        coloring: &[color::lerp([1.0, 0.0, 0.0], [0.0, 0.1, 1.0], 1..=6)]
    };
    
    pollster::block_on(run(automata, config));
}