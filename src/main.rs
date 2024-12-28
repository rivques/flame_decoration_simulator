use std::thread;

pub use app::App;
use logging::initialize_logging;
use types::{Simulation, LED};

pub mod app;
pub mod simulations;
pub mod types;
pub mod logging;

const LED_POSITIONS: [(usize, usize); 12] = [
    (103, 4),
    (104, 11),
    (105, 17),
    (106, 24),
    (110, 30),
    (115, 24),
    (118, 17),
    (119, 10),
    (120, 3),
    (112, 3),
    (111, 11),
    (111, 18),
];

fn main() -> color_eyre::Result<()> {
    initialize_logging()?;
    thread::sleep(std::time::Duration::from_secs(1));
    trace_dbg!("Starting up");
    let leds: Vec<_> = LED_POSITIONS
        .iter()
        .map(|(x, y)| LED {
            color: types::RGB { r: 0, g: 0, b: 0 },
            coords: (*x, *y),
        })
        .collect();

    let simulations: Vec<Box<dyn Simulation>> = simulations::get_simulations(&leds);

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new(simulations, leds).run(terminal);
    ratatui::restore();
    result
}
