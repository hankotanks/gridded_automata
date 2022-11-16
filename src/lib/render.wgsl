// Vertex shader

struct VertexInput {
    @location(0) pos: vec2<f32>
};

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) tex: vec2<f32>,
};

struct Size {
    width: u32,
    height: u32
}

@group(0) @binding(0)
var<uniform> size: Size;

@group(1) @binding(0)
var output_texture: texture_2d<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_pos = vec4<f32>(in.pos, 0.0, 1.0);
    out.tex = (in.pos + 1.0) * 0.5;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let x: i32 = i32(in.tex.x * f32(size.width));
    let y: i32 = i32(in.tex.y * f32(size.height));

    return textureLoad(output_texture, vec2<i32>(x, y), 0);
}