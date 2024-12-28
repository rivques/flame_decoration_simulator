use std::fmt::Debug;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGB<ComponentType> {
    pub r: ComponentType,
    pub g: ComponentType,
    pub b: ComponentType,
}

#[derive(Debug)]
pub struct LED {
    pub color: RGB<u8>,
    pub coords: (usize, usize),
}

pub trait Simulation: Debug {
    fn tick(
        &mut self,
        leds: &mut Vec<LED>,
        micros: u64,
        intensity_mod: f32,
    );

    fn new(leds: &[LED]) -> Self where Self: Sized;

    fn get_name(&self) -> &'static str;
}