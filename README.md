# Embedded Framework

## Overview

The `embedded_framework` is a Rust-based library designed to simplify common tasks in embedded systems development, such as GPIO and UART control, while leveraging Rust’s safety and performance features.

## Features

- **GPIO Control**: Manage GPIO pins with safety and performance.
- **UART Communication**: Simplify UART communication for embedded systems.
- **Mock Implementations**: Includes mock implementations for testing.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
embedded_framework = "0.1.0"

## Usage

use embedded_framework::gpio::{MockGpioPin, OutputPin, InputPin};

fn main() {
    let mut gpio_pin = MockGpioPin::new();
    gpio_pin.set_high().expect("Failed to set GPIO high");
    if gpio_pin.is_high().expect("Failed to read GPIO state") {
        println!("GPIO is high");
    }
    gpio_pin.set_low().expect("Failed to set GPIO low");
}

License

This project is licensed under the MIT License - see the LICENSE file for details.



