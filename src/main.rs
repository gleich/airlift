mod display;

use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Baseline, Text};

fn main() {
	let mut disp = display::setup();

	let style = MonoTextStyleBuilder::new()
		.font(&FONT_6X10)
		.text_color(BinaryColor::On)
		.build();

	Text::with_baseline("Hello World!", Point::new(0, 10), style, Baseline::Top)
		.draw(&mut disp)
		.expect("Failed to draw text");

	disp.flush().unwrap();
	disp.clear();
}
