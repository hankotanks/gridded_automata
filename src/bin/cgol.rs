use gridded_automata::{
    run,
    automata, 
    Config, 
    color, 
    Neighborhood
};

fn main() {
    let automata = automata::random_automata(
        automata::Size { width: 512, height: 512 },
        &[0, 1]
    );

    let config = Config {
        title: Some("Conway's Game of Life".into()),
        fps: 60,
        state_shader: include_str!("cgol.wgsl").into(),
        coloring: &[color::lerp(1..=6, [1.0, 0.2, 0.0], [0.1, 0.2, 1.0])],
        neighborhood: Neighborhood::Moore
    };
    
    pollster::block_on(run(automata, config));
}