use crate::file_finder;
use crate::ground;
use crate::image;
use crate::decompressor;
use std::collections::HashMap;
use anyhow::Result;

pub fn load(path: &str) -> Result<HashMap<u32, GroundWithImages>> {
    let mut meta = load_metadata(path)?;
    let mut all = HashMap::<u32, GroundWithImages>::new();
    for file in file_finder::find(path, "vgagr", ".dat")? {
        let Some(ground) = meta.remove(&file.number) else { continue };
        let vgagr = decompressor::decompress(&file.data);

        // Load the terrain imagery.
        let mut terrain = HashMap::<usize, image::Image>::new();
        for (i, info) in ground.terrain_info.iter().enumerate() {
            if info.width == 0 || info.height == 0 { continue }
            let image = image::Image::parse_4bpp_plus_mask(&vgagr[0], info.width, info.height,
                info.image_loc, info.mask_loc, &ground.palette);
            terrain.insert(i, image);
        }
        
        // Load the objects.
        let mut objects = HashMap::<usize, image::Animation>::new();
        for (i, object) in ground.object_info.iter().enumerate() {
            if object.width == 0 || object.height == 0 || object.frame_count == 0 { continue }
            let animation = image::Animation::parse_4bpp_plus_mask(&vgagr[1], object.width, object.height,
                object.frame_count as usize,
                object.animation_frames_base_loc as usize,
                object.animation_frames_base_loc as usize + object.mask_offset_from_image as usize,
                &ground.palette, object.animation_frame_data_size as usize);
            objects.insert(i, animation);
        }

        all.insert(file.number, GroundWithImages { ground, terrain, objects });
    }
    Ok(all)
}

pub struct GroundWithImages {
    pub ground: ground::Ground, 
    pub terrain: HashMap<usize, image::Image>,
    pub objects: HashMap<usize, image::Animation>,
}

// Loads all grounds' metadata (graphic sets).
fn load_metadata(path: &str) -> Result<HashMap<u32, ground::Ground>> {
    let files = file_finder::find(path, "ground", ".dat")?;
    let mut all = HashMap::<u32, ground::Ground>::new();
    for file in files {
        let ground = ground::Ground::parse(&file.data);
        all.insert(file.number, ground);
    }
    Ok(all)
}
