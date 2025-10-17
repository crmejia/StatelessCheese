use alloc::boxed::Box;
use esp_hal::{
    clock::CpuClock,
    i2c::master::I2c,
    time::{Duration, Instant, Rate},
    Async,
};
use esp_println as _;

use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::{Frame, Terminal};
use ssd1306::{
    mode::{BufferedGraphicsMode, DisplayConfig},
    prelude::{DisplayRotation, I2CInterface},
    size::DisplaySize128x64,
    I2CDisplayInterface, Ssd1306,
};

extern crate alloc;

/// Application trait to be implemented by the user.
pub trait App {
    /// Draw the UI frame.
    fn draw(&self, frame: &mut Frame);

    /// Run the application
    fn run(self)
    where
        Self: Sized,
    {
        run_app(self);
    }
}

/// Run the application with the provided [`App`] implementation.
///
/// It initializes the hardware, sets up the display and buttons,
/// and enters the main event loop.
///
/// Please note that this function is blocking and will not return.
/// It is meant to be called once at the start of the program (e.g., in `main`).
///
/// Errors are not handled and will cause a panic if they occur.
fn run_app(mut app: impl App) -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 72 * 1024);

    //set up OLED
    let i2c_bus = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_scl(peripherals.GPIO18)
    .with_sda(peripherals.GPIO23)
    .into_async();

    let interface = I2CDisplayInterface::new(i2c_bus);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // setup mousefood
    let embedded_backend_config = EmbeddedBackendConfig {
        flush_callback: Box::new(
            move |display: &mut Ssd1306<
                I2CInterface<I2c<'_, Async>>,
                DisplaySize128x64,
                BufferedGraphicsMode<DisplaySize128x64>,
            >| {
                display.flush().unwrap();
            },
        ),
        ..Default::default()
    };

    let backend = EmbeddedBackend::new(&mut display, embedded_backend_config);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}

        terminal
            .draw(|f| {
                app.draw(f);
            })
            .unwrap();
    }
}
