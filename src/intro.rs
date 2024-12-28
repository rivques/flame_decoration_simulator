pub const TEXT: &str = 
r#"Welcome to the Fire Decoration Simulator!

NOTE: It is HIGHLY recommended to run this program in Alacritty or another
terminal that supports full unicode and 24-bit color.

This project is designed to help me develop flame simulation techniques for my
#hacky-holidays fire decoration PCB. Each simulation available uses a different
technique to try and make the 12 simulated NeoPixels feel like a fire. Those
NeoPixels are sized and located as they will be on the PCB, so it's an accurate
simulation.

Available controls will be displayed at the bottom of the screen. Ctrl-C will
always exit the program.

Command-line arguments:
-h, --help: print this help message

To write your own simulation, see the README for instructions. It's available
at https://github.com/rivques/flame_decoration_simulator."#;