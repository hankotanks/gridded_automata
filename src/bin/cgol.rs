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

    use rules::symmetry::*;
    use rules::Neighborhood;
    use rules::Rule;
    let mut rules = rules::RuleList::new(ASYMMETRIC, Neighborhood::Moore);
    for i in 1u32..6 {
        rules.add_rule(i, i + 1, Rule::Counting { state: None, count: 2 } );
        rules.add_rule(i, i + 1, Rule::Counting { state: None, count: 3 } );
    }
    rules.add_rule(6, 6, Rule::Counting { state: None, count: 2 } );
    rules.add_rule(6, 6, Rule::Counting { state: None, count: 3 } );
    rules.add_rule(0, 1, Rule::Counting { state: None, count: 3 } );
    for i in 1u32..=6 {
        rules.add_rule(i, 0, Rule::Decay);
    }

    let config = Config {
        title: Some("Conway's Game of Life".into()),
        fps: 60,
        state_shader: rules::compile_compute_shader(rules),//include_str!("cgol.wgsl").into(),
        coloring: ColorScheme::Lerp { start: [1.0, 0.0, 0.0], end: [0.0, 0.1, 1.0], states: 6u32 }
    };
    
    pollster::block_on(run(automata, config));
}