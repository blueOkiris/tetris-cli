/*
 * Author: Dylan Turner
 * Description: Handle key input and drawing
 */

use termion::{
    clear::All, cursor::{ Goto, Hide },
    raw::{ RawTerminal, IntoRawMode },
    color::{ Color, Fg, Bg, Reset },
    async_stdin, AsyncReader
};
use std::io::{ Write, stdout, Stdout, Read };

// Double block shapes in a 10x20 grid plus borders and extra enter space
pub const SHAPE_STR: &'static str = "██";
pub const SHAPE_WIDTH: usize = 2;
pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;
pub const DISP_WIDTH: u16 = (SHAPE_WIDTH * GRID_WIDTH) as u16 + 2; 
pub const DISP_HEIGHT: u16 = GRID_HEIGHT as u16 + 4;

// An object that lets you draw to it
pub struct Canvas {
    out: RawTerminal<Stdout>
}

impl Canvas {
    pub fn new() -> Self {
        let mut out = stdout().into_raw_mode().unwrap();
        write!(out, "{}{}", All, Goto(1, 1)).unwrap();
        out.flush().unwrap();

        write!(out, "{}", Hide).unwrap(); // Hide the cursor

        Self { out }
    }

    // Can't figure out how to combine the two due to &'static part of str
    pub fn draw_strs(
            &mut self, lines: &Vec<&'static str>, pos: (u16, u16),
            fg: &dyn Color, bg: &dyn Color) {
        let (x, mut y) = pos;
        for line in lines {
            write!(
                self.out, "{}{}{}{}{}{}",
                Goto(x, y), Fg(fg), Bg(bg), line,
                Fg(Reset), Bg(Reset)
            ).unwrap();
            y += 1;
        }
    }

    pub fn draw_strings(
            &mut self, lines: &Vec<&String>, pos: (u16, u16),
            fg: &dyn Color, bg: &dyn Color) {
        let (x, mut y) = pos;
        for line in lines {
            write!(
                self.out, "{}{}{}{}{}{}",
                Goto(x, y), Fg(fg), Bg(bg), line,
                Fg(Reset), Bg(Reset)
            ).unwrap();
            y += 1;
        }
    }

    pub fn flush(&mut self) {
        self.out.flush().unwrap();
    }
}

// An object that lets you read key presses
pub struct KeyReader {
    inp: AsyncReader
}

impl KeyReader {
    pub fn new() -> Self {
        let inp = async_stdin();
        Self { inp }
    }

    pub fn get_key(&mut self) -> u8 {
        let mut key_bytes: [u8; 1] = [ 0 ];
        self.inp.read(&mut key_bytes).unwrap();
        return  key_bytes[0];
    }
}
