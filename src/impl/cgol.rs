use gridded_automata::{
    run,
    automata
};

fn main() {
    let size = automata::Size {
        height: 512,
        width: 512
    };

    let automata = automata::rand_automata(size);

    let compute_shader_file = include_str!("cgol_compute.wgsl").into();
    
    pollster::block_on(run(
        automata,
        compute_shader_file,
        60
    ));
}