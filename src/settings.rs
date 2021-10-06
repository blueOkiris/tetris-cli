/*
 * Author: Dylan Turner
 * Description: Storage of game settings
 */

pub const LOG_CONFIG_FILE : &'static str = "logging_config.yaml";

pub const MENU_STRS : [&'static str; 10] = [
    "Tetris CLI",
    "",
    "",
    "a/d - move",
    "q/e - rotate",
    "s - drop",
    "backspace - quit",
    "",
    "",
    "Enter to start!"
];

pub const SHAPE_WIDTH : isize = 2;
pub const SCREEN_WIDTH : u16 = 10 * (SHAPE_WIDTH as u16) + 2;
pub const SCREEN_HEIGHT : u16 = 20 + 1;

pub const FPS_DELAY : u32 = 1000000000 / 15; // 15 fps delay in ns
