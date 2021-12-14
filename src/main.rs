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
const MENU_COLOR: &dyn Color = &White;
const MENU_STR: &'static str =
    "╔══════════════════════════════════════╗\n\r\
     ║                                      ║\n\r\
     ║                                      ║\n\r\
     ║                                      ║\n\r\
     ║                                      ║\n\r\
     ║  Tetris CLI by Dylan Turner ca 2021  ║\n\r\
     ║                                      ║\n\r\
     ║                                      ║\n\r\
     ║              Controls:               ║\n\r\
     ║      - a/d -> move left/right        ║\n\r\
     ║     - q/e -> rotate left/right       ║\n\r\
     ║          - s -> drop piece           ║\n\r\
     ║      - backspace -> quit game        ║\n\r\
     ║                                      ║\n\r\
     ║                                      ║\n\r\
     ║        Press enter to begin...       ║\n\r\
     ║                                      ║\n\r\
     ║                                      ║\n\r\
     ║                                      ║\n\r\
     ║                                      ║\n\r\
     ║                                      ║\n\r\
     ║                                      ║\n\r\
     ╚══════════════════════════════════════╝\n\r";

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
        cnv.write(&String::from(MENU_STR), (1, 1), &MENU_COLOR, &Reset);
        cnv.flush();

        let key = inp.get_key();
        match key {
            b'\n' | b'\r' => play_game(&mut cnv, &mut inp), // Enter
            127 => break, // Backspace
            _ => {}
        }
    }
}
