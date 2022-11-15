use gridded_automata::{Config, automata, ColoringScheme, run};

fn main() {
    let (automata, dict) = automata::read_automata_from_file(
        image::open("./src/impl/wire_world_sample.bmp").unwrap()
    );

    let config = Config {
        title: Some("Wire World".into()),
        fps: 60,
        state_shader: include_str!("wire_world.wgsl").into(),
        coloring: ColoringScheme::Dictionary(dict)
    };
    
    pollster::block_on(run(automata, config));
}