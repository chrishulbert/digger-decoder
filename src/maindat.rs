// This is for decoding the contents of main.dat
// https://www.camanis.net/lemmings/files/docs/lemmings_main_dat_file_format.txt

use anyhow::{Result, bail};
use crate::bit_iter_ms_first;
use crate::image::{Image, Animation, Mask};
use crate::decompressor;

const SKILL_PANEL_WIDTH: usize = 320;
const SKILL_PANEL_HEIGHT: usize = 40;

pub struct MainDat {
    pub lemming_animations: LemmingAnimations,
    pub masks: Masks,
    pub countdown_numbers: [Image; 10],
    pub skill_panel_high_perf: Image,
    #[allow(dead_code)]
    pub skill_number_digits: SkillNumberDigits,
    pub game_font_high_perf: GameFont,
    pub main_menu: MainMenu,
    #[allow(dead_code)]
    pub skill_panel: Image,
    #[allow(dead_code)]
    pub game_font: GameFont,
}

pub struct MainMenu {
    pub background: Image,
    pub logo: Image,
    pub f1: Image,
    pub f2: Image,
    pub f3: Image,
    pub f4: Image,
    pub level_rating: Image,
    pub exit_to_dos: Image,
    pub music_note: Image,
    pub fx: Image,

    pub blink1: Animation,
    pub blink2: Animation,
    pub blink3: Animation,
    pub blink4: Animation,
    pub blink5: Animation,
    pub blink6: Animation,
    pub blink7: Animation,
    pub left_scroller: Animation,
    pub right_scroller: Animation,
    pub reel: Image,
    // TODO change this to skill indexes so it's not lemmings-1-specific.
    // TODO change to support 5 skills for oh-no-more.
    pub mayhem: Image,
    pub taxing: Image,
    pub tricky: Image,
    pub fun: Image,
    pub menu_font: Animation, // 16x16, 94 frames, '!'(33) - '~'(126), in ascii order. Not really an animation, but this makes texture atlas conversion simpler. 
}

#[derive(Default)]
pub struct GameFont {
    pub percent: Image,
    pub digits: [Image; 10], // 0-9
    pub dash: Image,
    pub letters: [Image; 26], // A-Z
}

#[allow(dead_code)]
pub struct SkillNumberDigits {
    pub left: [Image; 10],
    pub right: [Image; 10],
}

pub struct LemmingAnimations {
    pub walking_right: Animation,
    pub jumping_right: Animation, // Walking up a step 3-6px tall. This is a 1-frame 'animation'.
    pub walking_left: Animation,
    pub jumping_left: Animation, // This is a 1-frame 'animation'.
    pub digging: Animation,
    pub climbing_right: Animation,
    pub climbing_left: Animation,
    pub drowning: Animation,
    pub post_climb_right: Animation,
    pub post_climb_left: Animation,
    pub brick_laying_right: Animation,
    pub brick_laying_left: Animation,
    pub bashing_right: Animation,
    pub bashing_left: Animation,
    pub mining_right: Animation,
    pub mining_left: Animation,
    pub falling_right: Animation,
    pub falling_left: Animation,
    pub pre_umbrella_right: Animation,
    pub umbrella_right: Animation,
    pub pre_umbrella_left: Animation,
    pub umbrella_left: Animation,
    pub splatting: Animation,
    pub exiting: Animation,
    pub fried: Animation,
    pub blocking: Animation,
    pub shrugging_right: Animation, // Builder running out of bricks.
    pub shrugging_left: Animation,
    pub oh_no_ing: Animation,
    pub explosion: Animation, // 1 frame.
}

pub struct Masks {
    pub bash_right: Mask,
    pub bash_left: Mask,
    pub mine_right: Mask,
    pub mine_left: Mask,
    pub explosion: Mask,
}

impl LemmingAnimations {
    fn parse(data: &[u8], palette: &[u32; 16]) -> Result<LemmingAnimations> {
        Ok(LemmingAnimations {
            walking_right: Animation::parse(&data[0x0000..], 8, 16, 10, palette, 2),
            jumping_right: Animation::parse(&data[0x0140..], 1, 16, 10, palette, 2),
            walking_left: Animation::parse(&data[0x0168..], 8, 16, 10, palette, 2),
            jumping_left: Animation::parse(&data[0x02A8..], 1, 16, 10, palette, 2),
            digging: Animation::parse(&data[0x02D0..], 16, 16, 14, palette, 3),
            climbing_right: Animation::parse(&data[0x0810..], 8, 16, 12, palette, 2),
            climbing_left: Animation::parse(&data[0x0990..], 8, 16, 12, palette, 2),
            drowning: Animation::parse(&data[0x0B10..], 16, 16, 10, palette, 2),
            post_climb_right: Animation::parse(&data[0x0D90..], 8, 16, 12, palette, 2),
            post_climb_left: Animation::parse(&data[0x0F10..], 8, 16, 12, palette, 2),
            brick_laying_right: Animation::parse(&data[0x1090..], 16, 16, 13, palette, 3),
            brick_laying_left: Animation::parse(&data[0x1570..], 16, 16, 13, palette, 3), 
            bashing_right: Animation::parse(&data[0x1A50..], 32, 16, 10, palette, 3), 
            bashing_left: Animation::parse(&data[0x21D0..], 32, 16, 10, palette, 3), 
            mining_right: Animation::parse(&data[0x2950..], 24, 16, 13, palette, 3), 
            mining_left: Animation::parse(&data[0x30A0..], 24, 16, 13, palette, 3), 
            falling_right: Animation::parse(&data[0x37F0..], 4, 16, 10, palette, 2), 
            falling_left: Animation::parse(&data[0x3890..], 4, 16, 10, palette, 2), 
            pre_umbrella_right: Animation::parse(&data[0x3930..], 4, 16, 16, palette, 3),
            umbrella_right: Animation::parse(&data[0x3AB0..], 4, 16, 16, palette, 3), 
            pre_umbrella_left: Animation::parse(&data[0x3C30..], 4, 16, 16, palette, 3), 
            umbrella_left: Animation::parse(&data[0x3DB0..], 4, 16, 16, palette, 3),
            splatting: Animation::parse(&data[0x3F30..], 16, 16, 10, palette, 2), 
            exiting: Animation::parse(&data[0x41B0..], 8, 16, 13, palette, 2), 
            fried: Animation::parse(&data[0x4350 ..], 14, 16, 14, palette, 4), 
            blocking: Animation::parse(&data[0x4970..], 16, 16, 10, palette, 2), 
            shrugging_right: Animation::parse(&data[0x4BF0..], 8, 16, 10, palette, 2), 
            shrugging_left: Animation::parse(&data[0x4D30..], 8, 16, 10, palette, 2), 
            oh_no_ing: Animation::parse(&data[0x4E70..], 16, 16, 10, palette, 2), 
            explosion: Animation::parse(&data[0x50F0..], 1, 32, 32, palette, 3),
        })
    }
}

impl Mask {
    fn parse(data: &[u8], width: usize, height: usize, frame_count: usize) -> Mask {
        let pixels = width * height;
        let mut frames: Vec<Vec<u8>> = Vec::with_capacity(frame_count);
        for frame_index in 0..frame_count {
            let offset_bits = frame_index * pixels;
            let mut plane = bit_iter_ms_first::iterate(data).skip(offset_bits);
            let mut bitmap: Vec<u8> = Vec::with_capacity(pixels);
            for _ in 0..pixels {
                let bit = plane.next().unwrap();
                bitmap.push(bit);
            }
            frames.push(bitmap);
        }
        return Mask { frames: frames, width: width, height: height };
    }
}

impl Masks {
    fn parse(data: &[u8]) -> Masks {
        Masks {
            bash_right: Mask::parse(&data[0x0000..], 16, 10, 4),
            bash_left:  Mask::parse(&data[0x0050..], 16, 10, 4),
            mine_right: Mask::parse(&data[0x00a0..], 16, 13, 2),
            mine_left:  Mask::parse(&data[0x00d4..], 16, 13, 2),
            explosion:  Mask::parse(&data[0x0108..], 16, 22, 1),
        }
    }
}

fn parse_countdown_numbers(data: &[u8]) -> [Image; 10] {
    [
        Image::parse_1bpp(&data[0x017C..], 8, 8),
        Image::parse_1bpp(&data[0x0174..], 8, 8),
        Image::parse_1bpp(&data[0x016C..], 8, 8),
        Image::parse_1bpp(&data[0x0164..], 8, 8),
        Image::parse_1bpp(&data[0x015C..], 8, 8),
        Image::parse_1bpp(&data[0x0154..], 8, 8),
        Image::parse_1bpp(&data[0x014C..], 8, 8),
        Image::parse_1bpp(&data[0x0144..], 8, 8),
        Image::parse_1bpp(&data[0x013C..], 8, 8),
        Image::parse_1bpp(&data[0x0134..], 8, 8),
    ]
}

impl SkillNumberDigits {
    fn parse(data: &[u8]) -> SkillNumberDigits {
        SkillNumberDigits {
            left: [
                Image::parse_1bpp(&data[0x1908..], 8, 8),
                Image::parse_1bpp(&data[0x1918..], 8, 8),
                Image::parse_1bpp(&data[0x1928..], 8, 8),
                Image::parse_1bpp(&data[0x1938..], 8, 8),
                Image::parse_1bpp(&data[0x1948..], 8, 8),
                Image::parse_1bpp(&data[0x1958..], 8, 8),
                Image::parse_1bpp(&data[0x1968..], 8, 8),
                Image::parse_1bpp(&data[0x1978..], 8, 8),
                Image::parse_1bpp(&data[0x1988..], 8, 8),
                Image::parse_1bpp(&data[0x1998..], 8, 8),
            ],
            right: [
                Image::parse_1bpp(&data[0x1900..], 8, 8),
                Image::parse_1bpp(&data[0x1910..], 8, 8),
                Image::parse_1bpp(&data[0x1920..], 8, 8),
                Image::parse_1bpp(&data[0x1930..], 8, 8),
                Image::parse_1bpp(&data[0x1940..], 8, 8),
                Image::parse_1bpp(&data[0x1950..], 8, 8),
                Image::parse_1bpp(&data[0x1960..], 8, 8),
                Image::parse_1bpp(&data[0x1970..], 8, 8),
                Image::parse_1bpp(&data[0x1980..], 8, 8),
                Image::parse_1bpp(&data[0x1990..], 8, 8),
            ]
        }
    }
}

impl GameFont {
    fn parse(data: &[u8], palette: &[u32; 16]) -> GameFont {
        const SIZE_PER_CHAR: usize = 0x30;
        let mut font = GameFont::default();
        let mut offset: usize = 0;
        font.percent = Image::parse_3bpp(&data[offset..], 8, 16, palette);
        offset += SIZE_PER_CHAR;
        for i in 0..10 {
            font.digits[i] = Image::parse_3bpp(&data[offset..], 8, 16, palette);
            offset += SIZE_PER_CHAR;
        }
        font.dash = Image::parse_3bpp(&data[offset..], 8, 16, palette);
        offset += SIZE_PER_CHAR;
        for i in 0..26 {
            font.letters[i] = Image::parse_3bpp(&data[offset..], 8, 16, palette);
            offset += SIZE_PER_CHAR;
        }
        return font;
    }
}

impl MainMenu {
    fn parse(section_3: &[u8], section_4: &[u8], palette: &[u32; 16]) -> MainMenu {
        let mut back_palette = palette.clone(); // Make 0 solid black, not transparent, for the background.
        back_palette[0] = 0xff000000;
        MainMenu {
            background:     Image::parse_2bpp(&section_3, 320, 104, &back_palette),
            logo:           Image::parse_4bpp(&section_3[0x2080..], 632, 94, palette),
            f1:             Image::parse_4bpp(&section_3[0x9488..], 120, 61, palette),
            f2:             Image::parse_4bpp(&section_3[0xa2d4..], 120, 61, palette),
            f3:             Image::parse_4bpp(&section_3[0xb120..], 120, 61, palette),
            f4:             Image::parse_4bpp(&section_3[0xdc04..], 120, 61, palette),
            level_rating:   Image::parse_4bpp(&section_3[0xbf6c..], 120, 61, palette),
            exit_to_dos:    Image::parse_4bpp(&section_3[0xCDB8..], 120, 61, palette),
            music_note:     Image::parse_4bpp(&section_3[0xEA50..], 64, 31, palette),
            fx:             Image::parse_4bpp(&section_3[0xEE30..], 64, 31, palette),
            blink1:         Animation::parse(&section_4[0x0000..], 8, 32, 12, palette, 4),
            blink2:         Animation::parse(&section_4[0x0600..], 8, 32, 12, palette, 4),
            blink3:         Animation::parse(&section_4[0x0C00..], 8, 32, 12, palette, 4),
            blink4:         Animation::parse(&section_4[0x1200..], 8, 32, 12, palette, 4),
            blink5:         Animation::parse(&section_4[0x1800..], 8, 32, 12, palette, 4),
            blink6:         Animation::parse(&section_4[0x1E00..], 8, 32, 12, palette, 4),
            blink7:         Animation::parse(&section_4[0x2400..], 8, 32, 12, palette, 4),
            left_scroller:  Animation::parse(&section_4[0x2A00..], 16, 48, 16, palette, 4),
            right_scroller: Animation::parse(&section_4[0x4200..], 16, 48, 16, palette, 4),
            reel:           Image::parse_4bpp(&section_4[0x5A00..], 16, 16, palette),
            mayhem:         Image::parse_4bpp(&section_4[0x5A80..], 72, 27, &back_palette),
            taxing:         Image::parse_4bpp(&section_4[0x5E4C..], 72, 27, &back_palette),
            tricky:         Image::parse_4bpp(&section_4[0x6218..], 72, 27, &back_palette),
            fun:            Image::parse_4bpp(&section_4[0x65E4..], 72, 27, &back_palette),
            menu_font:      Animation::parse(&section_4[0x69B0..], 94, 16, 16, palette, 3),
        }
    }
}

macro_rules! rgba_from_rgb { ($r:expr, $g:expr, $b:expr) => {
    (($r as u32) << 24) + (($g as u32) << 16) + (($b as u32) << 8) + 0xff
}}

impl MainDat {
    fn parse(sections: &Vec<Vec<u8>>) -> Result<MainDat> {
        if sections.len() < 7 {
            bail!("Not enough sections");
        }

        let menu_palette: [u32; 16] = [
            0, // Transparent black.
            rgba_from_rgb!(128, 64, 32), // Browns 
            rgba_from_rgb!( 96, 48, 32), // 
            rgba_from_rgb!( 48,  0, 16), //
            rgba_from_rgb!( 32,  8,124), // Purples 
            rgba_from_rgb!( 64, 44,144), //
            rgba_from_rgb!(104, 88,164), // 
            rgba_from_rgb!(152,140,188), // 
            rgba_from_rgb!(  0, 80,  0), // Greens
            rgba_from_rgb!(  0, 96, 16), //
            rgba_from_rgb!(  0,112, 32), //
            rgba_from_rgb!(  0,128, 64), //
            rgba_from_rgb!(208,208,208), // White 
            rgba_from_rgb!(176,176,  0), // Yellow 
            rgba_from_rgb!( 64, 80,176), // Blue 
            rgba_from_rgb!(224,128,144), // Pink  
        ];
        let mut game_palette = menu_palette;
        game_palette[1] = rgba_from_rgb!( 64, 64,224); // Blue
        game_palette[2] = rgba_from_rgb!(  0,176,  0); // Green
        game_palette[3] = rgba_from_rgb!(240,208,208); // White
        game_palette[4] = rgba_from_rgb!(176,176,  0); // Yellow
        game_palette[5] = rgba_from_rgb!(240, 32, 32); // Red
        game_palette[6] = rgba_from_rgb!(128,128,128); // Grey

        Ok(MainDat {
            lemming_animations: LemmingAnimations::parse(&sections[0], &game_palette)?,
            masks: Masks::parse(&sections[1]),
            countdown_numbers: parse_countdown_numbers(&sections[1]),
            skill_panel_high_perf: Image::parse_4bpp(&sections[2], SKILL_PANEL_WIDTH, SKILL_PANEL_HEIGHT, &game_palette),
            skill_number_digits: SkillNumberDigits::parse(&sections[2]),
            game_font_high_perf: GameFont::parse(&sections[2][0x19a0..], &game_palette),
            main_menu: MainMenu::parse(&sections[3], &sections[4], &menu_palette),
            skill_panel: Image::parse_4bpp(&sections[6], SKILL_PANEL_WIDTH, SKILL_PANEL_HEIGHT, &game_palette),
            game_font: GameFont::parse(&sections[6][0x1900..], &game_palette),
        })
    }

    pub fn load(dir: &str) -> Result<MainDat> {
        let file: Vec<u8> = std::fs::read(format!("{}/main.dat", dir))?;
        let sections = decompressor::decompress(&file);
        Self::parse(&sections)
    }
}
