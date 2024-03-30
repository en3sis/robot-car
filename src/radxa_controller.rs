use linux_embedded_hal::sysfs_gpio::{Direction, Pin};
pub struct RadxaController {
    pin1: Pin,
    pin2: Pin,
    pin3: Pin,
    pin4: Pin,
}

impl RadxaController {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let pin1 = Pin::new(156);
        match pin1.export() {
            Ok(()) => println!("Gpio {} exported!", pin1.get_pin()),
            Err(err) => println!("Gpio {} could not be exported: {}", pin1.get_pin(), err),
        }

        let pin2 = Pin::new(157);
        match pin2.export() {
            Ok(()) => println!("Gpio {} exported!", pin2.get_pin()),
            Err(err) => println!("Gpio {} could not be exported: {}", pin2.get_pin(), err),
        }

        let pin3 = Pin::new(158);
        match pin3.export() {
            Ok(()) => println!("Gpio {} exported!", pin3.get_pin()),
            Err(err) => println!("Gpio {} could not be exported: {}", pin3.get_pin(), err),
        }

        let pin4 = Pin::new(159);
        match pin4.export() {
            Ok(()) => println!("Gpio {} exported!", pin4.get_pin()),
            Err(err) => println!("Gpio {} could not be exported: {}", pin4.get_pin(), err),
        }

        Ok(Self {
            pin1,
            pin2,
            pin3,
            pin4,
        })
    }
}
