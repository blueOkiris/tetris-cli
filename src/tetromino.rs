/*
 * Author: Dylan Turner
 * Description: Abstraction of block data into a manipulatable shape
 */

use rand::{
    distributions::{ Distribution, Standard },
    Rng, random
};
use termion::color::{
    Color, Magenta, Red, Green, Yellow, Blue, Cyan, LightYellow
};
use crate::io::SHAPE_STR;

const SHAPE_COORDS: [[(i16, i16); 4]; 7] = [
    [ (-1,  0), (0,  0), ( 1, 0), ( 0, 1) ],
    [ (-1, -1), (0, -1), ( 0, 0), ( 0, 1) ],
    [ ( 1, -1), (0, -1), ( 0, 0), ( 0, 1) ],
    [ ( 0, -1), (0,  0), ( 1, 0), ( 1, 1) ],
    [ ( 0, -1), (0,  0), (-1, 0), (-1, 1) ],
    [ ( 0,  0), (1,  0), ( 0, 1), ( 1, 1) ],
    [ ( 0, -1), (0,  0), ( 0, 1), ( 0, 2) ]
];
const SHAPE_COLORS: [&dyn Color; 7] = [
    &Magenta, &Yellow, &Blue, &Green, &Red, &LightYellow, &Cyan
];

#[derive(Copy, Clone)]
pub enum ShapeType { T, L, Mirror, S, Z, Square, Line }

impl Distribution<ShapeType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ShapeType {
        match rng.gen_range(0..7) {
            0 => ShapeType::T,
            1 => ShapeType::L,
            2 => ShapeType::Mirror,
            3 => ShapeType::S,
            4 => ShapeType::Z,
            5 => ShapeType::Square,
            6 => ShapeType::Line,
            _ => ShapeType::T // Required by rust but shouldn't happen
        }
    }
}

pub struct Tetromino<'a> {
    pub pos: (f32, f32),
    pub shape: ShapeType,
    pub coords: [(i16, i16); 4],
    pub fg: &'a dyn Color
}

impl<'a> Tetromino<'a> {
    pub fn select() -> Self {
        let shape = random();
        Self {
            pos: (7.0, 3.0),
            shape,
            coords: SHAPE_COORDS[shape as usize],
            fg: SHAPE_COLORS[shape as usize]
        }
    }
}
