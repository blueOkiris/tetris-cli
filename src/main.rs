/*
 * Author: Dylan Turner
 * Description: Main entry point to tetris game program
 */

mod settings;
mod io;
mod state;
mod game;
mod engine;
mod tetromino;

fn main() {
    io::init_logging(settings::LOG_CONFIG_FILE);

    // Set up io
    let mut disp = io::Display::new();
    let inp = io::Input::new();

    let mut data = game::GameData::new();
    
    // Main loop
    engine::run(inp, &mut disp, &mut data);
}
