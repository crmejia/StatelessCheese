use crate::setup::App;

use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::Frame;

use core::fmt::Write;
use defmt::error;
use heapless::String;
extern crate alloc;

/// Application state.
///
/// Here you can store any state you need for your application.
#[derive(Default)]
pub struct CheeseTestState {
    temperature: u8,
    humidity: u8,
}

pub fn run() {
    CheeseTestState::default().run();
}

impl App for CheeseTestState {
    /// Draw the UI frame.
    ///
    /// This is being called in the main loop to render the UI.
    fn draw(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        let horizontal = Layout::horizontal([Constraint::Percentage(50); 2]).spacing(1);
        let [top, main] = frame.area().layout(&vertical);
        let [first, second] = main.layout(&horizontal);

        render_title_widget(frame, top);
        render_temperature_widget(frame, first, self.temperature);
        render_humidity_widget(frame, second, self.humidity);
    }

    fn read_sensor(
        &mut self,
        dht11: &mut embedded_dht_rs::dht11::Dht11<esp_hal::gpio::Flex<'_>, esp_hal::delay::Delay>,
    ) {
        match dht11.read() {
            Ok(sensor_reading) => {
                self.humidity = sensor_reading.humidity;
                self.temperature = sensor_reading.temperature;
            }
            Err(error) => error!("An error occured: {}", defmt::Debug2Format(&error)),
        }
    }
}

fn render_title_widget(frame: &mut Frame, area: Rect) {
    let title = Line::from_iter([Span::from("Stateless").bold(), Span::from("Cheese")]).centered();
    frame.render_widget(title, area);
}
fn render_temperature_widget(frame: &mut Frame, area: Rect, temperature: u8) {
    let mut text: String<8> = String::new();
    write!(text, "{temperature}°C").unwrap();
    let title = "Temperature"; //Temperature is too long on tiny screen
    let block = Block::bordered().title(title);
    let paragraph = Paragraph::new(text.as_str())
        .style(Color::DarkGray)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(block);
    frame.render_widget(paragraph, area);
}

fn render_humidity_widget(frame: &mut Frame, area: Rect, humidity: u8) {
    let mut text: String<8> = String::new();
    write!(text, "{humidity}%").unwrap();
    let title = "Humidity";
    let block = Block::bordered().title(title);
    let paragraph = Paragraph::new(text.as_str())
        .style(Color::DarkGray)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(block);
    frame.render_widget(paragraph, area);
}
