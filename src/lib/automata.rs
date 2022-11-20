use std::{
    borrow,
    collections,
    ops::{ Index, IndexMut },
};

use winit::dpi;
use rand::seq;
use cgmath::Point2;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

// Helpful conversions from...
impl From<(u32, u32)> for Size {
    fn from(data: (u32, u32)) -> Self {
        Self { width: data.0, height: data.1 }
    }
}

// ...and to image dimensions
impl From<Size> for (u32, u32) {
    fn from(data: Size) -> Self {
        (data.width, data.height)
    }
}

// Similar conversions from the rendering engine's unit size...
impl From<dpi::Size> for Size {
    fn from(data: dpi::Size) -> Self {
        let ds = data.to_physical(1.0);
        let (width, height) = (ds.width, ds.height);

        Self { width, height }
    }
}

// ...and to it
impl From<Size> for dpi::Size {
    fn from(data: Size) -> Self {
        let (width, height) = (data.width, data.height);
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

pub fn random_automata(
    size: Size, 
    states: &[u32]
) -> Automata {
    random_automata_with_padding(size, states, 0)
}

pub fn random_automata_with_padding(
    size: Size, 
    states: &[u32],
    padding: u32
) -> Automata {
    let mut prng = rand::thread_rng();

    let mut automata = Automata::new(size);
    for x in padding..(size.width - padding) {
        for y in padding..(size.height - padding) {
            automata[(x, y).into()] = *seq::SliceRandom::choose(states, &mut prng).unwrap();
        }
    }
    
    automata
}

pub fn automata_from_pgm(file: borrow::Cow<'static, str>) -> Automata {
    let image = image::open(&*file)
        .unwrap()
        .to_luma8();

    // Create the new automata object
    let mut automata = Automata::new(image.dimensions().into());

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

            automata[(x, y).into()] = curr_state;
        }
    }

    automata
}