use crate::types::{Simulation, RGB, LED};

#[derive(Debug)]
pub struct FlashEverySecondSim {
    last_flash: u64,
    on_now: bool,
}

impl Simulation for FlashEverySecondSim {
    fn new(_leds: &[LED]) -> Self {
        Self { last_flash: 0, on_now: false }
    }

    fn get_name(&self) -> &'static str {
        "Flash every second"
    }

    fn tick(
        &mut self,
        leds: &mut Vec<LED>,
        micros: u64,
        brightness_mod: f32,
    ) {
        let brightness = (255.0 * brightness_mod) as u8;

        if micros - self.last_flash >= 1_000_000 {
            self.on_now = !self.on_now;
            self.last_flash = micros;

            for led in leds.iter_mut() {
                led.color = if self.on_now {
                    RGB { r: brightness, g: brightness, b: brightness }
                } else {
                    RGB { r: 0, g: 0, b: 0 }
                };
            }
        }
    }

}