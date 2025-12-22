// This is for parsing lemmings GROUND files:
// https://www.camanis.net/lemmings/files/docs/lemmings_vgagrx_dat_groundxo_dat_file_format.txt

#[derive(Default, Debug, Clone)]
pub struct ObjectInfo {
    pub is_exit: bool, // According to lemmings_lvl_file_format.txt, the first object for any ground is always exit, second is entrance.
    pub is_entrance: bool, 
    pub animation_flags: u16, // bit 0 = 0 for loops, 1 for triggered animations.
    pub start_animation_frame_index: u8, 
    pub frame_count: u8, // aka end_animation_frame_index in the docs, but I suspect that's wrong, because if you +1 to get the frame count, it fails to load.
    pub width: usize,
    pub height: usize,
    pub animation_frame_data_size: u16,
    pub mask_offset_from_image: u16,
    pub trigger_left: u16,
    pub trigger_top: u16,
    pub trigger_width: u8,
    pub trigger_height: u8,
    pub trigger_effect_id: u8, // 0=none, 1=lemming exits, 4=trigger trap, 5=drown, 6=disintegrate, 7=one way wall left, 8=one way right, 9=steel
    pub animation_frames_base_loc: u16,
    pub preview_image_index: u16,
    pub trap_sound_effect_id: u8,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct TerrainInfo {
    pub width: usize,
    pub height: usize,
    pub image_loc: u16,
    pub mask_loc: u16,
}

#[derive(Default, Clone)]
pub struct Palettes {
    pub ega: [u8; 8],
    pub ega_standard: [u8; 8],
    pub ega_preview: [u8; 8],
    pub vga: [u32; 8], // RGB Palette entries 8...15. Only 6 bits so 0x3f = 100%
    pub vga_standard: [u32; 8], // Doesn't seem to be used by the game.
    pub vga_preview: [u32; 8], // Always seems to match custom.
}

// Upgrades a 6-bit colour to 8, while still allowing 100% black and white.
fn colour_upgrade(six: u8) -> u8 {
    if six == 0 { 0 } else { (six << 2) + 3 }
}

impl Palettes {
    pub fn as_rgba(&self) -> [u32; 16] {
        fn rgba_from_docs(rgb: u32) -> u32 {
            let r6: u8 = (rgb >> 16) as u8;
            let g6: u8 = (rgb >> 8) as u8; // 'as u8' simply truncates the red bits.
            let b6: u8 = rgb as u8;
            let r8: u8 = colour_upgrade(r6);
            let g8: u8 = colour_upgrade(g6);
            let b8: u8 = colour_upgrade(b6);
            return ((r8 as u32) << 24) + ((g8 as u32) << 16) + ((b8 as u32) << 8) + 0xff;
        }
        [
            rgba_from_docs(0x000000), // black.
            rgba_from_docs(0x101038), // blue, used for the lemmings' bodies.
            rgba_from_docs(0x002C00), // green, used for hair.
            rgba_from_docs(0x3C3434), // white, used for skin.
            rgba_from_docs(0x2C2C00), // dirty yellow, used in the skill panel.
            rgba_from_docs(0x3C0808), // red, used in the nuke icon.
            rgba_from_docs(0x202020), // gray, used in the skill panel.
            self.vga[0], // Game duplicates custom[0] twice, oddly.
            self.vga[0],
            self.vga[1],
            self.vga[2],
            self.vga[3],
            self.vga[4],
            self.vga[5],
            self.vga[6],
            self.vga[7],
        ]
    }
}

pub struct Ground {
    pub object_info: [ObjectInfo; 16],
    pub terrain_info: [TerrainInfo; 64],
    pub palettes: Palettes,
}

impl Default for Ground {
    fn default() -> Ground {
        Ground {
            object_info: Default::default(),
            terrain_info: [Default::default(); 64], // Default only auto-derives up to 32 element arrays.
            palettes: Default::default(),
        }
    }
}

impl Ground {
    /// Parses a ground file.
    pub fn parse(data: &[u8]) -> Ground {
        assert!(data.len() == 1056);
        let mut ground = Ground::default();
        let mut data_iter = data.into_iter();
        for i in 0..16 {
            ground.object_info[i].animation_flags = read_u16(&mut data_iter);
            ground.object_info[i].start_animation_frame_index = *data_iter.next().unwrap();
            ground.object_info[i].frame_count = *data_iter.next().unwrap();
            ground.object_info[i].width = *data_iter.next().unwrap() as usize;
            ground.object_info[i].height = *data_iter.next().unwrap() as usize;
            ground.object_info[i].animation_frame_data_size = read_u16(&mut data_iter);
            ground.object_info[i].mask_offset_from_image = read_u16(&mut data_iter);
            let _unknown1 = read_u16(&mut data_iter);
            let _unknown2 = read_u16(&mut data_iter);
            ground.object_info[i].trigger_left = read_u16(&mut data_iter);
            ground.object_info[i].trigger_top = read_u16(&mut data_iter);
            ground.object_info[i].trigger_width = *data_iter.next().unwrap();
            ground.object_info[i].trigger_height = *data_iter.next().unwrap();
            ground.object_info[i].trigger_effect_id = *data_iter.next().unwrap();
            ground.object_info[i].animation_frames_base_loc = read_u16(&mut data_iter);
            ground.object_info[i].preview_image_index = read_u16(&mut data_iter);
            let _unknown3 = read_u16(&mut data_iter);
            ground.object_info[i].trap_sound_effect_id = *data_iter.next().unwrap();
        }
        ground.object_info[0].is_exit = true;
        ground.object_info[1].is_entrance = true;
        for i in 0..64 {
            ground.terrain_info[i].width = *data_iter.next().unwrap() as usize;
            ground.terrain_info[i].height = *data_iter.next().unwrap() as usize;
            ground.terrain_info[i].image_loc = read_u16(&mut data_iter);
            ground.terrain_info[i].mask_loc = read_u16(&mut data_iter);
            let _unknown = read_u16(&mut data_iter);
        }
        for i in 0..8 {
            ground.palettes.ega[i] = *data_iter.next().unwrap();
        }
        for i in 0..8 {
            ground.palettes.ega_standard[i] = *data_iter.next().unwrap();
        }
        for i in 0..8 {
            ground.palettes.ega_preview[i] = *data_iter.next().unwrap();
        }
        for i in 0..8 {
            ground.palettes.vga[i] = read_rgb(&mut data_iter);
        }
        for i in 0..8 {
            ground.palettes.vga_standard[i] = read_rgb(&mut data_iter);
        }
        for i in 0..8 {
            ground.palettes.vga_preview[i] = read_rgb(&mut data_iter);
        }
        ground
    }
}

// Unlike the .LVL file format, WORDs in groundXo.dat are stored little-endian (camanis.net).
fn read_u16<I>(data: &mut I) -> u16
where I: Iterator<Item = &u8> {
    let little = *data.next().unwrap();
    let big = *data.next().unwrap();
    return ((big as u16) << 8) + (little as u16);
}

// Read 3 RGB bytes, converting to 0-255 RGBA format
// Source file: (0x3F, 0x00, 0x00) gives you the brightest red you can get (camanis.net)
fn read_rgb<I>(data: &mut I) -> u32
where I: Iterator<Item = &u8> {
    let r6 = *data.next().unwrap();
    let g6 = *data.next().unwrap();
    let b6 = *data.next().unwrap();
    let r8: u8 = colour_upgrade(r6);
    let g8: u8 = colour_upgrade(g6);
    let b8: u8 = colour_upgrade(b6);
    return ((r8 as u32) << 24) + ((g8 as u32) << 16) + ((b8 as u32) << 8) + 0xff;
}
