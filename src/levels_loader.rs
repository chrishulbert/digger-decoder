use crate::file_finder;
use crate::level;
use crate::decompressor;
use anyhow::Result;

pub fn load(path: &str) -> Result<Vec<level::Level>> {
    let mut all = Vec::<level::Level>::new();
    for file in file_finder::find_2(path, "level", Some("dlvel"), ".dat")? {
        let sections = decompressor::decompress(&file.data);
        for section in sections {
            let level = level::parse(&section)?;
            all.push(level);
        }
    }
    all.sort_by(|a, b| (&a.name).cmp(&b.name));
    Ok(all)
}
