use gridded_automata::{
    run,
    cells
};

fn main() {
    let size = cells::Size {
        height: 128,
        width: 128
    };

    let automata = cells::rand_cells(size);

    let compute_shader_file = include_str!("cgol_compute.wgsl").into();
    
    pollster::block_on(run(
        automata,
        compute_shader_file,
        30
    ));
}