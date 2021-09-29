/*
 * Author: Dylan Turner
 * Description: Storage of game settings
 */

pub const LOG_CONFIG_FILE : &'static str = "logging_config.yaml";

pub const TITLE_STR : &'static str = "Tetris CLI";

pub const SHAPE_WIDTH : isize = 2;
pub const SCREEN_WIDTH : u32 = 10 * (SHAPE_WIDTH as u32) + 2;
pub const SCREEN_HEIGHT : u32 = 20 + 1;

pub const FPS_DELAY : u32 = 1000000000 / 1; // 1fps delay in ns
