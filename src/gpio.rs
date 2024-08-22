// src/gpio.rs

#[derive(Debug)]
pub enum GpioError {
    // Define your error variants here if needed
    GenericError,
}

pub trait OutputPin {
    fn set_high(&mut self) -> Result<(), GpioError>;
    fn set_low(&mut self) -> Result<(), GpioError>;
}

pub struct MockGpioPin {
    state: bool,
}

impl MockGpioPin {
    pub fn new() -> Self {
        MockGpioPin { state: false }
    }
}

impl OutputPin for MockGpioPin {
    fn set_high(&mut self) -> Result<(), GpioError> {
        println!("Setting GPIO high");
        self.state = true;
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), GpioError> {
        println!("Setting GPIO low");
        self.state = false;
        Ok(())
    }
}

pub trait InputPin {
    fn is_high(&self) -> Result<bool, GpioError>;
    fn is_low(&self) -> Result<bool, GpioError>;
}

impl InputPin for MockGpioPin {
    fn is_high(&self) -> Result<bool, GpioError> {
        Ok(self.state)
    }

    fn is_low(&self) -> Result<bool, GpioError> {
        Ok(!self.state)
    }
}
