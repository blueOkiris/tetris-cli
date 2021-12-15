/*
 * Author: Dylan Turner
 * Description: Main game loop
 */

use std::{
    thread::sleep,
    time::{ Instant, Duration }
};
use log::info;
use termion::color::{ Color, White, Reset };
use math::round::floor;
use crate::io::{
    Canvas, KeyReader,
    DISP_HEIGHT, GRID_WIDTH, GRID_HEIGHT, SHAPE_WIDTH, SHAPE_STR
};
use crate::tetromino::{ SHAPE_COLORS, Tetromino };

const FPS: u64 = 60;
const BORDER: [&'static str; DISP_HEIGHT as usize] = [
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
    "║                    ║",
    "║                    ║",
    "╚════════════════════╝"
];
const BORDER_COLOR: &dyn Color = &White;
const SCORE_COLOR: &dyn Color = &White;
const INITIAL_FALL_SPD: f32 = 0.01;
const FALL_KEY_SPD: f32 = 20.0;

enum Dir { Down, Left, Right }

struct GameState<'a> {
    pub score: u64,
    pub curr_shape: Tetromino<'a>,
    pub fall_spd: f32,
    pub blocks: [[i8; GRID_WIDTH]; GRID_HEIGHT]
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
            ]
        }
    }
}

// Main game loop function
pub fn play_game(
        cnv: &mut Canvas, inp: &mut KeyReader, hs_disp: &Vec<&String>) -> u64 {
    let mut state = GameState::new();
    
    let mut last_time = Instant::now();
    let interval_ms = 1_000 / FPS;
    loop {
        // Keep stable fps
        let now = Instant::now();
        let delta_time_ms =
            (now.duration_since(last_time).subsec_nanos() / 1_000_000) as u64;
        if delta_time_ms < interval_ms {
            sleep(Duration::from_millis(interval_ms - delta_time_ms));
            continue;
        }
        last_time = now;

        if !update(inp, &mut state) {
            break;
        }
        draw(cnv, hs_disp, &mut state);
    }

    state.score
}

fn update(inp: &mut KeyReader, state: &mut GameState) -> bool {
    let key = inp.get_key();
    match key {
        127 => return false, // Backspace -> back to menu
        b'a' => {
            if can_move(state, Dir::Left) {
                let (x, y) = state.curr_shape.pos;
                state.curr_shape.pos = (x - 1.0, y);
            }
        }, b'd' => {
            if can_move(state, Dir::Right) {
                let (x, y) = state.curr_shape.pos;
                state.curr_shape.pos = (x + 1.0, y);
            }
        }
        _ => {}
    }

    if can_move(state, Dir::Down) {
        let (shape_x, shape_y) = state.curr_shape.pos;
        state.curr_shape.pos = (shape_x, shape_y + state.fall_spd);
    }

    true
}

fn draw(cnv: &mut Canvas, hs_disp: &Vec<&String>, state: &mut GameState) {
    let score_str = format!("{:020}", state.score);
    let score_disp = vec![ &score_str ];
    cnv.draw_strs(&BORDER.to_vec(), (1, 1), BORDER_COLOR, &Reset);
    cnv.draw_strings(&hs_disp, (2, 2), SCORE_COLOR, &Reset);
    cnv.draw_strings(&score_disp, (2, 3), SCORE_COLOR, &Reset);
    
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
    for coord in state.curr_shape.coords {
        let (block_x, block_y) = coord;
        
        let x =
            (SHAPE_WIDTH as i16)
                * (block_x + floor(shape_x as f64, 0) as i16) + 2;
        let y = block_y + floor(shape_y as f64, 0) as i16 + 2 + 3;

        cnv.draw_strs(
            &vec![ SHAPE_STR ], (x as u16, y as u16),
            state.curr_shape.fg, &Reset
        );
    }

    cnv.flush();
}

fn can_move(state: &GameState, dir: Dir) -> bool {
    let (shape_x, shape_y) = state.curr_shape.pos;
    let shape_block_x = floor(shape_x as f64, 0) as i16;
    let shape_block_y = floor(shape_y as f64, 0) as i16;

    for coord in state.curr_shape.coords {
        let (mut coord_x, mut coord_y) = coord;
        coord_x += shape_block_x;
        coord_y += shape_block_y;

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
        if coord_x < 0 || coord_x >= GRID_WIDTH as i16
                || coord_y >= GRID_HEIGHT as i16 - 1 {
            return false;
        }

        if coord_y >= 0
                && state.blocks[coord_y as usize][coord_x as usize] != -1 {
            return false;
        }
    }
    true
}
