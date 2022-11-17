use std::{
    collections,
    ops::{Index, IndexMut, self}
};

use rand::{
    Rng,
    seq::SliceRandom
};

use winit::dpi;
use cgmath::Point2;

use crate::ColorScheme;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[allow(clippy::from_over_into)]
impl Into<dpi::Size> for Size {
    fn into(self) -> dpi::Size {
        let (width, height) = (self.width, self.height);
        dpi::Size::Physical(dpi::PhysicalSize { width, height } )
    }
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
        Self { data: vec![0; (size.width * size.height) as usize], size }
    }
}

pub fn rand_automata(size: Size, states: &[(ops::Range<u32>, f32)]) -> Automata {
    let mut prng = rand::thread_rng();

    Automata {
        data: { 
            (0..(size.width * size.height))
                .map(|_| { 
                    let choice = states.choose_weighted(&mut prng, |s| s.1 );
                    prng.gen_range(choice.unwrap().0.clone()) 
                } )
                .collect::<Vec<_>>() 
        },
        size
    }
}

pub fn load_automata_from_image(image: image::DynamicImage) -> (Automata, ColorScheme) {
    let image = image.to_rgb8();

    let size = Size { width: image.width(), height: image.height() };
    let mut automata = Automata::new(size);

    let mut states = collections::HashMap::new();
    for x in 0..automata.size.width {
        for y in 0..automata.size.height {
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

    fn to_color(pixel: image::Rgb<u8>) -> [f32; 3] {
        let p = pixel.0;
        [p[0] as f32 / 255f32, p[1] as f32 / 255f32, p[2] as f32 / 255f32]
    }

    let states = states
        .into_iter()
        .map(|(pixel, curr_state)| (curr_state, to_color(pixel)))
        .collect::<Vec<_>>();
    
    (automata, ColorScheme::Map(states))
}