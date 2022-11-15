@compute @workgroup_size(16, 16, 1)
fn main_cs(@builtin(global_invocation_id) id: vec3<u32>) {
    if(id.x < size.width && id.y < size.height) {
        let index = id.x + id.y * size.width;

        let coord = vec2<i32>(i32(id.x), i32(id.y));
        let state = current[index];
        
        let cell: Cell = main(coord, state);
        updated[index] = cell.state;
        textureStore(output_texture, coord, vec4<f32>(cell.color, 1.0));
    }
}