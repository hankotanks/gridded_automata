pub mod symmetry {
    pub struct Symmetry(u8);
    impl From<u8> for Symmetry {
        fn from(byte: u8) -> Self { Self(byte) }
    }

    pub const ASYMMETRIC: u8 = 0;
    pub const ROTATIONAL: u8 = 1 << 0;
    pub const FLIP_WIDTH: u8 = 1 << 1;
    pub const FLIP_HEIGHT: u8 = 1 << 2;
    pub const REFLECT: u8 = FLIP_WIDTH | FLIP_HEIGHT;
}

use std::{borrow, ops::Range, iter};

use symmetry::Symmetry;

pub enum Neighborhood {
    Moore,
    VonNeumann
}

struct Neighbors([u32; 8]);

pub enum Rule {
    Decay,
    Counting { state: Option<u32>, count: u32 }
    //Surroundings { neighbors: Neighbors }
}

pub struct StateChange(Vec<(u32, u32)>);

impl From<(u32, u32)> for StateChange {
    fn from(data: (u32, u32)) -> Self {
        Self(vec![data])
    }
}

impl From<(Range<u32>, Range<u32>)> for StateChange {
    fn from(data: (Range<u32>, Range<u32>)) -> Self {
        Self(data.0.zip(data.1).collect::<Vec<_>>())
    }
}

impl From<(Range<u32>, u32)> for StateChange {
    fn from(data: (Range<u32>, u32)) -> Self {
        Self(data.0.zip(iter::repeat(data.1)).collect::<Vec<_>>())
    }
}

pub struct RuleList {
    rules: Vec<(StateChange, Rule)>,
    symmetry: Symmetry,
    neighborhood: Neighborhood,
}

impl RuleList {
    pub fn new<S: Into<Symmetry>>(symmetry: S, neighborhood: Neighborhood) -> Self {
        Self {
            rules: Vec::new(),
            symmetry: symmetry.into(),
            neighborhood
        }
    }

    pub fn from_rules<S: Into<Symmetry>>(
        symmetry: S, 
        neighborhood: Neighborhood, 
        rules: Vec<(StateChange, Rule)>
    ) -> Self {
        Self { rules, symmetry: symmetry.into(), neighborhood }
    }

    pub fn add_rule(&mut self, state_change: StateChange, rule: Rule) {
        self.rules.push((state_change, rule));
    }
}

pub fn compile_compute_shader(rules: RuleList) -> borrow::Cow<'static, str> {
    let mut compute = Vec::new();
    compute.push("fn main(coord: vec2<i32>, state: u32) -> u32 {{".to_string());
    compute.push(match rules.neighborhood {
        Neighborhood::Moore => "    let adj = moore_neighborhood(coord);".to_string(),
        Neighborhood::VonNeumann => "    let adj = von_neumann_neighborhood(coord);".to_string()
    } );

    for (state_change, rule) in rules.rules.into_iter() {
        for (from, to) in state_change.0.into_iter() {
            compute.push(match rule {
                Rule::Decay => format!("    if state == {}u {{ return {}u; }}", from, to),
                Rule::Counting { state, count: occurrences } => match state {
                    Some(state) => format!(
                        "    if state == {}u && count_matching(adj, {}u) == {}u {{ return {}u; }}",
                        from, state, occurrences, to
                    ),
                    None => format!(
                        "    if state == {}u && count_living(adj) == {}u {{ return {}u; }}",
                        from, occurrences, to
                    )
                }
                
            } );
        }
    }

    compute.push("    return 0u;".to_string());
    compute.push("}}".to_string());

    compute.join("\n").into()
}
