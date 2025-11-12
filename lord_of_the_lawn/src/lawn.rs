use rand::Rng;

// Create a Lawn Struct
pub struct Lawn {
    pub size: u32,
    pub mowed: u32,
    pub payout_per_sqft: f64
}


// Lawn Implementations
impl Lawn {
    pub fn new(price_per_sqft: f64) -> Self {

        //Generate Random Lawn Size
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(25..250);

        //Create New Lawn
        Lawn {
            size,
            mowed: 0,
            payout_per_sqft: price_per_sqft
        }
    }

// Mow Function
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
    pub fn is_complete(&self) -> bool {
        self.mowed >= self.size
    }
}