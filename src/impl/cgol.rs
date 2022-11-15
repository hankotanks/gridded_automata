use gridded_automata::{
    run,
    automata, Config
};

fn main() {
    let size = automata::Size { height: 256, width: 256 };

    let automata = automata::rand_automata(size);

    let config = Config {
        fps: 60,
        state_shader: include_str!("cgol.wgsl").into()
    };
    
    pollster::block_on(run(
        automata,
        config
    ));
}