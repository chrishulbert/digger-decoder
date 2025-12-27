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
mod special;
mod specials_loader;
mod maindat;

use anyhow::Result;

fn main() -> Result<()> {
    // Main.dat

    let path = "data/lemmings";
    let grounds = grounds_loader::load(path)?;
    let levels = levels_loader::load(path)?;
    let specials = specials_loader::load(path)?;
    let maindat = maindat::MainDat::load(path)?;

    for (i, level) in levels.iter().enumerate() {
        let image = level_renderer::render(&level, &grounds, &specials);
        let png = image.as_png();
        let safe_name = file_safe_string(&level.name);
        let name = format!("output_level{}_{}.static.png", i, safe_name);
        std::fs::write(name, png)?;
    }

    Ok(())
}

fn file_safe_string(str: &str) -> String {
    let mut s = String::new();
    let mut was_nonprintable = false;
    for c in str.chars() {
        if c.is_ascii_alphanumeric() {
            if was_nonprintable {
                s.push(' ');
            }
            s.push(c);
            was_nonprintable = false;
        } else {
            was_nonprintable = true;
        }
    }
    s
}
