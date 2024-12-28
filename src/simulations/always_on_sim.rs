use crate::types::{Simulation, RGB, LED};

#[derive(Debug)]
pub struct AlwaysOnSim;

impl Simulation for AlwaysOnSim {
    fn tick(
        &mut self,
        leds: &mut Vec<LED>,
        micros: u64,
        brightness_mod: f32,
    ){
        let brightness = (255.0 * brightness_mod) as u8;
        for led in leds.iter_mut() {
            led.color = RGB { r: brightness, g: brightness, b: brightness };
        }
    }

    fn new(leds: &Vec<LED>,) -> Self {
        Self
    }

    fn get_name(&self) -> &'static str {
        "Always on"
    }
}