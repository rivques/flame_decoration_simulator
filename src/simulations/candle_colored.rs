use crate::{trace_dbg, types::{Simulation, LED, RGB}};

// notes on candle flames:
// - base of flame is near-transparent, slightly blue
// black wick in middle-lower-third
// above wick, flame is orange-yellow-white-yellow
// flame flickers in height and side-to-side

#[derive(Debug)]
pub struct CandleColoredSim {
    last_tick: u64,
}

impl CandleColoredSim {
    fn get_horiz_flicker(t: f32) -> f32 {
        // a periodic function that returns a value between -4 and 4
        let result = 1.5 * (f32::sin(2.0 * t) + f32::sin(t) + 0.3 * f32::sin(12.0 * t) + 0.1 * f32::sin(100.0 * t));
        result
    }
    fn get_vert_flicker(t: f32) -> f32 {
        // a periodic function that returns a value between -10 and 10
        let result = 5.0 * (0.4 * f32::sin(t) + 0.3 * f32::sin(2.0 * t) + f32::sin(3.0 * t) + 0.3 * f32::sin(8.0 * t) + 0.05 * f32::sin(130.0 * t));
        result
    }
    
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

impl Simulation for CandleColoredSim {
    fn get_name(&self) -> &'static str {
        "Candle flame (colored)"
    }

    fn new(_leds: &[LED]) -> Self where Self: Sized {
        // this pattern is expecting our actual layout, so we don't do fancy calculations here
        Self { last_tick: 0 }
    }

    fn tick(
            &mut self,
            leds: &mut Vec<LED>,
            micros: u64,
            intensity_mod: f32,
        ) {
            let candle_base = (107, 3); // config: where the base of the candle is
            let blue_leds = [0, 8]; // config: which LEDs are the base of the flame
            let wick_leds = [9, 10]; // config: which LEDs are the wick
            let flame_leds = [1, 2, 3, 4, 5, 6, 7, 11]; // config: which LEDs are the flame

            let blue_hue = 200.0; // config: hue of the base of the flame
            let wick_hue = 30.0; // config: hue of the wick
            let flame_hue = 40.0; // config: base hue of the flame


            let dt = (micros - self.last_tick) as f32 / 1_000_000.0; // find the time since the last tick
            self.last_tick = micros;

            // overall steps:
            // 1. get the current flicker
            // 2. set the wick color (near-constant)
            // 3. set the blue color (varies slightly with flicker)
            // 4. set the flame color (varies with flicker)

            let horiz_flicker = Self::get_horiz_flicker(micros as f32 / 1_000_000.0);
            let vert_flicker = Self::get_vert_flicker(micros as f32 / 1_000_000.0);

            let log_str = format!("horiz flicker: {0:.2}, vert flicker: {1:.2}", horiz_flicker, vert_flicker);
            trace_dbg!(log_str);

            for wick_led in wick_leds.iter() {
                leds[*wick_led].color = RGB { r: 0, g: 0, b: 0 };
            }

            for blue_led in blue_leds.iter() {
                // blue leds should be more intense when the flame is vert-flickered higher or horiz-flicker to their side
                // they should also be slightly dimmer when the intensity is lower
                let base_brightness = 0.2;
                let horiz_component = 0.02 * horiz_flicker; // -0.08 to 0.08
                let vert_component = 0.005 * vert_flicker; // -0.05 to 0.05
                let intensity_component = 0.03 * intensity_mod; // 0.0 to 0.03
                
                let brightness = base_brightness + horiz_component + vert_component + intensity_component; // 0.07 to 0.36
            
                leds[*blue_led].color = Self::hsv_to_rgb(blue_hue, 1.0, brightness);
            }
            

            // TODO: flame logic
    }
}