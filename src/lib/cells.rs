use std::ops::{Index, IndexMut};

use cgmath::Point2;
use rand::Rng;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Size {
    pub height: u32,
    pub width: u32
}

pub struct Cells {
    pub(crate) data: Vec<u32>,
    pub(crate) size: Size
}

impl Index<Point2<u32>> for Cells {
    type Output = u32;

    fn index(&self, index: Point2<u32>) -> &Self::Output {
        &self.data[(index.x + index.y * self.size.width) as usize]
    }
}

impl IndexMut<Point2<u32>> for Cells {
    fn index_mut(&mut self, index: Point2<u32>) -> &mut Self::Output {
        &mut self.data[(index.x + index.y * self.size.width) as usize]
    }
}

impl Cells {
    pub fn new(size: Size) -> Self {
        Cells { data: vec![0; (size.width * size.height) as usize], size }
    }
}

pub fn rand_cells(size: Size) -> Cells {
    let mut prng = rand::thread_rng();

    Cells {
        data: { 
            (0..(size.height * size.width))
                .map(|_| prng.gen_range(0..=1) )
                .collect::<Vec<_>>() 
            },
        size
    }
}