use linux_embedded_hal::I2cdev;
use ssd1306::mode::GraphicsMode;
use ssd1306::prelude::I2CInterface;
use ssd1306::{Builder, I2CDIBuilder};

pub fn setup() -> GraphicsMode<I2CInterface<I2cdev>> {
	let i2c = I2cdev::new("/dev/i2c-1").unwrap();
	let interface = I2CDIBuilder::new().init(i2c);
	let mut disp: GraphicsMode<I2CInterface<I2cdev>> = Builder::new().connect(interface).into();

	disp.init().unwrap();
	disp.flush().unwrap();

	disp
}
