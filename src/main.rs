mod utility_funcs;
mod game_loop;
mod player_turn;
mod splash_screen;
use std::{thread, time};

fn main() {
    utility_funcs::clear_terminal();
    splash_screen::main();
    thread::sleep(time::Duration::from_secs(1));
    utility_funcs::clear_terminal();
    game_loop::run();
}
