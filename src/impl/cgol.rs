use gridded_automata::{
    run,
    automata, 
    Config
};

fn main() {
    let automata = automata::rand_automata(automata::Size {
        height: 256,
        width: 256
    } );

    let config = Config {
        title: Some("Conway's Game of Life".into()),
        fps: 60,
        state_shader: include_str!("cgol.wgsl").into()
    };
    
    pollster::block_on(run(
        automata,
        config
    ));
}