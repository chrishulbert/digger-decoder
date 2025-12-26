mod bit_iter_dat;
mod bit_iter_ms_first;
mod decompressor;
mod ground;
mod grounds_loader;
mod image;
mod png;
mod level;
mod file_finder;

use anyhow::Result;

fn main() -> Result<()> {
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

    let path = "data/lemmings";
    let grounds = grounds_loader::load(path)?;

    // let levels = load_all_levels(path)?;
    // for l in &levels {
    //     println!("Level: {}", l.name);
    // }
    // println!("Levels: {}", levels.len());

    Ok(())
}

fn load_all_levels(dir: &str) -> Result<Vec<level::Level>> {
    let mut all: Vec<level::Level> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let Ok(entry) = entry else { continue };
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
    Ok(all)
}
