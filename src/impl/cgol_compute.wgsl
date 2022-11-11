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

fn wrap(coord: vec2<i32>) -> u32 {
    var n_c = vec2<i32>(coord.x % i32(size.width), coord.y % i32(size.height));
    
    if(n_c.x < 0) { n_c.x += i32(size.width); }
    if(n_c.y < 0) { n_c.y += i32(size.height); }

    return u32(n_c.x) + u32(n_c.y) * size.width;
}

fn adj_count(id: vec3<u32>) -> u32 {
    var adj: array<u32, 8> = array<u32, 8>();

    //let coord = vec2<i32>(i32(id.x), i32(id.y));
    let coord = vec2<i32>(i32(id.x % size.width), i32(id.x / size.width));

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

@compute @workgroup_size(64)
fn main_cs(
    @builtin(global_invocation_id) id: vec3<u32>
) {
    let index = id.x;
    let state = old[index];
    let adj_count = adj_count(id);

    if(state == 1u && (adj_count == 2u || adj_count == 3u)) {
        updated[id.x] = 1u;
    } else if(state == 0u && adj_count == 3u) {
        updated[id.x] = 1u;
    } else {
        updated[id.x] = 0u;
    }
}