#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use cheese_test::setup::App;
use esp_hal::main;
use esp_println as _;

use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::Frame;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

/// Application state.
///
/// Here you can store any state you need for your application.
#[derive(Default)]
pub struct AppState {}

impl App for AppState {
    /// Draw the UI frame.
    ///
    /// This is being called in the main loop to render the UI.
    fn draw(&self, frame: &mut Frame) {
        let text = "Ratatui on embedded devices!";
        let paragraph = Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true });
        let bordered_block = Block::bordered()
            .border_style(Style::new().yellow())
            .title("Mousefood");
        frame.render_widget(paragraph.block(bordered_block), frame.area());
    }
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    loop {
        AppState::default().run();
    }
}
