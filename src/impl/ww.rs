use gridded_automata::{
    run,
    automata, 
    Config
};

fn main() {
    let (automata, cs) = automata::read_automata_from_file(
        image::open("./src/impl/ww_sample_1.bmp").unwrap()
    );

    let config = Config {
        title: Some("Wire World".into()),
        fps: 30,
        state_shader: include_str!("ww.wgsl").into(),
        coloring: cs
    };
    
    pollster::block_on(run(automata, config));
}