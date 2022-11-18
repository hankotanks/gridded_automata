use gridded_automata::{
    run,
    automata, 
    Config, 
    ColorScheme, 
    rules
};

fn main() {
    let automata = automata::rand_automata(
        automata::Size { width: 512, height: 512 },
        None,
        &[(0..2, 1.0)]
    );

    use rules::{ symmetry, Neighborhood, Rule };
    let rules = rules::RuleList::from_rules(symmetry::ASYMMETRIC, Neighborhood::Moore, vec![
        ((1..6, 2..7).into(), Rule::Counting { state: None, count: 2 } ),
        ((1..6, 2..7).into(), Rule::Counting { state: None, count: 3 } ),
        ((6, 6).into(), Rule::Counting { state: None, count: 2 } ),
        ((6, 6).into(), Rule::Counting { state: None, count: 3 } ),
        ((0, 1).into(), Rule::Counting { state: None, count: 3 } ),
        ((1..7, 0).into(), Rule::Decay)
    ]);

    let config = Config {
        title: Some("Conway's Game of Life".into()),
        fps: 60,
        state_shader: rules::compile_compute_shader(rules),//include_str!("cgol.wgsl").into(),
        coloring: ColorScheme::Lerp { start: [1.0, 0.0, 0.0], end: [0.0, 0.1, 1.0], states: 6u32 }
    };
    
    pollster::block_on(run(automata, config));
}