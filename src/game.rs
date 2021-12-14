/*
 * Author: Dylan Turner
 * Description: Main game loop
 */

use std::{
    thread::sleep,
    time::{ Instant, Duration }
};
use termion::color::{ Color, White, Reset };
use math::round::floor;
use crate::io::{ Canvas, KeyReader, DISP_HEIGHT, GRID_WIDTH, GRID_HEIGHT, SHAPE_STR};
use crate::tetromino::Tetromino;

const FPS: u64 = 60;
const BORDER: [&'static str; DISP_HEIGHT as usize] = [
    "╔══════════════════════════════════════╗",
    "║   Your Score:                        ║",
    "║   High Score:                        ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "║                                      ║",
    "╚══════════════════════════════════════╝"
];
const BORDER_COLOR: &dyn Color = &White;
const SCORE_COLOR: &dyn Color = &White;
const INITIAL_FALL_SPD: f32 = 0.01;
const FALL_KEY_SPD: f32 = 20.0;

struct GameState<'a> {
    pub score: u64,
    pub curr_shape: Tetromino<'a>,
    pub fall_spd: f32,
    pub blocks: [[bool; GRID_WIDTH]; GRID_HEIGHT]
}

impl<'a> GameState<'a> {
    pub fn new() -> Self {
        Self {
            score: 0,
            curr_shape: Tetromino::select(),
            fall_spd: INITIAL_FALL_SPD,
            blocks: [
                [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ], [
                    false, false, false, false, false,
                    false, false, false, false, false
                ]
            ]
        }
    }
}

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

        // Handle input
        let key = inp.get_key();
        match key {
            127 => break, // Backspace -> back to menu
            _ => {}
        }

        // TODO: update game state
        state.score = 0;

        // Draw
        let score_str = format!("{:020}", state.score);
        let score_disp = vec![ &score_str ];
        cnv.draw_strs(&BORDER.to_vec(), (1, 1), BORDER_COLOR, &Reset);
        cnv.draw_strings(&hs_disp, (17, 3), SCORE_COLOR, &Reset);
        cnv.draw_strings(&score_disp, (17, 2), SCORE_COLOR, &Reset);
        
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if state.blocks[y][x] {
                    cnv.draw_strs(
                        &vec![ SHAPE_STR ],
                        ((x * SHAPE_STR.len() + 1) as u16, (y + 2) as u16),
                        state.curr_shape.fg, &Reset
                    );
                }
            }
        }
        
        let (shape_x, shape_y) = state.curr_shape.pos;
        for coord in state.curr_shape.coords {
            let (mut block_x, mut block_y) = coord;
            block_x += 2;
            block_y += 2;

            let x = 2 * (block_x + floor(shape_x as f64, 0) as i16) + 1;
            let y = block_y + floor(shape_y as f64, 0) as i16 + 2;

            cnv.draw_strs(
                &vec![ SHAPE_STR ], (x as u16, y as u16),
                state.curr_shape.fg, &Reset
            );
        }

        cnv.flush();
    }

    state.score
}
