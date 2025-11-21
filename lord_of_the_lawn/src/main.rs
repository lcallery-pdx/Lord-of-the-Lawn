mod lawn;
mod game;

use crate::game::Game;

fn main() {
    println!("Welcome to Lord of the Lawn!");
    println!("Press [Enter] to mow, [U] to upgrade, [Q] to quit.\n");
    // ToDo - Start Game

    let mut game = Game::new();
}
