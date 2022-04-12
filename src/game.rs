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
    "║                    ║",
    "╚════════════════════╝"
];
const BORDER_COLOR: &dyn Color = &White;
const SCORE_COLOR: &dyn Color = &White;
const SHAPE_DRAW_OFFSET: i16 = 5;
const INITIAL_FALL_SPD: f32 = 0.9;
const LAND_TIME_DELAY_S: f64 = 0.1;
const SPD_INC: f32 = 0.05;
const ROW_SCORE_INC: u64 = 100;
const PIECE_SCORE_INC: u64 = 100;
const PIECE_SCORE_FAL_MULT: f32 = 50.0;

#[derive(PartialEq, Clone, Copy)]
pub enum Dir {
    Down,
    Left,
    Right
}

pub struct GameState {
    score: u64,
    curr_shape: Tetromino,
    fall_spd: f32,
    blocks: [[i8; GRID_WIDTH]; GRID_HEIGHT],
    land_timer: f64
}

enum UpdateEndState {
    Quit,
    Lost,
    Continue
}

impl GameState {
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

    pub fn play(&mut self, cnv: &mut Canvas, inp: &mut KeyReader, hs_disp: &Vec<&String>) -> u64 {
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

            match self.update(inp, delta_time_ms) {
                UpdateEndState::Continue => {},
                UpdateEndState::Quit => {
                    return 0;
                }, UpdateEndState::Lost => {
                    break;
                }
            }
            self.draw(cnv, hs_disp);
        }

        self.score
    }
    
    fn update(&mut self, inp: &mut KeyReader, delta_time_ms: u64) -> UpdateEndState {
        let key = inp.get_key();
        match key {
            127 => return UpdateEndState::Quit, // Backspace -> back to menu
            b'a' => {
                if self.can_move_curr_shape(Dir::Left) {
                    self.curr_shape.pos.0 -= 1.0;
                }
            }, b'd' => {
                if self.can_move_curr_shape(Dir::Right) {
                    self.curr_shape.pos.0 += 1.0;
                }
            }, b'q' => {
                if self.can_rotate_curr_shape(Dir::Left) {
                    self.curr_shape.rotate(Dir::Left);
                }
            }, b'e' => {
                if self.can_rotate_curr_shape(Dir::Right) {
                    self.curr_shape.rotate(Dir::Right);
                }
            }, b's' => {
                self.curr_shape.pos.1 = floor(self.curr_shape.pos.1 as f64, 0) as f32;
                while self.can_move_curr_shape(Dir::Down) {
                    self.curr_shape.pos.1 += 0.5; // Make sure not to skip
                }
            }
            _ => {}
        }

        if self.can_move_curr_shape(Dir::Down) {
            self.curr_shape.pos.1 += self.fall_spd * (delta_time_ms as f32 / 1_000.0);
        } else if self.land_timer > 0.0 { // Allow a few ms for moving b4 settling
            self.land_timer -= delta_time_ms as f64 / 1_0000.0;
        } else if self.curr_shape.pos.1 <= 1.0 { // Landed at start means death
            return UpdateEndState::Lost;
        } else {
            self.score += PIECE_SCORE_INC + (self.fall_spd * PIECE_SCORE_FAL_MULT) as u64;

            self.save_tetromino();
            self.check_rows();

            self.land_timer = LAND_TIME_DELAY_S;
            self.curr_shape = Tetromino::select();
        }

        UpdateEndState::Continue
    }

    // Permanently store the block data of the current shape after landing
    fn save_tetromino(&mut self) {
        let (shape_x, shape_y) = self.curr_shape.pos;
        let shape_block_x = floor(shape_x as f64, 0) as i16;
        let shape_block_y = floor(shape_y as f64, 0) as i16;
        for coord in self.curr_shape.coords {
            let (mut coord_x, mut coord_y) = coord;
            coord_x += shape_block_x;
            coord_y += shape_block_y;

            self.blocks[coord_y as usize][coord_x as usize] = self.curr_shape.fg as i8;
        }
    }

    // Check if we can delete rows and shift everything
    fn check_rows(&mut self) {
        let mut num_filled_rows = 0;

        for y in 0..GRID_HEIGHT {
            let mut row_full = true;
            for x in 0..GRID_WIDTH {
                if self.blocks[y][x] == -1 {
                    row_full = false;
                    break;
                }
            }
            if row_full {
                num_filled_rows += 1;

                // Update game speed
                self.fall_spd += SPD_INC;

                // Move rows above down (will also replace data in row y, so no clear needed)
                for y_above in (0..y).rev() {
                    for x in 0..GRID_WIDTH {
                        self.blocks[y_above + 1][x] = self.blocks[y_above][x];
                    }
                }
            }
        }

        if num_filled_rows == 1 {
            self.score += ROW_SCORE_INC;
        } else if num_filled_rows > 1 {
            self.score += ROW_SCORE_INC * ((num_filled_rows - 1) << 1); // eg 100 to 200, 400, 600
        }
    }

    fn draw(&mut self, cnv: &mut Canvas, hs_disp: &Vec<&String>) {
        cnv.draw_strs(&BORDER.to_vec(), (1, 1), BORDER_COLOR, &Reset);

        let score_str = format!("{:020}", self.score);
        let score_disp = vec![ &score_str ];
        cnv.draw_strings(&hs_disp, (3, 1), SCORE_COLOR, &Reset);
        cnv.draw_strings(&score_disp, (3, 2), SCORE_COLOR, &Reset);
        
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if self.blocks[y][x] != -1 {
                    cnv.draw_strs(
                        &vec![ SHAPE_STR ],
                        ((x * SHAPE_WIDTH + 2) as u16, (y + SHAPE_DRAW_OFFSET as usize) as u16),
                        SHAPE_COLORS[self.blocks[y][x] as usize], &Reset
                    );
                }
            }
        }

        // Dealing with whole display! Not just grid
        let (shape_x, shape_y) = self.curr_shape.pos;
        let shape_block_x = floor(shape_x as f64, 0) as i16;
        let shape_block_y = floor(shape_y as f64, 0) as i16;
        for coord in self.curr_shape.coords {
            let (mut coord_x, mut coord_y) = coord;
            coord_x += shape_block_x;
            coord_y += shape_block_y;

            let x = coord_x * SHAPE_WIDTH as i16 + 2;
            let y = coord_y + SHAPE_DRAW_OFFSET;

            cnv.draw_strs(
                &vec![ SHAPE_STR ], (x as u16, y as u16), SHAPE_COLORS[self.curr_shape.fg], &Reset
            );
        }

        cnv.flush();
    }

    // Check if a block is able to move in a given direction
    fn can_move_curr_shape(&mut self, dir: Dir) -> bool {
        // Get position in a grid format (we want slow movement, so we use actually use floats)
        let (shape_x, shape_y) = self.curr_shape.pos;
        let shape_block_x = floor(shape_x as f64, 0) as i16;
        let shape_block_y = floor(shape_y as f64, 0) as i16;

        for coord in self.curr_shape.coords {
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
            if coord_x < 0 || coord_x >= GRID_WIDTH as i16 || coord_y >= GRID_HEIGHT as i16 {
                return false;
            }

            if coord_y >= 0 && self.blocks[coord_y as usize][coord_x as usize] != -1 {
                return false;
            }
        }
        true
    }

    // Basically check if "can_move" to the current position after rotation
    fn can_rotate_curr_shape(&mut self, dir: Dir) -> bool {
        if dir == Dir::Down {
            return true;
        }

        // Create temp shape and rotate it
        let mut temp_shape = self.curr_shape.clone();
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
            if coord_x < 0 || coord_x >= GRID_WIDTH as i16 || coord_y >= GRID_HEIGHT as i16 {
                return false;
            }

            if coord_y >= 0 && self.blocks[coord_y as usize][coord_x as usize] != -1 {
                return false;
            }
        }
        true
    }
}
