use std::vec;

use crate::types::{Simulation, LED};

mod always_on_sim;
mod flash_every_second;
mod rainbow_flood;
mod height_flicker;

#[must_use] pub fn get_simulations(leds: &[LED]) -> Vec<Box<dyn Simulation>> {
    vec![
        // Box::new(always_on_sim::AlwaysOnSim::new(leds)), // only used for testing, not useful in prod
        Box::new(flash_every_second::FlashEverySecondSim::new(leds)),
        Box::new(rainbow_flood::RainbowFloodSim::new(leds)),
        Box::new(height_flicker::HeightFlickerSim::new(leds)),
    ]
}