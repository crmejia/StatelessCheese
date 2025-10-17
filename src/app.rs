use crate::setup::App;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::Frame;
extern crate alloc;

/// Application state.
///
/// Here you can store any state you need for your application.
#[derive(Default)]
pub struct CheeseTestState {}

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
        render_temperature_widget(frame, first);
        render_humidity_widget(frame, second);
    }
}

fn render_title_widget(frame: &mut Frame, area: Rect) {
    let title = Line::from_iter([Span::from("Stateless").bold(), Span::from("Cheese")]).centered();
    frame.render_widget(title, area);
}
fn render_temperature_widget(frame: &mut Frame, area: Rect) {
    let text = "00°C";
    let title = "Temperature"; //Temperature is too long on tiny screen
    let block = Block::bordered().title(title);
    let paragraph = Paragraph::new(text)
        .style(Color::DarkGray)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(block);
    frame.render_widget(paragraph, area);
}

fn render_humidity_widget(frame: &mut Frame, area: Rect) {
    let text = "00%";
    let title = "Humidity";
    let block = Block::bordered().title(title);
    let paragraph = Paragraph::new(text)
        .style(Color::DarkGray)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(block);
    frame.render_widget(paragraph, area);
}
