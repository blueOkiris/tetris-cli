/*
 * Author: Dylan Turner
 * Description: Main entry point to tetris game program
 */

mod settings;
mod io;

use termion::color;
use termion::event::Key;
use std::time::{ Instant, Duration };
use std::thread;

enum Room {
    Menu
}

fn draw_walls(disp : &mut io::Display) {
    let mut wall_str = String::new();
    for _y in 0..settings::SCREEN_HEIGHT - 1 {
        wall_str += "█";
        for _x in 1..settings::SCREEN_WIDTH - 1 {
            wall_str += " ";
        }
        wall_str += "█\r\n";
    }
    for _x in 0..settings::SCREEN_WIDTH {
        wall_str += "█";
    }
    wall_str += "\r\n";
    
    disp.set_fg(color::Blue);
    disp.goto(1, 1);
    disp.write(wall_str.as_str());
}

fn main() {
    io::init_logging(settings::LOG_CONFIG_FILE);

    // Set up io
    let mut disp = io::Display::new();
    let inp = io::Input::new();
    
    // Main loop
    let room = Room::Menu;
    let mut quit = false;
    while !quit {
        // Get the time
        let start = Instant::now();

        // Handle inputs as they come:
        for key in inp.read_available() {
            match key {
                Key::Backspace => quit = true,
                _ => {}
            }
        }

        // Update screen
        disp.clear();
        disp.goto(1, 1);
        draw_walls(&mut disp);
        disp.set_fg(color::Reset);
        disp.set_bg(color::Reset);
        
        // Draw and update based on state
        match room {
            Room::Menu => {
                // Draw title
                let title_str = String::from(settings::TITLE_STR);
                let title_str_len = title_str.len() as u16;
                let title_str_pos =
                    settings::SCREEN_WIDTH / 2
                        - title_str_len / 2;
                disp.goto(title_str_pos + 1, 4);
                disp.write(settings::TITLE_STR);
            }
        }
        
        disp.goto(1, settings::SCREEN_HEIGHT);

        // Delay if needed
        let elapsed = start.elapsed().subsec_nanos();
        if elapsed < settings::FPS_DELAY {
            let sleep_time = settings::FPS_DELAY - elapsed;
            thread::sleep(Duration::new(0, sleep_time));
        }
    }
}
