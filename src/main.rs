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
    println!("-=[ Digger Decoder ]=-");
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage:");
        println!("digger-decoder data/lemmings");
    } else {
        decode(&args[1])?;
    }
    Ok(())
}

fn decode(path: &str) -> Result<()> {
    println!("Loading main...");
    let maindat = maindat::MainDat::load(path)?;
    println!("Loading grounds...");
    let grounds = grounds_loader::load(path)?;
    println!("Loading specials...");
    let specials = specials_loader::load(path)?;
    println!("Loading levels...");
    let levels = levels_loader::load(path)?;

    println!("Exporting levels...");
    for (i, level) in levels.iter().enumerate() {
        let image = level_renderer::render(&level, &grounds, &specials);
        let png = image.as_png();
        let safe_name = file_safe_string(&level.name);
        let name = format!("output_level{}_{}.static.png", i, safe_name);
        std::fs::write(name, png)?;
    }

    println!("Exporting grounds...");
    for (gi, ground) in grounds {
        for (oi, o) in ground.objects {
            std::fs::write(
                format!("output_ground{}_object{}.animation.png", gi, oi),
                o.as_apng())?;
        }
        for (ti, t) in ground.terrain {
            std::fs::write(
                format!("output_ground{}_terrain{}.static.png", gi, ti),
                t.as_png())?;
        }
    }

    println!("Exporting main...");
    std::fs::write("output_main_lemming_walking_right.animation.png", maindat.lemming_animations.walking_right.as_apng())?;
    std::fs::write("output_main_lemming_jumping_right.animation.png", maindat.lemming_animations.jumping_right.as_apng())?;
    std::fs::write("output_main_lemming_walking_left.animation.png", maindat.lemming_animations.walking_left.as_apng())?;
    std::fs::write("output_main_lemming_jumping_left.animation.png", maindat.lemming_animations.jumping_left.as_apng())?;
    std::fs::write("output_main_lemming_digging.animation.png", maindat.lemming_animations.digging.as_apng())?;
    std::fs::write("output_main_lemming_climbing_right.animation.png", maindat.lemming_animations.climbing_right.as_apng())?;
    std::fs::write("output_main_lemming_climbing_left.animation.png", maindat.lemming_animations.climbing_left.as_apng())?;
    std::fs::write("output_main_lemming_drowning.animation.png", maindat.lemming_animations.drowning.as_apng())?;
    std::fs::write("output_main_lemming_post_climb_right.animation.png", maindat.lemming_animations.post_climb_right.as_apng())?;
    std::fs::write("output_main_lemming_post_climb_left.animation.png", maindat.lemming_animations.post_climb_left.as_apng())?;
    std::fs::write("output_main_lemming_brick_laying_right.animation.png", maindat.lemming_animations.brick_laying_right.as_apng())?;
    std::fs::write("output_main_lemming_brick_laying_left.animation.png", maindat.lemming_animations.brick_laying_left.as_apng())?;
    std::fs::write("output_main_lemming_bashing_right.animation.png", maindat.lemming_animations.bashing_right.as_apng())?;
    std::fs::write("output_main_lemming_bashing_left.animation.png", maindat.lemming_animations.bashing_left.as_apng())?;
    std::fs::write("output_main_lemming_mining_right.animation.png", maindat.lemming_animations.mining_right.as_apng())?;
    std::fs::write("output_main_lemming_mining_left.animation.png", maindat.lemming_animations.mining_left.as_apng())?;
    std::fs::write("output_main_lemming_falling_right.animation.png", maindat.lemming_animations.falling_right.as_apng())?;
    std::fs::write("output_main_lemming_falling_left.animation.png", maindat.lemming_animations.falling_left.as_apng())?;
    std::fs::write("output_main_lemming_pre_umbrella_right.animation.png", maindat.lemming_animations.pre_umbrella_right.as_apng())?;
    std::fs::write("output_main_lemming_umbrella_right.animation.png", maindat.lemming_animations.umbrella_right.as_apng())?;
    std::fs::write("output_main_lemming_pre_umbrella_left.animation.png", maindat.lemming_animations.pre_umbrella_left.as_apng())?;
    std::fs::write("output_main_lemming_umbrella_left.animation.png", maindat.lemming_animations.umbrella_left.as_apng())?;
    std::fs::write("output_main_lemming_splatting.animation.png", maindat.lemming_animations.splatting.as_apng())?;
    std::fs::write("output_main_lemming_exiting.animation.png", maindat.lemming_animations.exiting.as_apng())?;
    std::fs::write("output_main_lemming_fried.animation.png", maindat.lemming_animations.fried.as_apng())?;
    std::fs::write("output_main_lemming_blocking.animation.png", maindat.lemming_animations.blocking.as_apng())?;
    std::fs::write("output_main_lemming_shrugging_right.animation.png", maindat.lemming_animations.shrugging_right.as_apng())?;
    std::fs::write("output_main_lemming_shrugging_left.animation.png", maindat.lemming_animations.shrugging_left.as_apng())?;
    std::fs::write("output_main_lemming_oh_no_ing.animation.png", maindat.lemming_animations.oh_no_ing.as_apng())?;
    std::fs::write("output_main_lemming_explosion.animation.png", maindat.lemming_animations.explosion.as_apng())?;

    std::fs::write("output_main_mask_bash_right.animation.png", maindat.masks.bash_right.as_apng())?;
    std::fs::write("output_main_mask_bash_left.animation.png", maindat.masks.bash_left.as_apng())?;
    std::fs::write("output_main_mask_mine_right.animation.png", maindat.masks.mine_right.as_apng())?;
    std::fs::write("output_main_mask_mine_left.animation.png", maindat.masks.mine_left.as_apng())?;
    std::fs::write("output_main_mask_explosion.animation.png", maindat.masks.explosion.as_apng())?;

    for (i, image) in maindat.countdown_numbers.iter().enumerate() {
        std::fs::write(
            format!("output_main_countdown{}.static.png", i),
            image.as_png())?;
    }

    std::fs::write("output_main_font_percent.static.png", maindat.game_font.percent.as_png())?;
    std::fs::write("output_main_font_dash.static.png", maindat.game_font.dash.as_png())?;
    for (i, im) in maindat.game_font.digits.iter().enumerate() {
        std::fs::write(
            format!("output_main_font_digit{}.static.png", i),
            im.as_png())?;
    }
    for (i, im) in maindat.game_font.letters.iter().enumerate() {
        std::fs::write(
            format!("output_main_font_letter{}.static.png", i),
            im.as_png())?;
    }

    std::fs::write("output_main_skill_panel_high.static.png", maindat.skill_panel_high_perf.as_png())?;
    std::fs::write("output_main_skill_panel.static.png", maindat.skill_panel.as_png())?;

    std::fs::write("output_main_menu_background.static.png", maindat.main_menu.background.as_png())?;
    std::fs::write("output_main_menu_logo.static.png", maindat.main_menu.logo.as_png())?;
    std::fs::write("output_main_menu_f1.static.png", maindat.main_menu.f1.as_png())?;
    std::fs::write("output_main_menu_f2.static.png", maindat.main_menu.f2.as_png())?;
    std::fs::write("output_main_menu_f3.static.png", maindat.main_menu.f3.as_png())?;
    std::fs::write("output_main_menu_f4.static.png", maindat.main_menu.f4.as_png())?;
    std::fs::write("output_main_menu_level_rating.static.png", maindat.main_menu.level_rating.as_png())?;
    std::fs::write("output_main_menu_exit_to_dos.static.png", maindat.main_menu.exit_to_dos.as_png())?;
    std::fs::write("output_main_menu_music_note.static.png", maindat.main_menu.music_note.as_png())?;
    std::fs::write("output_main_menu_fx.static.png", maindat.main_menu.fx.as_png())?;
    std::fs::write("output_main_menu_reel.static.png", maindat.main_menu.reel.as_png())?;
    std::fs::write("output_main_menu_mayhem.static.png", maindat.main_menu.mayhem.as_png())?;
    std::fs::write("output_main_menu_taxing.static.png", maindat.main_menu.taxing.as_png())?;
    std::fs::write("output_main_menu_tricky.static.png", maindat.main_menu.tricky.as_png())?;
    std::fs::write("output_main_menu_fun.static.png", maindat.main_menu.fun.as_png())?;

    std::fs::write("output_main_menu_blink1.animation.png", maindat.main_menu.blink1.as_apng())?;
    std::fs::write("output_main_menu_blink2.animation.png", maindat.main_menu.blink2.as_apng())?;
    std::fs::write("output_main_menu_blink3.animation.png", maindat.main_menu.blink3.as_apng())?;
    std::fs::write("output_main_menu_blink4.animation.png", maindat.main_menu.blink4.as_apng())?;
    std::fs::write("output_main_menu_blink5.animation.png", maindat.main_menu.blink5.as_apng())?;
    std::fs::write("output_main_menu_blink6.animation.png", maindat.main_menu.blink6.as_apng())?;
    std::fs::write("output_main_menu_blink7.animation.png", maindat.main_menu.blink7.as_apng())?;
    std::fs::write("output_main_menu_left_scroller.animation.png", maindat.main_menu.left_scroller.as_apng())?;
    std::fs::write("output_main_menu_right_scroller.animation.png", maindat.main_menu.right_scroller.as_apng())?;
    std::fs::write("output_main_menu_menu_font.animation.png", maindat.main_menu.menu_font.as_apng())?;

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
