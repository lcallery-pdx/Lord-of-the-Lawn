//! Entry point for the Lord of the Lawn game.
//!
//! This binary wires up the [`Game`] type and starts the main loop.

mod game;
mod lawn;

use crate::game::Game;

/// Starts the Lord of the Lawn terminal game.
///
/// Prints a short introduction and then hands control to [`Game::run`].
fn main() {
    println!("Welcome to Lord of the Lawn!");
    println!(
        "Press [Enter] to mow, [U] to upgrade mow rate, [A] to upgrade automower, [Q] to quit.\n"
    );

    // Start a new game and enter the main loop.
    let mut game = Game::new();
    game.run();
}
