/*
 * Author: Dylan Turner
 * Description: Definition of shapes and their functions
 */

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShapeType { T, L, MirrorL, S, Z, Square, Line }

pub struct Tetromino {
    x : f32,
    y : f32,

    shape : ShapeType,
    
    coords : [[i8; 2]; 4]
}

impl Tetromino {
    fn get_img(&self) -> String {
        return String::new();
    }

    fn get_clear_img(&self) -> String {
        return String::new();
    }

    fn rotate(&self, is_right : bool) {
        
    }
}

const BASE_SHAPE_COORDS : [(ShapeType, [[i8; 2]; 4]); 7] = [
    (ShapeType::T,          [ [ -1,  0 ], [  0,  0 ], [  1,  0 ], [  0,  1 ] ]),
    (ShapeType::L,          [ [ -1, -1 ], [  0, -1 ], [  0,  0 ], [  0,  1 ] ]),
    (ShapeType::MirrorL,    [ [  1, -1 ], [  0, -1 ], [  0,  0 ], [  0,  1 ] ]),
    (ShapeType::S,          [ [  0, -1 ], [  0,  0 ], [  1,  0 ], [  1,  1 ] ]),
    (ShapeType::Z,          [ [  0, -1 ], [  0,  0 ], [ -1,  0 ], [ -1,  1 ] ]),
    (ShapeType::Square,     [ [  0,  0 ], [  1,  0 ], [  0,  1 ], [  1,  1 ] ]),
    (ShapeType::Line,       [ [  0, -1 ], [  0,  0 ], [  0,  1 ], [  0,  2 ] ])
];
