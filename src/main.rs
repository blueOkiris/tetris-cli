/*
 * Author: Dylan Turner
 * Description: Main entry point to tetris game program
 */

mod settings;

use termion::raw::{ IntoRawMode, RawTerminal };
use termion::input::TermRead;
use termion::event::Key;
use termion::{ clear, cursor };

use std::io::{ stdin, stdout, Write, Stdout };
use std::sync::mpsc::channel;
use std::thread;
use std::time::{ Instant, Duration };

use log::info;
use log4rs;

enum Room {
    Menu
}

fn draw_bg_and_walls(output : &mut RawTerminal<Stdout>) {
    write!(output, "{}{}", cursor::Goto(1, 1), settings::TITLE_STR).unwrap();
}

fn clear_screen(output : &mut RawTerminal<Stdout>) {
    write!(output, "{}", clear::All);
}

fn main() {
    log4rs::init_file(settings::LOG_CONFIG_FILE, Default::default()).unwrap();
    info!("Started logging tetris-cli!");

    // Set up io
    let input = stdin();
    let mut output = stdout().into_raw_mode().unwrap();
    let (tx, rx) = channel();
    thread::spawn(move || {
        // Read from keyboard
        for c in input.keys() {
            tx.send(c).unwrap(); // Send down channel
        }
    });
    
    // Main loop
    let room = Room::Menu;
    let mut quit = false;
    while !quit {
        // Get the time
        let start = Instant::now();

        // Handle inputs as they come:
        for c in rx.try_iter() {
            match c.unwrap() {
                Key::Backspace => quit = true,
                _ => {}
            }
        }

        // Update screen
        clear_screen(&mut output);
        draw_bg_and_walls(&mut output);
        match room {
            Room::Menu => {
            }
        }

        // Delay if needed
        let elapsed = start.elapsed().subsec_nanos();
        if elapsed < settings::FPS_DELAY {
            let sleep_time = settings::FPS_DELAY - elapsed;
            thread::sleep(Duration::new(0, sleep_time));
        }
    }
}
