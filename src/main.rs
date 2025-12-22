mod reverse_bit_iterator;
mod decompressor;
mod ground;

use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    let ground_raw = fs::read("data/lemmings/ground0o.dat")?;
    let ground = ground::Ground::parse(&ground_raw);

    let dat = fs::read("data/lemmings/vga.dat")?;
    let sections = decompressor::decompress(&dat);

    println!("Decompressed {} sections", sections.len());

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
