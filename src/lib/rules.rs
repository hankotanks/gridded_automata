pub mod symmetry {
    pub struct Symmetry(u8);
    impl From<u8> for Symmetry {
        fn from(byte: u8) -> Self { Self(byte) }
    }

    pub const ASYMMETRIC: u8 = 0;
    pub const ROTATIONAL: u8 = 1 << 0;
    pub const PERMUTE: u8 = 1 << 1;
    pub const FLIP_WIDTH: u8 = 1 << 2;
    pub const FLIP_HEIGHT: u8 = 1 << 3;
    pub const REFLECT: u8 = FLIP_WIDTH | FLIP_HEIGHT;
}

use std::borrow;

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

pub struct RuleList {
    rules: Vec<(u32, u32, Rule)>,
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

    pub fn add_rule(&mut self, from: u32, to: u32, rule: Rule) {
        self.rules.push((from, to, rule));
    }
}

pub fn compile_compute_shader(rules: RuleList) -> borrow::Cow<'static, str> {
    let mut compute = Vec::new();
    compute.push("fn main(coord: vec2<i32>, state: u32) -> u32 {{".to_string());
    compute.push(match rules.neighborhood {
        Neighborhood::Moore => "    let adj = moore_neighborhood(coord);".to_string(),
        Neighborhood::VonNeumann => "    let adj = von_neumann_neighborhood(coord);".to_string()
    } );

    for (from, to, rule) in rules.rules.into_iter() {
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

    compute.push("    return 0u;".to_string());
    compute.push("}}".to_string());


    compute.join("\n").into()
}
/*
DECAY
if(state == {}) {{ return {}; }}

COUNT
if(state == {} && count_matching(neighborhood, {}) == {})

 */

/*
const WIRE: u32 = 1;
const HEAD: u32 = 2;
const TAIL: u32 = 3;
let mut a = Automata::new(4);
a.add_decomposition_rule(HEAD, TAIL);
a.add_decomposition_rule(TAIL, WIRE);
a.add_count_rule(WIRE, [2, 1..=2], HEAD, Neighborhood::Moore);

Automata::from_rules([
	Rule::Decomposition(HEAD, TAIL),
	Rule::Decomposition(TAIL, WIRE),
	Rule::Count { matching: HEAD, count: 1..=2, then: HEAD, neighborhood: Neighborhood::Moore }
]);

 */