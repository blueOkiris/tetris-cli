/*
 * Author: Dylan Turner
 * Description: Code for main game loop, I/O setup etc
 */

use crate::io;
use crate::game;
use crate::state;
use crate::settings;

use termion::color;
use std::time::{ Instant, Duration };
use std::thread;
use std::collections::HashMap;

pub fn run(
        inp : io::Input, disp : &mut io::Display, data : &mut game::GameData) {
    while !data.quit {
        // Get the time
        let start = Instant::now();

        reset_screen(disp);
        
        // Draw and update based on state
        let state_map : HashMap<game::Room, state::State> =
            game::GAME_STATES.iter().cloned().collect();
        match state_map.get(&data.room) {
            None => { },
            Some(state) => {
                (state.draw)(disp);
                (state.update)(data, inp.read_available());
            }
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

fn reset_screen(disp : &mut io::Display) {
    disp.clear();
    disp.goto(1, 1);
    draw_walls(disp);
    disp.set_fg(color::Reset);
    disp.set_bg(color::Reset);
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
