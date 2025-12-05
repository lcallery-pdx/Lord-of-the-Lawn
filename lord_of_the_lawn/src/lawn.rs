//! Core lawn model used by the Lord of the Lawn game.
//!
//! A `Lawn` tracks its size, how much has been mowed, and the payout per square foot.

use rand::Rng;

// Create a Lawn Struct
/// A single lawn that can be mowed in the game.
///
/// `Lawn` tracks its total `size` in square feet, how much has been
/// `mowed` so far, and how much money each square foot is worth.
pub struct Lawn {
    pub size: u32,
    pub mowed: u32,
    pub payout_per_sqft: f64,
}

// Lawn Implementations
impl Lawn {
    /// Creates a new [`Lawn`] with a random size and the given payout.
    ///
    /// The size is chosen uniformly from 25 up to (but not including) 250
    /// square feet.
    pub fn new(price_per_sqft: f64) -> Self {
        //Generate Random Lawn Size
        let mut rng = rand::rng();
        let size = rng.random_range(25..250);

        //Create New Lawn
        Lawn {
            size,
            mowed: 0,
            payout_per_sqft : price_per_sqft,
        }
    }

    // Mow Function
    /// Mows `amount` square feet of this lawn.
    ///
    /// The [`Lawn::mowed`] value is clamped so that it never exceeds
    /// [`Lawn::size`].
    pub fn mow(&mut self, amount: u32) {
        //Check if amount mowed is less than lawn size, if so mow `amount`
        if self.mowed < self.size {
            self.mowed += amount;
            //If amount mowed is over the lawn size set amount mowed to lawn size
            if self.mowed > self.size {
                self.mowed = self.size;
            }
        }
    }
    
    // Lawn Completed Fuction
    /// Returns `true` if the entire lawn has been mowed.
    pub fn is_complete(&self) -> bool {
        self.mowed >= self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lawn_generation_range() {
        // Create several lawns to test random output
        for _ in 0..50 {
            let lawn = Lawn::new(1.0);

            assert!(
                (25..250).contains(&lawn.size),
                "Generated lawn size {} was not in expected range (25–249)",
                lawn.size
            );

            assert_eq!(
                lawn.mowed, 0,
                "New lawns should always start with mowed = 0"
            );
        }
    }


    #[test]
    fn test_mow_increases_mowed() {
        let mut lawn = Lawn::new(1.0);
        let initial_mowed = lawn.mowed;

        lawn.mow(10);

        assert!(
            lawn.mowed > initial_mowed,
            "Mowing should increase the mowed amount"
        );

        assert!(
            lawn.mowed <= lawn.size,
            "Mowed amount should never exceed lawn size"
        );
    }
}
