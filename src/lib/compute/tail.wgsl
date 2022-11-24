fn main_cs(@builtin(global_invocation_id) id: vec3<u32>) {
    if(id.x < size.width && id.y < size.height) {
        let index = id.x + id.y * size.width;
        let coord = vec2<i32>(i32(id.x), i32(id.y));

        updated[index] = main(neighborhood(coord), current[index]);
        textureStore(output_texture, coord, vec4<f32>(get_color(current[index]), 1.0));
    }
}