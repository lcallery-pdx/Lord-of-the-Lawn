use crate:: lawn::Lawn;
use std::io::stdout;
use std::io::Write;


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


    fn mow_one_sqft(&mut self) {
        self.current_lawn.mow(self.mower_efficiency);

        if self.current_lawn.is_complete() {
            let payout = self.current_lawn.size as f64 * self.price_per_sqft;
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