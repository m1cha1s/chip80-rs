

pub trait Memory {
    // Factory
    fn new() -> Self;

    // Basic I/O operations interface
    fn read(&mut self, address: u16) -> Option<u8>;
    fn write(&mut self, address: u16, value: u8);
    
    // Block operations interface
    fn mem_copy(&mut self, source: u16, lenght: u16, destination: u16);
    fn mem_move(&mut self, source: u16, lenght: u16, destination: u16);
}