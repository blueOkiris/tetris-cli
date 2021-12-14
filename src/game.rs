/*
 * Author: Dylan Turner
 * Description: Main game loop
 */

use std::{
    thread::sleep,
    time::{ Instant, Duration }
};
use termion::color::{ Color, White, Reset };
use crate::io::{ Canvas, KeyReader, DISP_HEIGHT };

const FPS: u64 = 60;
const BORDER: [&'static str; DISP_HEIGHT as usize] = [
    "╔══════════════════════════════════════╗",
    "║   Your Score:                        ║",
    "║   High Score:                        ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "╚══════════════════════════════════════╝"
];
const BORDER_COLOR: &dyn Color = &White;
const SCORE_COLOR: &dyn Color = &White;

pub fn play_game(
        cnv: &mut Canvas, inp: &mut KeyReader, hs_disp: &Vec<&String>) -> u64 {
    let mut score: u64 = 0;
    
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

        // Handle input
        let key = inp.get_key();
        match key {
            127 => break, // Backspace -> back to menu
            _ => {}
        }

        // TODO: update game state
        score = 0;

        // Draw
        let score_str = format!("{:020}", score);
        let score_disp = vec![ &score_str ];
        cnv.draw_strs(&BORDER.to_vec(), (1, 1), BORDER_COLOR, &Reset);
        cnv.draw_strings(&hs_disp, (17, 3), SCORE_COLOR, &Reset);
        cnv.draw_strings(&score_disp, (17, 2), SCORE_COLOR, &Reset);
        // TODO: Draw blocks and stuff
        cnv.flush();
    }

    score
}
