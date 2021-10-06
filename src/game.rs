/*
 * Author: Dylan Turner
 * Description: Game code (i.e. non-Engine code)
 */

use crate::state;
use crate::io;
use crate::settings;
use crate::tetromino;

use rand::Rng;
use termion::event::Key;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Room {
    Menu, Game, GameOver
}

pub struct GameData {
    pub room : Room,
    pub quit : bool,
    fall_spd : f32,
    placement_ctr : u8,
    score : u32,
    last_shape : tetromino::Tetromino,
    cur_shape : tetromino::Tetromino
}

impl GameData {
    pub fn new() -> GameData {
        return GameData {
            room : Room::Menu,
            quit : false,
            fall_spd : INITIAL_FALL,
            placement_ctr : PLACEMENT_DELAY,
            score : 0,
            last_shape : select_shape(),
            cur_shape : select_shape()
        };
    }
}

pub const INITIAL_FALL : f32 = 0.01;
pub const FALL_INC : f32 = 0.001;
pub const PLACEMENT_DELAY : u8 = 50;
pub const SHAPE_DEF : (f32, f32) = (5.0, 3.0);

pub const GAME_STATES : [(Room, state::State); 3] = [
    (Room::Menu, state::State {
        draw : |data : &mut GameData, disp : &mut io::Display| {
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
                    Key::Char('\n') => {
                        // Start new game
                        data.quit = false;
                        data.cur_shape = select_shape();
                        data.last_shape = data.cur_shape;
                        data.fall_spd = INITIAL_FALL;
                        data.placement_ctr = PLACEMENT_DELAY;
                        data.score = 0;
                        data.room = Room::Game;
                    }
                    _ => {}
                }
            }
        }
    }), (Room::Game, state::State {
        draw : |data : &mut GameData, disp : &mut io::Display| {
            if data.score != 0 {
                // Clear old image

            }

            // Draw new image


            // Draw score


        }, update : |data : &mut GameData, keys : Vec<Key>| {
            // Move
            for key in keys {
                match key {
                    Key::Backspace => data.quit = true,
                    Key::Char('a') => {
                        // Move left
                        
                    }, Key::Char('d') => {
                        // Move right
                        
                    }, Key::Char('q') => {
                        // Rotate ccw
                        
                    }, Key::Char('e') => {
                        // Rotate cw
                        
                    }, Key::Char('s') => {
                        // Drop
                        
                    }, _ => {}
                }
            }

            // Fall

        }
    }), (Room::GameOver, state::State {
        draw : |data : &mut GameData, disp : &mut io::Display| {

        }, update : |data : &mut GameData, keys : Vec<Key>| {
            // Move
            for key in keys {
                match key {
                    Key::Backspace => data.quit = true,
                    Key::Char('\n') => data.room = Room::Menu,
                    _ => {}
                }
            }
        }
    })
];

pub fn select_shape() -> tetromino::Tetromino {
    let (def_x, def_y) = SHAPE_DEF;
    let shape_num = rand::thread_rng().gen_range(0..6);
    let shape_enum = match shape_num {
        0 => tetromino::ShapeType::T,
        1 => tetromino::ShapeType::L,
        2 => tetromino::ShapeType::MirrorL,
        3 => tetromino::ShapeType::S,
        4 => tetromino::ShapeType::Z,
        5 => tetromino::ShapeType::Square,
        6 => tetromino::ShapeType::Line,
        _ => tetromino::ShapeType::T
    };
    let coords_ls : HashMap<tetromino::ShapeType, [[i8; 2]; 4]> =
        tetromino::BASE_SHAPE_COORDS.iter().cloned().collect();
    let coords = match coords_ls.get(&shape_enum) {
        None => tetromino::BASE_SHAPE_COORDS[0].1,
        Some(shape_coords) => shape_coords.clone()
    };

    return tetromino::Tetromino {
        x : def_x,
        y : def_y,
        shape : shape_enum,
        coords : coords
    };
}
