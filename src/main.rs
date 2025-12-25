mod bit_iter_dat;
mod bit_iter_ms_first;
mod decompressor;
mod ground;
mod image;
mod png;
mod level;

use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    let ground_raw = fs::read("data/lemmings/ground0o.dat")?;
    let ground = ground::Ground::parse(&ground_raw);

    let vgagr_raw = fs::read("data/lemmings/vgagr0.dat")?;
    let vgagr = decompressor::decompress(&vgagr_raw);

    let mut terrains: Vec<image::Image> = Vec::new();
    for (i, terrain) in ground.terrain_info.iter().enumerate() {
        if terrain.width == 0 || terrain.height == 0 { continue }

        let image = image::Image::parse_4bpp_plus_mask(&vgagr[0], terrain.width, terrain.height, terrain.image_loc, terrain.mask_loc, &ground.palette);
        // let image: image::Image = if terrain.width > 0 && terrain.height > 0 {
        // } else {
        //     image::Image::empty()
        // };
        let png = image.as_png();
        terrains.push(image);

        let path = format!("output_terrain{}.png", i);
        std::fs::write(path, &png)?;
    }

    let mut objects: Vec<image::Animation> = Vec::new();
    for (i, object) in ground.object_info.iter().enumerate() {
        if object.width == 0 || object.height == 0 || object.frame_count == 0 { continue }

        let animation = image::Animation::parse_4bpp_plus_mask(&vgagr[1], object.width, object.height, object.animation_frames_base_loc as usize, object.animation_frames_base_loc as usize + object.mask_offset_from_image as usize, &ground.palette, object.animation_frame_data_size as usize, object.frame_count as usize);
        let png = animation.as_apng();
        objects.push(animation);

        let path = format!("output_object{}.animation.png", i);
        std::fs::write(path, &png)?;
    }

    // TODO return GroundCombined which combines groundNo and vgagrN.

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

    let levels = load_all_levels("data/lemmings")?;
    for l in &levels {
        println!("Level: {}", l.name);
    }
    println!("Levels: {}", levels.len());

    Ok(())
}

fn load_all_levels(dir: &str) -> Result<Vec<level::Level>> {
    let mut all: Vec<level::Level> = Vec::new();
    for entry in fs::read_dir(dir)? {
        if let Ok(entry) = entry {
            let raw_name = entry.file_name().into_string().unwrap();
            let file_name = raw_name.to_lowercase();
            if (file_name.starts_with("level") || file_name.starts_with("dlvel")) && file_name.ends_with(".dat") {
                let file_number: i32 = file_name[5..8].parse().unwrap();
                let filename = format!("{}/{}", dir, raw_name);
                let raw: Vec<u8> = fs::read(&filename)?;
                // println!("Decompressing: {}", filename);
                let sections = decompressor::decompress(&raw);
                for (section_index, section) in sections.iter().enumerate() {
                    let level = level::parse(section)?;
                    all.push(level);
                }
            }
        }
    }
    Ok(all)
}