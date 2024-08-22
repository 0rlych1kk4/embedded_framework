pub struct MockUart {
    buffer: Vec<u8>,
}

impl MockUart {
    pub fn new() -> Self {
        MockUart { buffer: Vec::new() }
    }
}

pub trait UartPeripheral {
    fn read(&self) -> u8;
    fn write(&mut self, byte: u8);
}

impl UartPeripheral for MockUart {
    fn read(&self) -> u8 {
        self.buffer.last().cloned().unwrap_or(0)
    }

    fn write(&mut self, byte: u8) {
        println!("Writing byte: {}", byte);
        self.buffer.push(byte);
    }
}

pub trait UartWrite {
    fn write_str(&mut self, s: &str);
}

impl UartWrite for MockUart {
    fn write_str(&mut self, s: &str) {
        println!("Writing string: {}", s);
        for byte in s.bytes() {
            self.write(byte);
        }
    }
}
