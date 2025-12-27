use crate::bit_iter_ms_first;
use crate::png;

#[derive(Default)]
pub struct Image {
    pub bitmap: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

pub struct Animation {
    pub frames: Vec<Vec<u32>>, // Think of this as an array of frames, where each frame is Vec<u32>.
    pub width: usize,
    pub height: usize,
}

pub struct Mask {
    pub frames: Vec<Vec<u8>>, // 1 means take a pixel out, 0 means leave alone.
    pub width: usize,
    pub height: usize,
}

impl Mask {
    pub fn as_apng(&self) -> Vec<u8> {
        let frames: Vec<Vec<u32>> = self.frames.iter().map(|frame|
            frame.iter().map(|pixel| {
                if *pixel == 0 { 0 } else {0x888888ff }
            }).collect()
        ).collect();
        let animation = Animation { frames, width: self.width, height: self.height };
        animation.as_apng()
    }
}

impl Image {
    /// Parses where 0=transparent, 1=white.
    pub fn parse_1bpp(data: &[u8], width: usize, height: usize) -> Image {
        let pixels = width * height;
        let mut plane = bit_iter_ms_first::iterate(data);
        let mut bitmap: Vec<u32> = Vec::with_capacity(pixels);
        for _ in 0..pixels {
            let bit = plane.next().unwrap();
            bitmap.push(if bit==0 { 0 } else { 0xffffffff } );
        }
        Image { bitmap, width, height }
    }

    pub fn parse_2bpp(data: &[u8], width: usize, height: usize, palette: &[u32; 16]) -> Image {
        let bitmap = parse_2bpp_frame(data, width, height, 0, &palette);
        Image { bitmap, width, height }
    }

    pub fn parse_3bpp(data: &[u8], width: usize, height: usize, palette: &[u32; 16]) -> Image {
        let bitmap = parse_3bpp_frame(data, width, height, 0, &palette);
        Image { bitmap, width, height }
    }

    pub fn parse_4bpp(data: &[u8], width: usize, height: usize, palette: &[u32; 16]) -> Image {
        let bitmap = parse_4bpp_frame(data, width, height, 0, &palette);
        Image { bitmap, width, height }
    }

    pub fn parse_4bpp_plus_mask(data: &[u8], width: usize, height: usize, image_loc: usize, mask_loc: usize, palette: &[u32; 16]) -> Image {
        let bitmap = parse_4bpp_plus_mask_frame(data, width, height, image_loc, mask_loc, palette);
        Image { bitmap, width, height }
    }

    pub fn as_png(&self) -> Vec<u8> {
        png::png_data(self.width as u32, self.height as u32, &self.bitmap)
    }
}

impl Animation {
    pub fn parse_4bpp_plus_mask(data: &[u8], width: usize, height: usize, frame_count: usize, image_loc: usize, mask_loc: usize, palette: &[u32; 16], stride: usize) -> Animation {
        let mut frames: Vec<Vec<u32>> = Vec::new();
        for i in 0..frame_count {
            let offset = stride * i;
            let frame = parse_4bpp_plus_mask_frame(data, width, height, offset + image_loc, offset + mask_loc, palette);
            frames.push(frame);
        }
        Animation { frames, width, height }
    }

    fn parse_4bpp(data: &[u8], width: usize, height: usize, frame_count: usize, palette: &[u32; 16]) -> Animation {
        let mut frames: Vec<Vec<u32>> = Vec::new();
        for i in 0..frame_count {
            let offset = i * width * height * 4;
            let frame = parse_4bpp_frame(data, width, height, offset, palette);
            frames.push(frame);
        }
        Animation { frames, width, height }
    }

    fn parse_3bpp(data: &[u8], width: usize, height: usize, frame_count: usize, palette: &[u32; 16]) -> Animation {
        let mut frames: Vec<Vec<u32>> = Vec::new();
        for i in 0..frame_count {
            let offset = i * width * height * 3;
            let frame = parse_3bpp_frame(data, width, height, offset, palette);
            frames.push(frame);
        }
        Animation { frames, width, height }
    }

    fn parse_2bpp(data: &[u8], width: usize, height: usize, frame_count: usize, palette: &[u32; 16]) -> Animation {
        let mut frames: Vec<Vec<u32>> = Vec::new();
        for i in 0..frame_count {
            let offset = i * width * height * 2;
            let frame = parse_2bpp_frame(data, width, height, offset, palette);
            frames.push(frame);
        }
        Animation { frames, width, height }
    }
    
    pub fn parse(data: &[u8], width: usize, height: usize, frames: usize, palette: &[u32; 16], bpp: u8) -> Animation {
        if bpp == 2 {
            Self::parse_2bpp(data, width, height, frames, palette)
        } else if bpp == 3 {
            Self::parse_3bpp(data, width, height, frames, palette)
        } else if bpp == 4 {
            Self::parse_4bpp(data, width, height, frames, palette)
        } else {
            panic!("Unsupported BPP")
        }
    }

    pub fn as_apng(&self) -> Vec<u8> {
        png::apng_data(self.width as u32, self.height as u32, &self.frames)
    }
}

// Helpers used for both images and animations:
pub fn parse_4bpp_plus_mask_frame(data: &[u8], width: usize, height: usize, image_loc: usize, mask_loc: usize, palette: &[u32; 16]) -> Vec<u32> {
    let image_data: &[u8] = &data[image_loc..];
    let mask_data: &[u8] = &data[mask_loc..];
    let pixels = width * height;
    let mut plane_0 = bit_iter_ms_first::iterate(image_data);
    let mut plane_1 = bit_iter_ms_first::iterate(image_data).skip(pixels);
    let mut plane_2 = bit_iter_ms_first::iterate(image_data).skip(pixels * 2);
    let mut plane_3 = bit_iter_ms_first::iterate(image_data).skip(pixels * 3);
    let mut mask_iter = bit_iter_ms_first::iterate(mask_data);
    let mut bitmap: Vec<u32> = Vec::with_capacity(pixels);
    for _ in 0..pixels {
        let colour_index =
            plane_0.next().unwrap() +
            (plane_1.next().unwrap() << 1) +
            (plane_2.next().unwrap() << 2) +
            (plane_3.next().unwrap() << 3);
        let colour = palette[colour_index as usize];
        let masked_colour: u32 = if mask_iter.next().unwrap() == 0 { 0 } else { colour };
        bitmap.push(masked_colour);
    }
    bitmap
}

// For images, offset_bits should be 0; for animations should be frame_index * pixels * BPP;
fn parse_4bpp_frame(data: &[u8], width: usize, height: usize, offset_bits: usize, palette: &[u32; 16]) -> Vec<u32> {
    let pixels = width * height;
    let mut plane_0 = bit_iter_ms_first::iterate(data).skip(offset_bits);
    let mut plane_1 = bit_iter_ms_first::iterate(data).skip(offset_bits + pixels);
    let mut plane_2 = bit_iter_ms_first::iterate(data).skip(offset_bits + pixels * 2);
    let mut plane_3 = bit_iter_ms_first::iterate(data).skip(offset_bits + pixels * 3);
    let mut bitmap: Vec<u32> = Vec::with_capacity(pixels);
    for _ in 0..pixels {
        let colour_index =
            plane_0.next().unwrap() +
            (plane_1.next().unwrap() << 1) +
            (plane_2.next().unwrap() << 2) +
            (plane_3.next().unwrap() << 3);
        let colour = palette[colour_index as usize];
        bitmap.push(colour);
    }
    bitmap
}

fn parse_3bpp_frame(data: &[u8], width: usize, height: usize, offset_bits: usize, palette: &[u32; 16]) -> Vec<u32> {
    let pixels = width * height;
    let mut plane_0 = bit_iter_ms_first::iterate(data).skip(offset_bits);
    let mut plane_1 = bit_iter_ms_first::iterate(data).skip(offset_bits + pixels);
    let mut plane_2 = bit_iter_ms_first::iterate(data).skip(offset_bits + pixels * 2);
    let mut bitmap: Vec<u32> = Vec::with_capacity(pixels);
    for _ in 0..pixels {
        let colour_index =
            plane_0.next().unwrap() +
            (plane_1.next().unwrap() << 1) +
            (plane_2.next().unwrap() << 2);
        let colour = palette[colour_index as usize];
        bitmap.push(colour);
    }
    bitmap
}

fn parse_2bpp_frame(data: &[u8], width: usize, height: usize, offset_bits: usize, palette: &[u32; 16]) -> Vec<u32> {
    let pixels = width * height;
    let mut plane_0 = bit_iter_ms_first::iterate(data).skip(offset_bits);
    let mut plane_1 = bit_iter_ms_first::iterate(data).skip(offset_bits + pixels);
    let mut bitmap: Vec<u32> = Vec::with_capacity(pixels);
    for _ in 0..pixels {
        let colour_index =
            plane_0.next().unwrap() +
            (plane_1.next().unwrap() << 1);
        let colour = palette[colour_index as usize];
        bitmap.push(colour);
    }
    bitmap
}
