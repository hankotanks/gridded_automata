use gridded_automata::{
    run,
    automata, 
    Config, 
    color, 
    Neighborhood
};

fn main() {
    let automata = automata::random_automata_with_padding(
        automata::Size { width: 512, height: 512 },
        &[0, 1, 2],
        192
    );

    let config = Config {
        title: Some("Seeds".into()),
        fps: 60,
        state_shader: include_str!("seeds.wgsl").into(),
        coloring: &[color::alive([1.0; 3])],
        neighborhood: Neighborhood::Moore
    };
    
    pollster::block_on(run(automata, config));
}