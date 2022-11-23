@compute @workgroup_size(2, 2, 1)
fn main_cs(@builtin(global_invocation_id) id: vec3<u32>) {
    if(id.x < size.width && id.y < size.height) {
        let index = id.x + id.y * size.width;
        let coord = vec2<i32>(i32(id.x), i32(id.y));

        let state: u32 = main(coord, current[index]);
        let color = vec4<f32>(get_color(current[index]), 1.0);

        updated[index] = state;
        textureStore(output_texture, coord, color);
    }
}