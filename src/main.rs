mod game;
mod world;
mod particle;
mod utils;
mod particles;
use std::time::Duration;

use game::Game;

pub fn main() {
    let mut game = Game::new(1280, 720, 4);
    game.start();

    while game.running() {
        game.poll_events();
        game.update();
        game.draw();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }
}
