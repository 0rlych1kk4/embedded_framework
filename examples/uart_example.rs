use embedded_framework::uart::{MockUart, UartPeripheral, UartWrite};

fn main() {
    let mut uart = MockUart::new();  // Add `mut` here

    uart.write(42);
    uart.write_str("Hello, UART!");

    let received = uart.read();
    println!("Received: {}", received);
}


// Build and Run Examples
// cargo build --examples
// cargo run --example gpio_example
// cargo run --example uart_example