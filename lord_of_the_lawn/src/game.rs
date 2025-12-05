//! Game loop and high-level progression logic for Lord of the Lawn.
//!
//! This module owns the overall game state, including the current [`Lawn`],
//! upgrade costs, and user input handling.

use crate::lawn::Lawn;
use crossterm::event::{self, Event, KeyCode};
use std::io::Write;
use std::io::stdout;
use std::time::{Duration, Instant};

/// Top–level game state and logic for Lord of the Lawn.
///
/// A `Game` owns the currently active [`Lawn`], the player's money,
/// and all upgrade costs and mowing rates.
pub struct Game {
    current_lawn: Lawn,
    total_money: f64,
    price_per_sqft: f64,
    mower_efficiency: u32,
    mower_upgrade_cost: f64,
    pub auto_mower_rate: f64,
    pub auto_mower_upgrade_cost: f64,
}

/// Creates a new [`Game`] with a fresh [`Lawn`] and default pricing.
impl Game {
    pub fn new() -> Self {
        Game {
            current_lawn: Lawn::new(0.25),
            total_money: 0.0,
            price_per_sqft: 0.25,
            mower_efficiency: 1,
            mower_upgrade_cost: 10.0,
            auto_mower_rate: 0.0,
            auto_mower_upgrade_cost: 25.0,
        }
    }
    /// Starts the main game loop.
    ///
    /// This method blocks until the player chooses to quit with `Q`.
    pub fn run(&mut self) {
        let mut last_tick = Instant::now();

        loop {
            // time-based update for automower
            let now = Instant::now();
            let elapsed = now.duration_since(last_tick);
            last_tick = now;
            self.tick(elapsed.as_secs_f64());

            self.display_status();

            // Wait for user input (non-blocking timeout)
            if event::poll(Duration::from_millis(500)).unwrap()
                && let Event::Key(key_event) = event::read().unwrap()
            {
                match key_event.code {
                    KeyCode::Enter => self.mow_manual(),
                    KeyCode::Char('u') | KeyCode::Char('U') => self.upgrade_mower(),
                    KeyCode::Char('a') | KeyCode::Char('A') => self.upgrade_auto_mower(),
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        println!("\nGoodbye! You earned ${:.2}.", self.total_money);
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    /// Advances the simulation forward by `elapsed_secs` seconds.
    ///
    /// This applies progress from the auto-mower, if it is enabled.
    fn tick(&mut self, elapsed_secs: f64) {
        if self.auto_mower_rate > 0.0 && elapsed_secs > 0.0 {
            let amount = (self.auto_mower_rate * elapsed_secs).floor() as u32;
            if amount > 0 {
                self.current_lawn.mow(amount);
                self.handle_completed_lawn();
            }
        }
    }

    /// Checks whether the current lawn is complete and, if so, pays out
    /// the reward and replaces it with a new randomly sized lawn.
    fn handle_completed_lawn(&mut self) {
        if self.current_lawn.is_complete() {
            let payout = self.current_lawn.size as f64 * self.current_lawn.payout_per_sqft;
            self.total_money += payout;
            println!(
                "\nLawn complete! You earned ${:.2} for {} sqft.",
                payout, self.current_lawn.size
            );

            self.current_lawn = Lawn::new(self.price_per_sqft);
        }
    }

    /// Handles a single manual mow action based on `mower_efficiency`.
    fn mow_manual(&mut self) {
        self.current_lawn.mow(self.mower_efficiency);
        self.handle_completed_lawn();
    }

    /// Attempts to purchase an auto-mower upgrade.
    ///
    /// On success this increases [`Game::auto_mower_rate`] and doubles
    /// [`Game::auto_mower_upgrade_cost`].
    fn upgrade_auto_mower(&mut self) {
        if self.total_money >= self.auto_mower_upgrade_cost {
            let cost = self.auto_mower_upgrade_cost;
            self.total_money -= cost;
            // each upgrade adds 1 sqft/sec
            self.auto_mower_rate += 1.0;
            self.auto_mower_upgrade_cost *= 2.0;

            println!(
                "\nAuto-mower upgraded! Now cutting {:.1} sqft/sec. Next upgrade costs ${:.2}.",
                self.auto_mower_rate, self.auto_mower_upgrade_cost
            );
        } else {
            println!(
                "\nNot enough money to upgrade auto-mower! Need ${:.2}, have ${:.2}.",
                self.auto_mower_upgrade_cost, self.total_money
            );
        }
    }

    /// Prints the current game status line to the terminal.
    fn display_status(&self) {
        print!(
            "\rCurrent Lawn: {}/{} sqft | Money: ${:.2} | Rate: ${:.2}/sqft | Mower: {} sqft/click | Upgrade: ${:.2} (U) | Auto: {:.1} sqft/s (A: ${:.2})   ",
            self.current_lawn.mowed,
            self.current_lawn.size,
            self.total_money,
            self.price_per_sqft,
            self.mower_efficiency,
            self.mower_upgrade_cost,
            self.auto_mower_rate,
            self.auto_mower_upgrade_cost,
        );
        stdout().flush().unwrap();
    }

    /// Attempts to purchase a manual mower upgrade.
    ///
    /// On success this increases `mower_efficiency` and doubles
    /// [`Game::mower_upgrade_cost`].
    fn upgrade_mower(&mut self) {
        if self.total_money >= self.mower_upgrade_cost {
            let cost = self.mower_upgrade_cost;
            self.total_money -= cost;
            self.mower_efficiency += 1;

            self.mower_upgrade_cost *= 2.0;

            println!(
                "\n Mower upgraded! Now mowing {} sqft per click. Next upgrade costs ${:.2}.",
                self.mower_efficiency, self.mower_upgrade_cost
            );
        } else {
            println!(
                "\n Not enough money to upgrade! Need ${:.2}, have ${:.2}.",
                self.mower_upgrade_cost, self.total_money
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upgrade_mower_increases_efficiency() {
        let mut game = Game::new();

        game.total_money = 100.0;
        let old_eff = game.mower_efficiency;
        let old_cost = game.mower_upgrade_cost;

        game.upgrade_mower();

        assert_eq!(game.mower_efficiency, old_eff + 1);
        assert_eq!(game.mower_upgrade_cost, old_cost * 2.0);
        assert!((game.total_money - (100.0 - old_cost)).abs() < 1e-6);
    }

    #[test]
    fn mow_one_sqft_increases_mowed_by_efficiency() {
        let mut game = Game::new();

        // Set up a predictable lawn
        game.current_lawn = Lawn {
            size: 100,
            mowed: 0,
            payout_per_sqft: game.price_per_sqft,
        };

        game.mower_efficiency = 5;

        game.mow_manual();

        assert_eq!(game.current_lawn.mowed, 5);
    }

    #[test]
    fn tick_applies_auto_mower_progress() {
        let mut game = Game::new();

        game.current_lawn = Lawn {
            size: 100,
            mowed: 0,
            payout_per_sqft: game.price_per_sqft,
        };
        game.auto_mower_rate = 10.0;

        game.tick(3.0);

        assert_eq!(game.current_lawn.mowed, 30);
    }
}
