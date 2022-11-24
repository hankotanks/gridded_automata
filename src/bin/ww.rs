use gridded_automata::{
    run,
    color,
    automata::automata_from_pgm, 
    Config, 
    Neighborhood
};

fn main() {
    let automata = automata_from_pgm("./src/bin/ww_p2.pgm").unwrap();
    
    let config = Config {
        title: Some("Wire World".into()),
        fps: 30,
        state_shader: include_str!("ww.wgsl").into(),
        coloring: &[
            color::map(1, [1.0, 0.4, 0.0]),
            color::map(2, [1.0; 3]),
            color::map(3, [0.0, 0.2, 1.0])
        ],
        neighborhood: Neighborhood::Moore
    };
    
    pollster::block_on(run(automata, config));
}