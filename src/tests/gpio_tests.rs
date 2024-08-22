#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpio_high_low() {
        let mut gpio_pin = MockGpioPin::new();

        assert!(gpio_pin.set_high().is_ok());
        assert!(gpio_pin.is_high().unwrap());

        assert!(gpio_pin.set_low().is_ok());
        assert!(gpio_pin.is_low().unwrap());
    }
}
