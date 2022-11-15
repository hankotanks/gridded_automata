//
// Declare some useful structs
//

struct Size { 
    height: u32, 
    width: u32 
}

struct Neighborhood { 
    cells: array<u32, 8> 
}

struct Cell {
    state: u32,
    color: vec3<f32>
}

//
// Read uniforms and storages
//

@group(0) @binding(0)
var<uniform> size: Size;

@group(1) @binding(0)
var<storage, read> current: array<u32>;

@group(1) @binding(1)
var<storage, read_write> updated: array<u32>;

@group(2) @binding(0)
var output_texture: texture_storage_2d<rgba32float, write>;

//
// Helper methods
//

fn wrap(coord: vec2<i32>) -> u32 {
    var n_c = vec2<i32>(coord.x % i32(size.width), coord.y % i32(size.height));
    
    if(n_c.x < 0) { n_c.x += i32(size.width); }
    if(n_c.y < 0) { n_c.y += i32(size.height); }

    return u32(n_c.x) + u32(n_c.y) * size.width;
}

//
// User-accessible methods
//

fn moore_neighborhood(coord: vec2<i32>) -> Neighborhood {
    var neighborhood: Neighborhood;
    neighborhood.cells = array<u32, 8>();
    
    neighborhood.cells[0] = current[wrap(vec2<i32>(coord.x - 1, coord.y - 1))];
    neighborhood.cells[1] = current[wrap(vec2<i32>(coord.x, coord.y - 1))];
    neighborhood.cells[2] = current[wrap(vec2<i32>(coord.x + 1, coord.y - 1))];
    neighborhood.cells[3] = current[wrap(vec2<i32>(coord.x - 1, coord.y))];
    neighborhood.cells[4] = current[wrap(vec2<i32>(coord.x + 1, coord.y))];
    neighborhood.cells[5] = current[wrap(vec2<i32>(coord.x - 1, coord.y + 1))];
    neighborhood.cells[6] = current[wrap(vec2<i32>(coord.x, coord.y + 1))];
    neighborhood.cells[7] = current[wrap(vec2<i32>(coord.x + 1, coord.y + 1))];

    return neighborhood;
}

fn von_neumann_neighborhood(coord: vec2<i32>) -> Neighborhood {
    var neighborhood: Neighborhood;
    neighborhood.cells = array<u32, 8>();

    neighborhood.cells[0] = current[wrap(vec2<i32>(coord.x, coord.y - 1))];
    neighborhood.cells[1] = current[wrap(vec2<i32>(coord.x - 1, coord.y))];
    neighborhood.cells[2] = current[wrap(vec2<i32>(coord.x + 1, coord.y))];
    neighborhood.cells[3] = current[wrap(vec2<i32>(coord.x, coord.y + 1))];

    return neighborhood;
}

fn count_living(neighborhood: Neighborhood) -> u32 {
    var neighbor_count = 0u;
    
    if(neighborhood.cells[0] != 0u) { neighbor_count++; }
    if(neighborhood.cells[1] != 0u) { neighbor_count++; }
    if(neighborhood.cells[2] != 0u) { neighbor_count++; }
    if(neighborhood.cells[3] != 0u) { neighbor_count++; }
    if(neighborhood.cells[4] != 0u) { neighbor_count++; }
    if(neighborhood.cells[5] != 0u) { neighbor_count++; }
    if(neighborhood.cells[6] != 0u) { neighbor_count++; }
    if(neighborhood.cells[7] != 0u) { neighbor_count++; }

    return neighbor_count;
}

fn color_mono(l_color: vec3<f32>, c_s: u32) -> vec3<f32> {
    if(c_s != 0u) {
        return l_color;
    } else {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
}

fn color_lerp(start: vec3<f32>, end: vec3<f32>, max_s: u32, c_s: u32) -> vec3<f32> {
    if(c_s == 0u) { return vec3<f32>(0.0, 0.0, 0.0); }

    let s = f32(c_s) / f32(max_s);

    return mix(start, end, vec3<f32>(s, s, s));
}