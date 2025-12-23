mod bit_iter_dat;
mod bit_iter_ms_first;
mod decompressor;
mod ground;
mod image;
mod png;

use std::{fmt::format, fs};
use anyhow::Result;

fn main() -> Result<()> {
    let ground_raw = fs::read("data/lemmings/ground0o.dat")?;
    let ground = ground::Ground::parse(&ground_raw);

    let vgagr_raw = fs::read("data/lemmings/vgagr0.dat")?;
    let vgagr = decompressor::decompress(&vgagr_raw);

    let mut terrains: Vec<image::Image> = Vec::new();
    for (i, terrain) in ground.terrain_info.iter().enumerate() {
        let image = image::Image::parse_4bpp_plus_mask(&vgagr[0], terrain.width, terrain.height, terrain.image_loc, terrain.mask_loc, &ground.palette);
        // let image: image::Image = if terrain.width > 0 && terrain.height > 0 {
        // } else {
        //     image::Image::empty()
        // };
        let png = image.as_png();
        terrains.push(image);

        if terrain.width > 0 && terrain.height > 0 {
            let path = format!("out_terrain{}.png", i);
            std::fs::write(path, &png)?;
        }
    }

    // let mut object_sprites: AnimationMap = AnimationMap::new();
    // for (i, object) in ground.object_info.iter().enumerate() {
    //     if object.is_valid() {
    //         let sprite = image::Image::parse_4bpp_plus_mask(&vgagr[1], object.width, object.height, object.animation_frames_base_loc as usize, object.animation_frames_base_loc as usize + object.mask_offset_from_image as usize, &palette, object.animation_frame_data_size as usize, object.frame_count as usize);
    //         object_sprites.insert(i as i32, sprite);
    //     }
    // }

    // which are compressed include the following:
    // XgagrX.dat
    // levelXXX.dat
    // XgaspecX.dat
    // Main.dat
    // Cgamain.dat
    // groundXo.dat files are NOT compressed. 
    // The XgaspecX.dat has a second layer of compression underneath the .dat compression scheme (that is, once you
    // decompress an XgaspecX.dat file using the normal .DAT decompression algorithm,
    // you have to apply yet another decompression algorithm to get to the actual
    // data); I'll explain that in a separate document.

    Ok(())
}
