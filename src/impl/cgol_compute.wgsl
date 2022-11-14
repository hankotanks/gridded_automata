struct Size {
    height: u32,
    width: u32
}

@group(0) @binding(0)
var<uniform> size: Size;

@group(1) @binding(0)
var<storage, read> old: array<u32>;

@group(1) @binding(1)
var<storage, read_write> updated: array<u32>;

@group(1) @binding(2)
var output_texture: texture_storage_2d<r32float, write>;

fn wrap(coord: vec2<i32>) -> u32 {
    var n_c = vec2<i32>(coord.x % i32(size.width), coord.y % i32(size.height));
    
    if(n_c.x < 0) { n_c.x += i32(size.width); }
    if(n_c.y < 0) { n_c.y += i32(size.height); }

    return u32(n_c.x) + u32(n_c.y) * size.width;
}

fn adj_count(coord: vec2<i32>) -> u32 {
    var alive: u32 = 0u;
    if(old[wrap(vec2<i32>(coord.x - 1, coord.y - 1))] == 1u) { alive++; };
    if(old[wrap(vec2<i32>(coord.x, coord.y - 1))] == 1u) { alive++; };
    if(old[wrap(vec2<i32>(coord.x + 1, coord.y - 1))] == 1u) { alive++; };
    if(old[wrap(vec2<i32>(coord.x - 1, coord.y))] == 1u) { alive++; };
    if(old[wrap(vec2<i32>(coord.x + 1, coord.y))] == 1u) { alive++; };
    if(old[wrap(vec2<i32>(coord.x - 1, coord.y + 1))] == 1u) { alive++; };
    if(old[wrap(vec2<i32>(coord.x, coord.y + 1))] == 1u) { alive++; };
    if(old[wrap(vec2<i32>(coord.x + 1, coord.y + 1))] == 1u) { alive++; };

    return alive;
}

@compute @workgroup_size(16, 16, 1)
fn main_cs(
    @builtin(global_invocation_id) id: vec3<u32>
) {
    if(id.x < size.width && id.y < size.height) {
        let index = id.x + id.y * size.width;
        let coord = vec2<i32>(i32(id.x), i32(id.y));

        let state = old[index];
        let adj_count = adj_count(coord);

        var new_state = state;

        if(state == 1u && (adj_count == 2u || adj_count == 3u)) {
            new_state = 1u;
        } else if(state == 0u && adj_count == 3u) {
            new_state = 1u;
        } else {
            new_state = 0u;
        }

        updated[index] = new_state;
        textureStore(output_texture, coord, vec4<f32>(f32(new_state), 0.0, 0.0, 1.0));
    }
}