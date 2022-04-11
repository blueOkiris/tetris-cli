/*
 * Author: Dylan Turner
 * Description: Main game loop
 */

use std::{
    thread::sleep,
    time::{
        Instant, Duration
    }
};
use termion::color::{
    Color, White, Reset
};
use math::round::floor;
use crate::io::{
    Canvas, KeyReader,
    DISP_HEIGHT, GRID_WIDTH, GRID_HEIGHT, SHAPE_WIDTH, SHAPE_STR
};
use crate::tetromino::{
    SHAPE_COLORS, Tetromino
};

pub const FPS: u64 = 60;
const BORDER: [&'static str; DISP_HEIGHT as usize] = [
    "H:                    ",
    "S:                    ",
    "╔════════════════════╗",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "║                    ║",
    "╚════════════════════╝"
];
const BORDER_COLOR: &dyn Color = &White;
const SCORE_COLOR: &dyn Color = &White;
const SHAPE_DRAW_OFFSET: i16 = 5;
const INITIAL_FALL_SPD: f32 = 0.9;
const LAND_TIME_DELAY_S: f64 = 0.1;

#[derive(PartialEq, Clone, Copy)]
pub enum Dir {
    Down,
    Left,
    Right
}

struct GameState<'a> {
    pub score: u64,
    pub curr_shape: Tetromino<'a>,
    pub fall_spd: f32,
    pub blocks: [[i8; GRID_WIDTH]; GRID_HEIGHT],
    pub land_timer: f64
}

impl<'a> GameState<'a> {
    pub fn new() -> Self {
        Self {
            score: 0,
            curr_shape: Tetromino::select(),
            fall_spd: INITIAL_FALL_SPD,
            blocks: [
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ],
                [ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ]
            ], land_timer: LAND_TIME_DELAY_S
        }
    }
}

enum UpdateEndState {
    Quit,
    Lost,
    Continue
}

// Main game loop function
pub fn play_game(cnv: &mut Canvas, inp: &mut KeyReader, hs_disp: &Vec<&String>) -> u64 {
    let mut state = GameState::new();

    let mut last_time = Instant::now();
    let interval_ms = 1_000 / FPS;
    loop {
        // Keep stable fps
        let now = Instant::now();
        let delta_time_ms = (now.duration_since(last_time).subsec_nanos() / 1_000_000) as u64;
        if delta_time_ms < interval_ms {
            sleep(Duration::from_millis(interval_ms - delta_time_ms));
            continue;
        }
        last_time = now;

        match update(inp, &mut state, delta_time_ms) {
            UpdateEndState::Continue => {},
            UpdateEndState::Quit => {
                return 0;
            }, UpdateEndState::Lost => {
                break;
            }
        }
        draw(cnv, hs_disp, &mut state);
    }

    state.score
}

fn update(inp: &mut KeyReader, state: &mut GameState, delta_time_ms: u64) -> UpdateEndState {
    let key = inp.get_key();
    match key {
        127 => return UpdateEndState::Quit, // Backspace -> back to menu
        b'a' => {
            if can_move(state, Dir::Left) {
                state.curr_shape.pos.0 -= 1.0;
            }
        }, b'd' => {
            if can_move(state, Dir::Right) {
                state.curr_shape.pos.0 += 1.0;
            }
        }, b'q' => {
            if can_rotate(state, Dir::Left) {
                state.curr_shape.rotate(Dir::Left);
            }
        }, b'e' => {
            if can_rotate(state, Dir::Right) {
                state.curr_shape.rotate(Dir::Right);
            }
        }, b's' => {
            state.curr_shape.pos.1 = floor(state.curr_shape.pos.1 as f64, 0) as f32;
            while can_move(state, Dir::Down) {
                state.curr_shape.pos.1 += 1.0;
            }
        }
        _ => {}
    }

    if can_move(state, Dir::Down) {
        state.curr_shape.pos.1 += state.fall_spd * (delta_time_ms as f32 / 1_000.0);
    } else if state.land_timer > 0.0 { // Allow a few ms for moving b4 settling
        state.land_timer -= delta_time_ms as f64 / 1_0000.0;
    } else if state.curr_shape.pos.1 <= 1.0 { // Landed at start means death
        return UpdateEndState::Lost;
    } else {
        state.score += 10 + (state.fall_spd * 50.0) as u64;

        // TODO: Store blocks
        // TODO: Delete completed rows, score, and increase fall speed

        state.land_timer = LAND_TIME_DELAY_S;
        state.curr_shape = Tetromino::select();
    }

    UpdateEndState::Continue
}

fn draw(cnv: &mut Canvas, hs_disp: &Vec<&String>, state: &mut GameState) {
    cnv.draw_strs(&BORDER.to_vec(), (1, 1), BORDER_COLOR, &Reset);

    let score_str = format!("{:020}", state.score);
    let score_disp = vec![ &score_str ];
    cnv.draw_strings(&hs_disp, (3, 1), SCORE_COLOR, &Reset);
    cnv.draw_strings(&score_disp, (3, 2), SCORE_COLOR, &Reset);
    
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if state.blocks[y][x] != -1 {
                cnv.draw_strs(
                    &vec![ SHAPE_STR ],
                    ((x * SHAPE_WIDTH + 2) as u16, (y + 2) as u16),
                    SHAPE_COLORS[state.blocks[y][x] as usize], &Reset
                );
            }
        }
    }

    // Dealing with whole display! Not just grid
    let (shape_x, shape_y) = state.curr_shape.pos;
    let shape_block_x = floor(shape_x as f64, 0) as i16;
    let shape_block_y = floor(shape_y as f64, 0) as i16;
    for coord in state.curr_shape.coords {
        let (mut coord_x, mut coord_y) = coord;
        coord_x += shape_block_x;
        coord_y += shape_block_y;

        let x = coord_x * 2 + 1;
        let y = coord_y + SHAPE_DRAW_OFFSET;

        cnv.draw_strs(&vec![ SHAPE_STR ], (x as u16, y as u16), state.curr_shape.fg, &Reset);
    }

    cnv.flush();
}

// Check if a block is able to move in a given direction
fn can_move(state: &mut GameState, dir: Dir) -> bool {
    // Get position in a grid format (we want slow movement, so we use actually use floats)
    let (shape_x, shape_y) = state.curr_shape.pos;
    let shape_block_x = floor(shape_x as f64, 0) as i16;
    let shape_block_y = floor(shape_y as f64, 0) as i16;

    for coord in state.curr_shape.coords {
        let (mut coord_x, mut coord_y) = coord;
        coord_x += shape_block_x;
        coord_y += shape_block_y;

        // Adjust the x/y in the direction we want to test
        match dir {
            Dir::Left => {
                coord_x -= 1;
            }, Dir::Right => {
                coord_x += 1;
            }, Dir::Down => {
                coord_y += 1;
            }
        }

        // Deal with just grid! Not whole display
        if coord_x < 1 || coord_x >= GRID_WIDTH as i16 || coord_y >= GRID_HEIGHT as i16 - 1 {
            return false;
        }

        if coord_y >= 0 && state.blocks[coord_y as usize][coord_x as usize] != -1 {
            return false;
        }
    }
    true
}

// Basically check if "can_move" to the current position after rotation
fn can_rotate(state: &mut GameState, dir: Dir) -> bool {
    if dir == Dir::Down {
        return true;
    }

    // Create temp shape and rotate it
    let mut temp_shape = state.curr_shape.clone();
    temp_shape.rotate(dir);

    // Check if it's valid
    let (shape_x, shape_y) = temp_shape.pos;
    let shape_block_x = floor(shape_x as f64, 0) as i16;
    let shape_block_y = floor(shape_y as f64, 0) as i16;
    for coord in temp_shape.coords {
        let (mut coord_x, mut coord_y) = coord;
        coord_x += shape_block_x;
        coord_y += shape_block_y;

        // NOTE: Unlike can_move, we don't want to adjust the coords

        // Deal with just grid! Not whole display
        if coord_x < 1 || coord_x >= GRID_WIDTH as i16 || coord_y >= GRID_HEIGHT as i16 - 1 {
            return false;
        }

        if coord_y >= 0 && state.blocks[coord_y as usize][coord_x as usize] != -1 {
            return false;
        }
    }
    true
}
