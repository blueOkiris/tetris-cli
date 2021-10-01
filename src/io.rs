/*
 * Author: Dylan Turner
 * Description: Functions for manipulation of screen data
 */

use termion::raw::{ IntoRawMode, RawTerminal };
use termion::{ clear, cursor, color };
use termion::input::TermRead;
use termion::event::Key;

use std::io::{ stdin, stdout, Write, Stdout, Error };

use std::sync::mpsc::{ channel, Receiver };
use std::thread;

use log::info;
use log4rs;

pub fn init_logging(log_file : &str) {
    log4rs::init_file(log_file, Default::default()).unwrap();
    info!("Started logging tetris-cli!");
}

pub struct Display {
    output : RawTerminal<Stdout>
}

impl Display {
    pub fn new() -> Display {
        let output = stdout().into_raw_mode().unwrap();
        return Display { output };
    }

    pub fn clear(&mut self) {
        write!(
            &mut self.output, "{}{}{}", 
            color::Fg(color::Reset),
            color::Bg(color::Reset),
            clear::All
        ).unwrap();
    }

    pub fn goto(&mut self, x : u16, y : u16) {
        write!(&mut self.output, "{}", cursor::Goto(x, y)).unwrap();
    }
    
    pub fn set_fg<C : color::Color>(&mut self, fg : C) {
        write!(&mut self.output, "{}", color::Fg(fg)).unwrap();
    }
    
    pub fn set_bg<C : color::Color>(&mut self, bg : C) {
        write!(&mut self.output, "{}", color::Bg(bg)).unwrap();
    }

    pub fn write(&mut self, msg : &str) {
        write!(&mut self.output, "{}", msg).unwrap();
    }
}

pub struct Input {
    rx : Receiver<Result<Key, Error>>
}

impl Input {
    pub fn new() -> Input {
        let input = stdin();
        let (tx, rx) = channel();
        thread::spawn(move || {
            // Read from keyboard
            for key in input.keys() {
                tx.send(key).unwrap(); // Send down channel
            }
        });
        return Input { rx };
    }

    pub fn read_available(&self) -> Vec<Key> {
        let mut keys = Vec::new();
        for key in self.rx.try_iter() {
            keys.push(key.unwrap());
        }
        return keys;
    }
}
