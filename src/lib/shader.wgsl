// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex: vec2<f32>,
};

struct Size {
    height: u32,
    width: u32
}

@group(0) @binding(0)
var<uniform> size: Size;

@group(1) @binding(0)
var output_texture: texture_2d<f32>;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex = model.tex;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let coord = vec2<i32>(
        i32(in.tex.x * f32(size.width)), 
        i32(in.tex.y * f32(size.height))
    );

    let color = textureLoad(output_texture, coord, 0);
    return color;
}