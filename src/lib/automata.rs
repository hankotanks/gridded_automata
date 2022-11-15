use std::{ops::{Index, IndexMut}, collections::HashMap};

use cgmath::Point2;
use image::{DynamicImage, GenericImageView};
use rand::Rng;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Size {
    pub height: u32,
    pub width: u32
}

pub struct Automata {
    pub(crate) data: Vec<u32>,
    pub(crate) size: Size
}

impl Index<Point2<u32>> for Automata {
    type Output = u32;

    fn index(&self, index: Point2<u32>) -> &Self::Output {
        &self.data[(index.x + index.y * self.size.width) as usize]
    }
}

impl IndexMut<Point2<u32>> for Automata {
    fn index_mut(&mut self, index: Point2<u32>) -> &mut Self::Output {
        &mut self.data[(index.x + index.y * self.size.width) as usize]
    }
}

impl Automata {
    pub fn new(size: Size) -> Self {
        Automata { data: vec![0; (size.width * size.height) as usize], size }
    }
}

pub fn rand_automata(size: Size) -> Automata {
    let mut prng = rand::thread_rng();

    Automata {
        data: { 
            (0..(size.height * size.width))
                .map(|_| prng.gen_range(0..=1) )
                .collect::<Vec<_>>() 
            },
        size
    }
}

pub fn read_automata_from_file(image: DynamicImage) -> (Automata, HashMap<u32, [f32; 3]>) {
    let size = image.dimensions();
    let size = Size { height: size.1, width: size.0 };

    let mut states: HashMap<image::Rgb<u8>, u32> = HashMap::new();

    let mut automata = Automata::new(size);

    let image = image.into_rgb8();

    for x in 0..size.width {
        for y in 0..size.height {
            let curr_state;
            match states.get(&image[(x, y)]) {
                Some(&state) => curr_state = state,
                None => { 
                    curr_state = states.len() as u32;
                    states.insert(image[(x, y)], curr_state);
                }
            }
            automata[Point2::new(x, y)] = curr_state;
        }
    }

    let mut dict = HashMap::new();
    for (pixel, state) in states.into_iter() {
        dict.insert(state, [
            pixel.0[0] as f32 / 255.0,
            pixel.0[1] as f32 / 255.0,
            pixel.0[2] as f32 / 255.0
        ]);
    }
    
    (automata, dict)
}