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
use std::{
    io::{ Write, stdout, Stdout, Read }
};

// Double block shapes in a 10x20 grid plus borders and extra enter space
pub const SHAPE_STR: &'static str = "██";
pub const DISP_WIDTH: u16 = (SHAPE_STR.len() as u16 * 10) + 2; // 10 blks + brdr
pub const DISP_HEIGHT: u16 = 20 + 3; // 20 blocks, 1 space @ top, border, & NL

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

    pub fn draw(
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
