use gridded_automata::{
    run,
    automata, 
    Config, 
    ColorScheme
};

fn main() {
    let automata = automata::rand_automata(
        automata::Size { width: 512, height: 512 },
        &[(0..3, 1.0)]
    );

    let config = Config {
        title: Some("Brian's Brain".into()),
        fps: 60,
        state_shader: include_str!("bb.wgsl").into(),
        coloring: ColorScheme::Map(vec![(2, [0.0, 0.4, 0.8]), (1, [0.0, 0.8, 0.4])])
    };
    
    pollster::block_on(run(automata, config));
}