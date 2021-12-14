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
    "║                                      ║",
    "║                                      ║",
    "╚══════════════════════════════════════╝"
];
const BORDER_COLOR: &dyn Color = &White;

pub fn play_game(cnv: &mut Canvas, inp: &mut KeyReader) {
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

        // Draw
        cnv.draw(&BORDER.to_vec(), (1, 1), BORDER_COLOR, &Reset);
        // TODO: Draw blocks and stuff
        cnv.flush();
    }
}
