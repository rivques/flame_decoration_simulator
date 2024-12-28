use crate::{trace_dbg, types::{Simulation, LED, RGB}};

#[derive(Debug)]
pub struct HeightFlickerSim {
    last_tick: u64,
    current_height: f32, // from 0 to 1, where the pattern is vertically
    pattern_height: f32,
}
impl Simulation for HeightFlickerSim {
    fn new(leds: &[LED]) -> Self where Self: Sized { // the constructor for the simulation.
        // calculate the height of the pattern only once
        let pattern_height = leds.iter().map(|led| led.coords.1).max().unwrap() as f32 - leds.iter().map(|led| led.coords.1).min().unwrap() as f32;
        Self { last_tick: 0, current_height: 0.5, pattern_height }
    }

    fn get_name(&self) -> &'static str { // this is what shows up in the UI
        "Height flicker"
    }

    fn tick(
        &mut self,
        leds: &mut Vec<LED>,
        micros: u64,
        brightness_mod: f32,
    ) {
        let color = RGB {r: 255f32, g: 30f32, b: 0f32}; // config: the color of the pattern
        let variance_per_second = (6.0 * brightness_mod) + 0.5; // config: what % of height the pattern can move up or down per second
        let center_bias = 0.2; // config: how much the pattern is biased towards the center
        // TODO: center harder when burning softer

        let dt = (micros - self.last_tick) as f32 / 1_000_000.0; // find the time since the last tick
        self.last_tick = micros;

        let nudge = (rand::random::<f32>() - 0.5) * variance_per_second * dt; // how much to move the pattern
        self.current_height += nudge;
        self.current_height += (0.5 - self.current_height) * center_bias * dt; // bias towards the center

        if self.current_height < 0.0 {
            self.current_height = 0.0;
        } else if self.current_height > 1.0 {
            self.current_height = 1.0;
        }

        for (i, led) in leds.iter_mut().enumerate() {
            let distance = (led.coords.1 as f32 - self.current_height * self.pattern_height); // how far the LED is from the pattern
            let brightness = 
                if distance > 2.0 {
                    0.0
                } else {
                    if distance < -2.0 {
                        1.0
                    } else {
                        1.0 - (distance.abs() / 2.0)
                    }
                };
            let log_str = format!("current height: {}, led: {}, distance: {}, brightness: {}", self.current_height, i, distance, brightness);
            trace_dbg!(log_str);
            led.color = RGB {
                r: (color.r as f32 * brightness) as u8,
                g: (color.g as f32 * brightness) as u8,
                b: (color.b as f32 * brightness) as u8,
            };
        }

    }
}