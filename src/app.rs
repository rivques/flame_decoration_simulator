use std::time::{Duration, Instant};

/// This is a simulator for my fire decoration PCB. It has a menu to choose a simulation technique and a simulation page to run the simulation.
/// The simulation page includes accurately laid out pixels simulation the `NeoPixels`.
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{canvas::{Canvas, Painter, Shape}, Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

use crate::{intro, types::{Simulation, LED}};

#[derive(Debug)]
enum AppPage {
    Intro,
    Menu(usize),
    Simulation(usize, Instant),
}

#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    /// Current page of the application.
    page: AppPage,

    simulations: Vec<Box<dyn Simulation>>,

    current_leds: Vec<LED>,

    current_intensity_mod: f32,
}

impl App {
    /// Construct a new instance of [`App`].
    #[must_use] pub fn new(simulations: Vec<Box<dyn Simulation>>, leds: Vec<LED>,) -> Self {
        Self {
            running: false,
            page: AppPage::Intro,
            simulations,
            current_leds: leds,
            current_intensity_mod: 1.0,
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            terminal.hide_cursor()?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&mut self, frame: &mut Frame) {
        // There's always a top bar with the title. It also has a status message that contains relevant key bindings.
        // Below the top bar, there's a main area that changes based on the current page.

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(frame.area());

        let top_bar = Block::default().borders(Borders::ALL);

        let title = Paragraph::new(Text::styled(
            "Fire Decoration Simulator",
            Style::default().fg(Color::Green),
        ))
        .alignment(Alignment::Center)
        .block(top_bar);

        frame.render_widget(title, chunks[0]);

        match self.page {
            AppPage::Intro => {
                let intro = Paragraph::new(Text::styled(
                    intro::TEXT,
                    Style::new().fg(Color::Green),
                ))
                .alignment(Alignment::Center);
                frame.render_widget(intro, chunks[1]);

                let instructions = Paragraph::new(
                    Text::styled(
                        "Press Enter to continue",
                        Style::new().fg(Color::Yellow),
                    )
                )
                .alignment(Alignment::Center);
                frame.render_widget(instructions, chunks[2]);
            }
            AppPage::Menu(simnum) => {
                let menu = Paragraph::new(Text::styled(
                    "Choose a simulation technique:",
                    Style::new().fg(Color::Green),
                ));
                frame.render_widget(menu, chunks[1]);

                let mut simulation_lines = vec![];

                for (i, simulation) in self.simulations.iter().enumerate() {
                    let simulation_name = simulation.get_name();
                    let simulation_line = Line::styled(
                        simulation_name,
                        Style::default().fg(if simnum == i {
                            Color::Yellow
                        } else {
                            Color::White
                        })
                        .bg(if simnum == i {
                            Color::Blue
                        } else {
                            Color::Reset
                        }),
                    );
                    simulation_lines.push(simulation_line);
                }
                let simulation_paragraph = Paragraph::new(simulation_lines).centered();
                frame.render_widget(simulation_paragraph, chunks[1]);

                // status message
                let status = Paragraph::new(
                    Line::raw("Navigate: ↑/↓, Select: Enter, Quit: Esc/q")
                        .style(Style::new().fg(Color::Yellow)),
                )
                .centered();
                frame.render_widget(status, chunks[2]);
            }
            AppPage::Simulation(simnum, start_time) => {
                let simulation_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Min(0),
                            Constraint::Length(2),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[1]);
                let simulation = &mut self.simulations[simnum];
                
                // tick the simulation
                simulation.tick(&mut self.current_leds, start_time.elapsed().as_micros().try_into().unwrap(), self.current_intensity_mod);

                // get bounding box of LEDs
                let mut min_x = i32::MAX;
                let mut max_x = i32::MIN;
                let mut min_y = i32::MAX;
                let mut max_y = i32::MIN;
                for led in &self.current_leds {
                    min_x = min_x.min(led.coords.0 as i32);
                    max_x = max_x.max(led.coords.0 as i32);
                    min_y = min_y.min(led.coords.1 as i32);
                    max_y = max_y.max(led.coords.1 as i32);
                }
                // add some padding
                min_x -= 3;
                max_x += 3;
                min_y -= 3;
                max_y += 3;

                let width = max_x - min_x;
                let height = max_y - min_y;
                let ideal_aspect_ratio = f64::from(width) / f64::from(height);

                // now, the canvas has a fixed aspect ratio, so we need to adjust the aspect ratio of the bounding box by adding padding
                // the canvas's size is (simulation_layout[0].width, simulation_layout[0].height * 2) because we have twice as much vertical resolution as horizontal
                let canvas_aspect_ratio = f64::from(simulation_layout[0].width) / f64::from(simulation_layout[0].height * 2);
                if canvas_aspect_ratio > ideal_aspect_ratio {
                    // canvas is wider than the bounding box, so we need to add padding to the left 
                    let new_width = (f64::from(height) * canvas_aspect_ratio) as i32;
                    let padding = (new_width - width) / 2;
                    min_x -= padding;
                    max_x += padding;
                } else {
                    // canvas is taller than the bounding box, so we need to add padding to the top
                    let new_height = (f64::from(width) / canvas_aspect_ratio) as i32;
                    let padding = (new_height - height) / 2;
                    min_y -= padding;
                    max_y += padding;
                }
            

                let canvas = Canvas::default()
                    .block(Block::default().borders(Borders::ALL).title("Simulation: ".to_owned() + simulation.get_name()))
                    .paint(|ctx| {
                        self.current_leds.iter().map(|led| {
                            let x = led.coords.0 as f64;
                            let y = led.coords.1 as f64;
                            let color = led.color;
                            FilledCircle{
                                x,
                                y,
                                radius: 2.0,
                                color: Color::Rgb(color.r, color.g, color.b),
                            }
                        })
                        .for_each(
                            |circle| ctx.draw(&circle)
                        );
                    })
                    .x_bounds([f64::from(min_x), f64::from(max_x)])
                    .y_bounds([f64::from(min_y), f64::from(max_y)]);
                frame.render_widget(canvas, simulation_layout[0]);
                // current intensity
                let intensity = Paragraph::new(
                    Line::raw("Intensity: ".to_owned() + &format!("{:.1}", self.current_intensity_mod))
                        .style(Style::new().fg(Color::Green)),
                )
                .centered();
                frame.render_widget(intensity, simulation_layout[1]);

                // status message
                let status = Paragraph::new(
                    Line::raw("Back to menu: Esc/q, Change intensity: ↑/↓")
                        .style(Style::new().fg(Color::Yellow)),
                )
                .centered();
                frame.render_widget(status, chunks[2]);
            }
        };
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(10)).is_ok_and(|ready| ready) {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q')) => match self.page {
                AppPage::Menu(_) => self.quit(),
                AppPage::Simulation(..) => self.page = AppPage::Menu(0),
                AppPage::Intro => self.quit(),
            },
            (KeyModifiers::CONTROL, KeyCode::Char('c' | 'C')) => self.quit(),
            (_, KeyCode::Up) => match self.page {
                AppPage::Menu(ref mut simnum) => {
                    if *simnum > 0 {
                        *simnum -= 1;
                    }
                }
                AppPage::Simulation(..) => {
                    self.current_intensity_mod += 0.1;
                    if self.current_intensity_mod > 1.0 {
                        self.current_intensity_mod = 1.0;
                    }
                }
                AppPage::Intro => {}
            },
            (_, KeyCode::Down) => match self.page {
                AppPage::Menu(ref mut simnum) => {
                    if *simnum < self.simulations.len() - 1 {
                        *simnum += 1;
                    }
                }
                AppPage::Simulation(..) => {
                    self.current_intensity_mod -= 0.1;
                    if self.current_intensity_mod < 0.0 {
                        self.current_intensity_mod = 0.0;
                    }
                }
                AppPage::Intro => {}
            },
            #[allow(clippy::single_match, reason = "the simulation page may care about Enter in the future")]
            (_, KeyCode::Enter) => match self.page {
                AppPage::Menu(simnum) => {
                    self.page = AppPage::Simulation(simnum, Instant::now());
                }
                AppPage::Intro => {
                    self.page = AppPage::Menu(0);
                }
                AppPage::Simulation(..) => {}
            },
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

/// A circle with a given center and radius and with a given color
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FilledCircle {
    /// `x` coordinate of the circle's center
    pub x: f64,
    /// `y` coordinate of the circle's center
    pub y: f64,
    /// Radius of the circle
    pub radius: f64,
    /// Color of the circle
    pub color: Color,
}

const RAD_MULT: f64 = 2.0;

impl Shape for FilledCircle {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        for angle in 0..360 {
            for dist in 0..=(self.radius * RAD_MULT) as i32 {
                let radians = f64::from(angle).to_radians();
                let circle_x = (f64::from(dist)/RAD_MULT).mul_add(radians.cos(), self.x);
                let circle_y = (f64::from(dist)/RAD_MULT).mul_add(radians.sin(), self.y);
                if let Some((x, y)) = painter.get_point(circle_x, circle_y) {
                    painter.paint(x, y, self.color);
                }
            }
        }
    }
}
