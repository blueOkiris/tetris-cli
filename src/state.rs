/*
 * Author: Dylan Turner
 * Description: Struct for defining game states
 */

use crate::io::Display;
use crate::game::GameData;

use termion::event::Key;

type DrawFn = fn(data : &mut GameData, disp : &mut Display);
type UpdateFn = fn(data : &mut GameData, keys : Vec<Key>);

#[derive(Clone, Copy)]
pub struct State {
    pub draw : DrawFn,
    pub update : UpdateFn
}
