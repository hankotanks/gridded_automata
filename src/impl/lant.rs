use gridded_automata::{
    run,
    automata, 
    Config, 
    ColorScheme
};

fn main() {
    let size = automata::Size { width: 128, height: 128 };

    let mut automata = automata::Automata::new(size);
    automata[(size.width / 2, size.height / 2).into()] = 2;

    let config = Config {
        title: Some("Langton's Ant".into()),
        fps: 60,
        state_shader: include_str!("lant.wgsl").into(),
        coloring: ColorScheme::Map(vec![
            (0, [0.0; 3]), (1, [1.0, 0.0, 0.0]), (2, [1.0, 0.0, 0.0]), (3, [1.0, 0.0, 0.0]), (4, [1.0, 0.0, 0.0]),
            (5, [1.0; 3]), (6, [1.0, 0.0, 0.0]), (7, [1.0, 0.0, 0.0]), (8, [1.0, 0.0, 0.0]), (9, [1.0, 0.0, 0.0])
        ])
    };
    
    pollster::block_on(run(automata, config));
}