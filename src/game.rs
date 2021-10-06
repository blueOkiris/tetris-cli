/*
 * Author: Dylan Turner
 * Description: Game code (i.e. non-Engine code)
 */

use crate::state;
use crate::io;
use crate::settings;

use termion::event::Key;

pub enum Room {
    Menu
}

pub struct GameData {
    pub room : Room,
    pub quit : bool
}

impl GameData {
    pub fn new() -> GameData {
        return GameData {
            room : Room::Menu,
            quit : false
        };
    }
}

pub const MENU_STATE : state::State = state::State {
    draw : |disp : &mut io::Display| {
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
    }, update : |data : &mut GameData, keys : Vec<Key>| {
        for key in keys {
            match key {
                Key::Backspace => data.quit = true,
                _ => {}
            }
        }
    }
};
