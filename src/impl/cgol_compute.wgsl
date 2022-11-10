struct Size {
    height: u32,
    width: u32
}

@group(0) @binding(0)
var<uniform> size: Size;

@group(0) @binding(1)
var<storage, read_write> automata: array<u32>;

fn to_coord(index: u32) -> vec2<i32> {
    return vec2<i32>(i32(index / size.width), i32(index % size.width));
}

// Coord must be wrapped
fn to_index(coord: vec2<i32>) -> u32 {
    return u32(coord.x) + u32(coord.y) * size.width;
}

fn wrap(coord: vec2<i32>) -> vec2<i32> {
    var n_c = vec2<i32>(coord.x % i32(size.width), coord.y % i32(size.height));
    
    if(n_c.x < 0) { n_c.x += i32(size.width); }
    if(n_c.y < 0) { n_c.y += i32(size.height); }

    return n_c;
}

fn living(adj: array<u32, 8>) -> u32 {
    var alive: u32 = 0u;
    for(var i = 0; i < 8; i++) {
        if(i == 0) { if(adj[0] == u32(1)) { alive++; } }
        if(i == 1) { if(adj[1] == u32(1)) { alive++; } }
        if(i == 2) { if(adj[2] == u32(1)) { alive++; } }
        if(i == 3) { if(adj[3] == u32(1)) { alive++; } }
        if(i == 4) { if(adj[4] == u32(1)) { alive++; } }
        if(i == 5) { if(adj[5] == u32(1)) { alive++; } }
        if(i == 6) { if(adj[6] == u32(1)) { alive++; } }
        if(i == 7) { if(adj[7] == u32(1)) { alive++; } }
    }

    return alive;
}

fn moore_neighborhood(coord: vec2<i32>) -> array<u32, 8> {
    var adj: array<u32, 8> = array<u32, 8>();

    adj[0] = automata[to_index(wrap(vec2<i32>(coord.x - 1, coord.y - 1)))];
    adj[1] = automata[to_index(wrap(vec2<i32>(coord.x, coord.y - 1)))];
    adj[2] = automata[to_index(wrap(vec2<i32>(coord.x + 1, coord.y - 1)))];
    adj[3] = automata[to_index(wrap(vec2<i32>(coord.x - 1, coord.y)))];
    adj[4] = automata[to_index(wrap(vec2<i32>(coord.x + 1, coord.y)))];
    adj[5] = automata[to_index(wrap(vec2<i32>(coord.x - 1, coord.y + 1)))];
    adj[6] = automata[to_index(wrap(vec2<i32>(coord.x, coord.y + 1)))];
    adj[7] = automata[to_index(wrap(vec2<i32>(coord.x + 1, coord.y + 1)))];

    return adj;
}

@compute @workgroup_size(64)
fn main_cs(
    @builtin(global_invocation_id) id: vec3<u32>
) {
    let state = automata[id.x];
    let adj_count = living(moore_neighborhood(to_coord(id.x)));

    if(state == 1u && (adj_count == 2u || adj_count == 3u)) {
        automata[id.x] = 1u;
    } else if(state == 0u && adj_count == 3u) {
        automata[id.x] = 1u;
    } else {
        automata[id.x] = 0u;
    }
}