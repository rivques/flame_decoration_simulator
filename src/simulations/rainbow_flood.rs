use crate::types::{Simulation, RGB, LED};

#[derive(Debug)]
pub struct RainbowFloodSim { // state used by the simulation goes here
    last_tick: u64, // the last time we were called
    hue: f32, // 0 to 360, how far along the rainbow the bottom of the pattern is
    pattern_height: f32, // the y-distance between the top and bottom LED. calculated at initiation
}

impl RainbowFloodSim {
    fn hsv_to_rgb(h: f32, s: f32, v: f32) -> RGB<u8> { // a helper function to convert HSV to RGB
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        RGB {
            r: ((r + m) * 255.0) as u8,
            g: ((g + m) * 255.0) as u8,
            b: ((b + m) * 255.0) as u8,
        }
    }
}

impl Simulation for RainbowFloodSim {
    fn new(leds: &Vec<LED>) -> Self where Self: Sized { // the constructor for the simulation.
        // calculate the height of the pattern only once
        let pattern_height = leds.iter().map(|led| led.coords.1).max().unwrap() as f32 - leds.iter().map(|led| led.coords.1).min().unwrap() as f32;
        Self { last_tick: 0, hue: 0.0, pattern_height }
    }

    fn get_name(&self) -> &'static str { // this is what shows up in the UI
        "Rainbow flood"
    }

    fn tick(
            &mut self,
            leds: &mut Vec<LED>, // the LEDs we're controlling. We can change their colors here
            micros: u64, // the number of microseconds since the program started
            brightness_mod: f32, // a user-provided value between 0 and 1. should control intensity of the simulation
        ) {
            let hue_speed = 120.0; // config: degrees per second to move the rainbow

            let dt = (micros - self.last_tick) as f32 / 1_000_000.0; // find the time since the last tick
            self.last_tick = micros;

            self.hue += hue_speed * dt; // advance the hue
            self.hue %= 360.0;
            
            for led in leds.iter_mut() {
                // set each LED as appropriate for its coordinates
                let y = led.coords.1 as f32;
                let hue = self.hue + (y / self.pattern_height) * 360.0;
                led.color = Self::hsv_to_rgb(hue % 360.0, 1.0, brightness_mod);
            }
    }
}