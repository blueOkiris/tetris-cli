/*
 * Author: Dylan Turner
 * Description: Main entry point to tetris game program
 */

mod io;
mod game;
mod highscore;
mod tetromino;

use log4rs::init_file;
use log::info;
use std::{
    thread::sleep,
    time::{
        Instant, Duration
    }
};
use termion::{
    color::{
        Color, White, Reset
    }, terminal_size
};
use crate::io::{
    DISP_WIDTH, DISP_HEIGHT, Canvas, KeyReader, SHAPE_WIDTH
};
use crate::game::{
    FPS, play_game
};
use crate::highscore::SaveData;

const LOG_FILE: &'static str = "logging_config.yaml";
const MENU: [&'static str; DISP_HEIGHT as usize] = [
    "                      ",
    "╔════════════════════╗",
    "║  T E T R I S  CLI  ║",
    "║ Created by Dylan T ║",
    "║     circa 2022     ║",
    "║                    ║",
    "║                    ║",
    "║     Controls:      ║",
    "║ - a/d - left/right ║",
    "║   - q/e - rotate   ║",
    "║ - s -> drop piece  ║",
    "║   - back -> quit   ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║  Enter to begin... ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║     High Score:    ║",
    "║                    ║",
    "║                    ║",
    "╚════════════════════╝",
    "                      "
];
const MENU_COLOR: &dyn Color = &White;

fn main() {
    init_file(LOG_FILE, Default::default()).unwrap();
    info!("Started new game!");
    info!("{}", SHAPE_WIDTH);

    // Load high score from config file
    let save = SaveData::load_config();
    let mut high_score = save.assert_hs();

    // Check that terminal is big enough
    let (width, height) = terminal_size().unwrap();
    if width < DISP_WIDTH || height < DISP_HEIGHT {
        println!(
            "Cannot start game! Terminal window too small. Must be at least {}x{}",
            DISP_WIDTH, DISP_HEIGHT
        );
        return;
    }

    let mut cnv = Canvas::new();
    let mut inp = KeyReader::new();

    // Show the menu and controls before launching the game
    let mut last_time = Instant::now();
    let interval_ms = 1_000 / FPS;
    loop {
        // Keep stable fps
        let now = Instant::now();
        let delta_time_ms =
            (now.duration_since(last_time).subsec_nanos() / 1_000_000) as u64;
        if delta_time_ms < interval_ms {
            sleep(Duration::from_millis(interval_ms - delta_time_ms));
            continue;
        }
        last_time = now;

        let hs_str = format!("{:020}", high_score);
        let hs_disp = vec![ &hs_str ];

        cnv.draw_strs(&MENU.to_vec(), (1, 1), &MENU_COLOR, &Reset);
        cnv.draw_strings(&hs_disp, (2, 22), &MENU_COLOR, &Reset);
        cnv.flush();

        let key = inp.get_key();
        match key {
            b'\n' | b'\r' => { // Enter
                high_score = play_game(&mut cnv, &mut inp, &hs_disp);
                SaveData::save_value(high_score);
            }, 127 => break, // Backspace
            _ => {}
        }
    }

    cnv.reset();
}
