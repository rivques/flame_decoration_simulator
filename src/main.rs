pub use app::App;
use simulations::{AlwaysOnSim, FlashEverySecondSim};
use types::{Simulation, LED};

pub mod app;
pub mod types;
pub mod simulations;

const LED_POSITIONS: [(usize, usize); 12] = [
    (103, 96),
    (104, 89),
    (105, 83),
    (106, 76),
    (110, 70),
    (115, 76),
    (118, 83),
    (119, 90),
    (120, 97),
    (112, 97),
    (111, 89),
    (111, 82),
];

fn main() -> color_eyre::Result<()> {
    let mut leds: Vec<_> = LED_POSITIONS
        .iter()
        .map(|(x, y)| LED {
            color: types::RGB { r: 0, g: 0, b: 0 },
            coords: (*x, *y),
        })
        .collect();    

    let simulations: Vec<Box<dyn Simulation>> = vec![
        Box::new(AlwaysOnSim::new(&leds)),
        Box::new(FlashEverySecondSim::new(&leds)),
    ];

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new(simulations, leds).run(terminal);
    ratatui::restore();
    result
}
