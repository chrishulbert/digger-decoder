use crate::file_finder;
use crate::decompressor;
use crate::special;
use crate::image;
use std::collections::HashMap;
use anyhow::Result;

pub fn load(path: &str) -> Result<HashMap::<u32, image::Image>> {
    let mut all = HashMap::<u32, image::Image>::new();
    for file in file_finder::find(path, "vgaspec", ".dat")? {
        let sections = decompressor::decompress(&file.data);
        let Some(section) = sections.first() else { continue };
        let special = special::parse(&section)?;
        all.insert(file.number, special);
    }
    Ok(all)
}
