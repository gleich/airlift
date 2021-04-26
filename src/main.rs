use embedded_graphics::fonts::{self, Text};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::style::TextStyleBuilder;
use linux_embedded_hal::I2cdev;
use ssd1306::mode::GraphicsMode;
use ssd1306::{Builder, I2CDIBuilder};
extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();
	ctrlc::set_handler(move || {
		r.store(false, Ordering::SeqCst);
	})
	.expect("Error setting Ctrl-C handler");

	let i2c = I2cdev::new("/dev/i2c-1").unwrap();

	let interface = I2CDIBuilder::new().init(i2c);
	let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();

	disp.init().unwrap();
	disp.flush().unwrap();

	let text_style = TextStyleBuilder::new(fonts::Font6x12)
		.text_color(BinaryColor::On)
		.background_color(BinaryColor::Off)
		.build();

	while running.load(Ordering::SeqCst) {
		Text::new("Hello World!", Point::new(0, 30))
			.into_styled(text_style)
			.into_iter()
			.draw(&mut disp)
			.expect("Failed to draw text");

		disp.flush().unwrap();
		disp.clear();
	}
	disp.clear();
	disp.flush().unwrap();
}
