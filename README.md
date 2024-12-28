# flame_decoration_simulator

A simulator for developing animation for my [fire decoration PCB](https://github.com/rivques/fire_decoration). The intention is that since both this sim and the PCB are written in Rust, I can copy-paste the animations I've written here onto the board once it's manufactured.

## Usage
Note: this project uses weird characters that don't display properly in some terminals. I _highly_ recommend using a terminal like [Alacritty](https://alacritty.org/) that supports full Unicode and true color.

Clone the project, then use `cargo run`. 

Coming soon: this project will be usable on Cargo Cult.www

## Writing a new simulation
1. Make a new file in `src/simulations/` with the name of your simulation.
2. Make a struct that implements the `Simulation` trait. `RainbowFlood` is a good example to build off of.
3. Go to `src/simmulations.rs`.
4. Add your simulation to the vec returned by `get_simulations()`.

## License

Copyright (c) rivques <38469076+rivques@users.noreply.github.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
