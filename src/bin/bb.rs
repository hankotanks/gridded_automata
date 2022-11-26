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
        240
    );

    let config = Config {
        title: Some("Brian's Brain".into()),
        fps: 60,
        state_shader: include_str!("bb.wgsl").into(),
        coloring: &[
            color::map(1, [0.0, 0.0, 1.0]),
            color::map(2, [0.0, 1.0, 0.0])
        ],
        neighborhood: Neighborhood::Moore
    };
    
    pollster::block_on(run(automata, config));
}