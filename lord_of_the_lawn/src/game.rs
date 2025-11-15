use crate:: lawn::Lawn;

pub struct Game {
    current_lawn: Lawn,
    total_money: f64,
    price_per_sqft: f64,
    mower_efficiency: u32
}