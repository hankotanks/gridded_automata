let START_COLOR: vec3<f32> = vec3<f32>(1.0, 0.2, 0.0);
let END_COLOR: vec3<f32> = vec3<f32>(0.0, 0.2, 1.0);
let TOTAL_STATES: u32 = 6u;

fn main(coord: vec2<i32>, state: u32) -> Cell {
    let neighborhood = moore_neighborhood(coord);
    var adj = count_living(neighborhood);

    var output: Cell;
    if((state != 0u && adj == 2u) || adj == 3u) {
        output.state = state;
        if(state < TOTAL_STATES) { output.state++; }
    } else if(state != 0u && adj != 2u && adj != 3u) {
        output.state = 0u;
    }

    output.color = color_lerp(START_COLOR, END_COLOR, TOTAL_STATES, output.state);

    return output;
}
