

pub trait Memory {
    // Factory
    fn new() -> Self;

    // Basic I/O operations interface
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
    
    // Block operations interface
    fn mem_copy(&mut self, source: u16, lenght: u16, destination: u16);
    fn mem_move(&mut self, source: u16, lenght: u16, destination: u16);
}

pub mod default {
    use crate::memory::Memory;

    pub struct Mem {
        pub memory: [u8; usize::pow(2, 16)], // 64KB of memory
        pub size: u16,
    }

    impl Memory for Mem {
        fn new() -> Mem {
            Mem {
                memory: [0; usize::pow(2, 16)],
                size: u16::pow(2, 16),
            }
        }

        fn read(&mut self, address: u16) -> u8 {
            self.memory[address as usize]
        }

        fn write(&mut self, address: u16, value: u8) {
            self.memory[address as usize] = value;
        }
        
        fn mem_copy(&mut self, source: u16, lenght: u16, destination: u16) {
            if source+lenght <= self.size && destination+lenght <= self.size {
                for i in 0..lenght {
                    self.memory[(destination+i) as usize] = self.memory[(source+i) as usize];
                }
            }
        }
        
        fn mem_move(&mut self, source: u16, lenght: u16, destination: u16) {
            if source+lenght <= self.size && destination+lenght <= self.size {
                for i in 0..lenght {
                    self.memory[(destination+i) as usize] = self.memory[(source+i) as usize];
                    self.memory[(source+i) as usize] = 0x00;
                }
            }
        }
    }
}