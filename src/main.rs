mod display;

use embedded_graphics::fonts::{self, Text};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::style::TextStyleBuilder;

fn main() {
	let mut disp = display::setup();

	let text_style = TextStyleBuilder::new(fonts::Font6x12)
		.text_color(BinaryColor::On)
		.background_color(BinaryColor::Off)
		.build();

	Text::new("Hello World!", Point::new(0, 30))
		.into_styled(text_style)
		.into_iter()
		.draw(&mut disp)
		.expect("Failed to draw text");

	disp.flush().unwrap();
	disp.clear();
}
