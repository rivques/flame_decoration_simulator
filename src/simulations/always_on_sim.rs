use crate::types::{Simulation, RGB, LED};

#[derive(Debug)]
pub struct AlwaysOnSim;

impl Simulation for AlwaysOnSim {
    fn tick(
        &mut self,
        leds: &mut Vec<LED>,
        _micros: u64,
        intensity_mod: f32,
    ){
        let brightness = (255.0 * intensity_mod) as u8;
        for led in leds.iter_mut() {
            led.color = RGB { r: brightness, g: brightness, b: brightness };
        }
    }

    fn new(_leds: &[LED],) -> Self {
        Self
    }

    fn get_name(&self) -> &'static str {
        "Always on"
    }
}