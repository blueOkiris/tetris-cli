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

fn menu_state_draw(disp : &mut io::Display) {
    // Draw title and play info
    let mut line = 5;
    for msg in settings::MENU_STRS {
        // Get horizontal position
        let msg_str_len = msg.len() as u16;
        let msg_str_pos = settings::SCREEN_WIDTH / 2 - msg_str_len / 2;
        
        // Print it out
        disp.goto(msg_str_pos + 1, line);
        disp.write(msg);

        line += 1;
    }
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
            Room::Menu => menu_state_draw(&mut disp)
        }
        
        // Position cursor off screen
        disp.goto(settings::SCREEN_WIDTH + 1, settings::SCREEN_HEIGHT);

        // Delay if needed
        let elapsed = start.elapsed().subsec_nanos();
        if elapsed < settings::FPS_DELAY {
            let sleep_time = settings::FPS_DELAY - elapsed;
            thread::sleep(Duration::new(0, sleep_time));
        }
    }
}
