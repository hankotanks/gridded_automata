use gridded_automata::{
    run,
    automata, 
    Config, 
    ColorScheme
};

fn main() {
    let automata = automata::rand_automata(
        automata::Size { width: 512, height: 512 },
        Some(automata::Size { width: 32, height: 32 } ),
        &[(0..2, 1.0)]
    );

    let config = Config {
        title: Some("Seeds".into()),
        fps: 60,
        state_shader: include_str!("seeds.wgsl").into(),
        coloring: ColorScheme::Living([1.0; 3])
    };
    
    pollster::block_on(run(automata, config));
}