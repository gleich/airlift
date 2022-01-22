use linux_embedded_hal::I2cdev;
use ssd1306::mode::{BufferedGraphicsMode, DisplayConfig};
use ssd1306::prelude::{DisplaySize128x32, I2CInterface};
use ssd1306::rotation::DisplayRotation;
use ssd1306::{I2CDisplayInterface, Ssd1306};

pub fn setup(
) -> Ssd1306<I2CInterface<I2cdev>, DisplaySize128x32, BufferedGraphicsMode<DisplaySize128x32>> {
	let i2c = I2cdev::new("/dev/i2c-1").unwrap();
	let interface = I2CDisplayInterface::new(i2c);
	let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
		.into_buffered_graphics_mode();

	display.init().unwrap();
	display.flush().unwrap();

	display
}
