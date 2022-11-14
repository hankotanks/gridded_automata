#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 2]
}

impl Vertex {
    pub fn description<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
        }
    }
}

pub(crate) const CLIP_SPACE_EXTREMA: [Vertex; 4] = [
    Vertex { pos: [-1.0, -1.0] },
    Vertex { pos: [ 1.0, -1.0] },
    Vertex { pos: [-1.0,  1.0] },
    Vertex { pos: [ 1.0,  1.0] },
];