// examples/gpio_example.rs
use embedded_framework::gpio::{MockGpioPin, OutputPin, InputPin};

fn main() {
    let mut gpio_pin = MockGpioPin::new();

    gpio_pin.set_high().expect("Failed to set GPIO high");
    if gpio_pin.is_high().expect("Failed to read GPIO state") {
        println!("GPIO pin is high");
    }

    gpio_pin.set_low().expect("Failed to set GPIO low");
    if gpio_pin.is_low().expect("Failed to read GPIO state") {
        println!("GPIO pin is low");
    }
}
