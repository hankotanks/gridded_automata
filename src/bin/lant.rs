use gridded_automata::{
    run,
    automata, 
    color,
    Config, 
};

fn main() {
    let size = automata::Size { width: 128, height: 128 };

    let mut automata = automata::Automata::new(size);
    automata[(size.width/ 2, size.height / 2).into()] = 2;

    let config = Config {
        title: Some("Langton's Ant".into()),
        fps: 60,
        state_shader: include_str!("lant.wgsl").into(),
        coloring: &[
            color::map(0, [0.0; 3]),
            color::map_range(1..=4, [1.0, 0.0, 0.0]),
            color::map(5, [1.0; 3]),
            color::map_range(6..=9, [1.0, 0.0, 0.0])
        ]
    };

    /*
    ColorScheme::new(vec![
        map(0, [0.0; 3]),
        map(5, [1.0; 3]),
        map_range(1..=4, [1.0, 0.0, 0.0]),
        map_range(6..=9, [1.0, 0.0, 0.0])
    ])
     */
    
    pollster::block_on(run(automata, config));
}