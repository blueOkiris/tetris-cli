/*
 * Author: Dylan Turner
 * Description: Abstraction of block data into a manipulatable shape
 */

use rand::{
    distributions::{
        Distribution, Standard
    }, Rng, random
};
use termion::color::{
    Color, Magenta, Red, Green, Yellow, Blue, Cyan, LightYellow
};
use crate::game::Dir;

/*
 * Shape could be -2 to 2 in all directions (bc rotations)
 * so needs to be 3x4 shape
 *      -2  -1  0   1    2
 *    _____________________
 * -2 |   |   |   |   |   |
 *    _____________________
 * -1 |   |   |   |   |   |
 *    ---------------------
 *  0 |   |   |   |   |   |
 *    ---------------------
 *  1 |   |   |   |   |   |
 *    ---------------------
 *  2 |   |   |   |   |   |
 *    ---------------------
 */
const SHAPE_COORDS: [[(i16, i16); 4]; 7] = [
    [ (-1,  0), (0,  0), ( 1, 0), ( 0, 1) ],
    [ (-1, -1), (0, -1), ( 0, 0), ( 0, 1) ],
    [ ( 1, -1), (0, -1), ( 0, 0), ( 0, 1) ],
    [ ( 0, -1), (0,  0), ( 1, 0), ( 1, 1) ],
    [ ( 0, -1), (0,  0), (-1, 0), (-1, 1) ],
    [ ( 0,  0), (1,  0), ( 0, 1), ( 1, 1) ],
    [ ( 0, -1), (0,  0), ( 0, 1), ( 0, 2) ]
];
pub const SHAPE_COLORS: [&dyn Color; 7] = [
    &Magenta, &Yellow, &Blue, &Green, &Red, &LightYellow, &Cyan
];

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub pos: (f32, f32),
    pub shape: ShapeType,
    pub coords: [(i16, i16); 4],
    pub fg: usize
}

impl Tetromino {
    pub fn select() -> Self {
        let shape = random();
        Self {
            pos: (4.0, 0.0),
            shape,
            coords: SHAPE_COORDS[shape as usize],
            fg: shape as usize
        }
    }

    pub fn rotate(&mut self, dir: Dir) {
        if self.shape == ShapeType::Square {
            return;
        }

        for i in 0..4 {
            let (x, y) = self.coords[i];
            match dir {
                Dir::Left => self.coords[i] = (y, -x),
                Dir::Right => self.coords[i] = (-y, x),
                _ => {}
            }
        }
    }
}
