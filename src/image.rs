use crate::bit_iter_ms_first;
use crate::png;

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
    pub width: isize,
    pub height: isize,
}

impl Image {
    pub fn empty() -> Image {
        Image { bitmap: Vec::new(), width: 0, height: 0 }
    }

    /// Parses where 0=transparent, 1=white.
    fn parse_1bpp(data: &[u8], width: usize, height: usize) -> Image {
        let pixels = width * height;
        let mut plane = bit_iter_ms_first::iterate(data);
        let mut bitmap: Vec<u32> = Vec::with_capacity(pixels);
        for _ in 0..pixels {
            let bit = plane.next().unwrap();
            bitmap.push(if bit==0 { 0 } else { 0xffffffff } );
        }
        return Image { bitmap: bitmap, width: width, height: height };
    }

    fn parse_2bpp(data: &[u8], width: usize, height: usize, palette: [u32; 16]) -> Image {
        let pixels = width * height;
        let mut plane_0 = bit_iter_ms_first::iterate(data);
        let mut plane_1 = bit_iter_ms_first::iterate(data).skip(pixels);
        let mut bitmap: Vec<u32> = Vec::with_capacity(pixels);
        for _ in 0..pixels {
            let colour_index =
                plane_0.next().unwrap() +
                (plane_1.next().unwrap() << 1);
            let colour = palette[colour_index as usize];
            bitmap.push(colour);
        }
        return Image { bitmap: bitmap, width: width, height: height };
    }

    fn parse_3bpp(data: &[u8], width: usize, height: usize, palette: [u32; 16]) -> Image {
        let pixels = width * height;
        let mut plane_0 = bit_iter_ms_first::iterate(data);
        let mut plane_1 = bit_iter_ms_first::iterate(data).skip(pixels);
        let mut plane_2 = bit_iter_ms_first::iterate(data).skip(pixels * 2);
        let mut bitmap: Vec<u32> = Vec::with_capacity(pixels);
        for _ in 0..pixels {
            let colour_index =
                plane_0.next().unwrap() +
                (plane_1.next().unwrap() << 1) +
                (plane_2.next().unwrap() << 2);
            let colour = palette[colour_index as usize];
            bitmap.push(colour);
        }
        return Image { bitmap: bitmap, width: width, height: height };
    }

    fn parse_4bpp(data: &[u8], width: usize, height: usize, palette: [u32; 16]) -> Image {
        let pixels = width * height;
        let mut plane_0 = bit_iter_ms_first::iterate(data);
        let mut plane_1 = bit_iter_ms_first::iterate(data).skip(pixels);
        let mut plane_2 = bit_iter_ms_first::iterate(data).skip(pixels * 2);
        let mut plane_3 = bit_iter_ms_first::iterate(data).skip(pixels * 3);
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
        return Image { bitmap: bitmap, width: width, height: height };
    }

    pub fn parse_4bpp_plus_mask(data: &[u8], width: usize, height: usize, image_loc: usize, mask_loc: usize, palette: &[u32; 16]) -> Image {
        let bitmap = parse_4bpp_plus_mask_frame(data, width, height, image_loc, mask_loc, palette);
        return Image { bitmap: bitmap, width: width, height: height };
    }

    fn parse_8bpp(data: &[u8], width: usize, height: usize, palette: [u32; 16]) -> Image {
        let pixels = width * height;
        let mut bitmap = Vec::<u32>::with_capacity(pixels);
        for i in 0..pixels {
            bitmap.push(palette[data[i] as usize]);
        }
        return Image { bitmap: bitmap, width: width, height: height };
    }

    pub fn as_png(&self) -> Vec<u8> {
        png::png_data(self.width as u32, self.height as u32, &self.bitmap)
    }
}

impl Animation {
    pub fn parse_4bpp_plus_mask(data: &[u8], width: usize, height: usize, image_loc: usize, mask_loc: usize, palette: &[u32; 16], stride: usize, frame_count: usize) -> Animation {
        let mut frames: Vec<Vec<u32>> = Vec::new();
        for i in 0..frame_count {
            let offset = stride * i;
            let frame = parse_4bpp_plus_mask_frame(data, width, height, offset + image_loc, offset + mask_loc, palette);
            frames.push(frame);
        }
        Animation {
            frames: frames,
            width: width,
            height: height,
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