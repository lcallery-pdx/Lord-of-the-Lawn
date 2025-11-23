use crate:: lawn::Lawn;
use std::io::stdout;
use std::io::Write;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub struct Game {
    current_lawn: Lawn,
    total_money: f64,
    price_per_sqft: f64,
    mower_efficiency: u32
}

impl Game {
    pub fn new() -> Self{
        Game {
            current_lawn: Lawn::new(0.25),
            total_money: 0.0,
            price_per_sqft: 0.25,
            mower_efficiency: 1
        }
    }

    pub fn run(&mut self) {
        loop {
            self.display_status();

            // Wait for user input (non-blocking timeout)
            if event::poll(Duration::from_millis(500)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    match key_event.code {
                        KeyCode::Enter => self.mow_one_sqft(),
                        KeyCode::Char('u') | KeyCode::Char('U') => self.upgrade_mower(),
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            println!("\nGoodbye! You earned ${:.2}.", self.total_money);
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }


    fn mow_one_sqft(&mut self) {
        self.current_lawn.mow(self.mower_efficiency);

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

    fn display_status(&self) {
        print!(
            "\rCurrent Lawn: {}/{} sqft | Money: ${:.2} | Rate: ${:.2}/sqft | Mower: {} sqft/click   ",
            self.current_lawn.mowed,
            self.current_lawn.size,
            self.total_money,
            self.price_per_sqft,
            self.mower_efficiency
        );
        stdout().flush().unwrap();
    }

     fn upgrade_mower(&mut self) {
        self.mower_efficiency += 1;
        println!(
            "\nMower upgraded! Now mowing {} sqft per click.",
            self.mower_efficiency
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*; // brings Game and Lawn into scope

    #[test]
    fn upgrade_mower_increases_efficiency() {
        let mut game = Game::new();

        let old_eff = game.mower_efficiency;
        game.upgrade_mower();

        assert_eq!(game.mower_efficiency, old_eff + 1);
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

        game.mow_one_sqft();

        assert_eq!(game.current_lawn.mowed, 5);
    }
}
