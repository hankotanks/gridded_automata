use std::{
    borrow,
    ops::{ Index, IndexMut }, 
    io, 
    fs,
    collections
};

use winit::dpi;
use cgmath::Point2;

#[cfg(feature = "from_rand")]
use rand::seq;

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

#[cfg(feature = "from_rand")]
pub fn random_automata(
    size: Size, 
    states: &[u32]
) -> Automata {
    random_automata_with_padding(size, states, 0)
}

#[cfg(feature = "from_rand")]
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

pub fn automata_from_pgm<C: Into<borrow::Cow<'static, str>>>(file: C) -> anyhow::Result<Automata> {
    #[derive(Debug)]
    enum PgmFormat { P2, P5 }

    let file: borrow::Cow<'static, str> = file.into();
    if file[file.len() - 4..] != *".pgm" { 
        anyhow::bail!(io::Error::from(io::ErrorKind::InvalidInput));
    }

    let image = io::BufReader::new(fs::File::open(&*file)?);

    let mut lines = io::BufRead::lines(image);

    let mut format: Option<PgmFormat> = None;
    'format: while format.is_none() {
        if let Some(result) = lines.next() {
            let line = result?;
            if line.starts_with('#') { continue 'format; }
            match line.split('#').next().unwrap() {
                "P2" => format = Some(PgmFormat::P2),
                "P5" => format= Some(PgmFormat::P5),
                _ => {  }
            }
        } else {
            anyhow::bail!(io::Error::from(io::ErrorKind::InvalidInput));
        }
    }

    if format.is_none() { anyhow::bail!(io::Error::from(io::ErrorKind::InvalidInput)); }
    let format = format.unwrap();

    let mut size: Option<Size> = None;
    'size: while size.is_none() {
        if let Some(result) = lines.next() {
            let line = result?;
            if line.starts_with('#') { continue 'size; }
            let line = line.split('#').next().unwrap();
            let line = line.split(' ').collect::<Vec<_>>();
            let width = line[0].to_string().parse::<u32>();
            let height = line[0].to_string().parse::<u32>();
            if width.is_err() || height.is_err() { 
                anyhow::bail!(io::Error::from(io::ErrorKind::InvalidInput));
            }

            size = Some((width.unwrap(), height.unwrap()).into());
        } else {
            anyhow::bail!(io::Error::from(io::ErrorKind::InvalidInput));
        }
    }

    if size.is_none() { anyhow::bail!(io::Error::from(io::ErrorKind::InvalidInput)); }
    let size = size.unwrap();

    'seek: loop {
        if let Some(result) = lines.next() {
            let line = result?;
            if line.starts_with('#') { continue 'seek; }
            let line = line.split('#').next().unwrap();
            if line.to_string().parse::<u32>().is_ok() {
                break 'seek;
            }
        } else {
            anyhow::bail!(io::Error::from(io::ErrorKind::InvalidInput));
        }
    }

    let mut automata = Automata::new(size);
    automata.data = match format {
        PgmFormat::P2 => { // ASCII
            let mut data = Vec::new();
            for line in lines {
                for word in line?.split(' ') {
                    if let Ok(state) =  word.parse::<u32>() {
                        data.push(state);
                    }
                }
            }

            data
        },
        PgmFormat::P5 => { // binary
            let cells = size.width * size.height;
            let mut data = Vec::new();
            'outer: for line in lines {
                '_inner: for state in line?.as_bytes() {
                    if data.len() as u32 >= cells { break 'outer; }
                    data.push(*state as u32);
                }
            }

            data
        }
    };

    anyhow::Ok(automata)
}

#[cfg(feature = "from_image")]
pub fn automata_from_image<C: Into<borrow::Cow<'static, str>>>(file: C) -> anyhow::Result<Automata> {
    let image = image::open(&*file.into())?.to_luma8();

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

    anyhow::Ok(automata)
}