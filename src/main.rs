/*
 * Author: Dylan Turner
 * Description: Main entry point to tetris game program
 */

mod io;
mod game;

use log4rs::init_file;
use log::info;
use termion::{
    color::{ Color, White, Reset },
    terminal_size
};
use crate::io::{ DISP_WIDTH, DISP_HEIGHT, Canvas, KeyReader };
use crate::game::play_game;

const LOG_FILE: &'static str = "logging_config.yaml";
const MENU: [&'static str; DISP_HEIGHT as usize] = [
    "╔══════════════════════════════════════╗",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║  Tetris CLI by Dylan Turner ca 2021  ║",
    "║                                      ║",
    "║                                      ║",
    "║              Controls:               ║",
    "║      - a/d -> move left/right        ║",
    "║     - q/e -> rotate left/right       ║",
    "║          - s -> drop piece           ║",
    "║      - backspace -> quit game        ║",
    "║                                      ║",
    "║                                      ║",
    "║        Press enter to begin...       ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "╚══════════════════════════════════════╝"
];
const MENU_COLOR: &dyn Color = &White;

fn main() {
    init_file(LOG_FILE, Default::default()).unwrap();
    info!("Started new game!");

    // Check that terminal is big enough
    let (width, height) = terminal_size().unwrap();
    if width < DISP_WIDTH || height < DISP_HEIGHT {
        println!(
            "Cannot start game! Terminal window too small. Must be {}x{}",
            DISP_WIDTH, DISP_HEIGHT
        );
        return;
    }

    let mut cnv = Canvas::new();
    let mut inp = KeyReader::new();

    // Show the menu and controls before launching the game
    loop {
        cnv.draw(&MENU.to_vec(), (1, 1), &MENU_COLOR, &Reset);
        cnv.flush();

        let key = inp.get_key();
        match key {
            b'\n' | b'\r' => play_game(&mut cnv, &mut inp), // Enter
            127 => break, // Backspace
            _ => {}
        }
    }
}
