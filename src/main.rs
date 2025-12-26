mod bit_iter_dat;
mod bit_iter_ms_first;
mod decompressor;
mod ground;
mod grounds_loader;
mod image;
mod png;
mod level;
mod levels_loader;
mod file_finder;
mod level_renderer;

use anyhow::Result;

fn main() -> Result<()> {

    // which are compressed include the following:
    // XgaspecX.dat
    // Main.dat
    // The XgaspecX.dat has a second layer of compression underneath the .dat compression scheme (that is, once you
    // decompress an XgaspecX.dat file using the normal .DAT decompression algorithm,
    // you have to apply yet another decompression algorithm to get to the actual
    // data); I'll explain that in a separate document.

    let path = "data/lemmings";
    let grounds = grounds_loader::load(path)?;
    let levels = levels_loader::load(path)?;

    for (i, level) in levels.iter().enumerate() {
        let image = level_renderer::render(&level, &grounds);
        let png = image.as_png();
        let name = format!("output_level{}_{}.static.png", i, level.name);
        std::fs::write(name, png)?;
    }
    println!("Levels: {}", levels.len());

    Ok(())
}
