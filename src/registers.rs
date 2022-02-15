

pub trait Registers {
    // Factory
    fn new() -> Self;

    // General purpose registers interface
    fn get_vx(&mut self, register_number: u8) -> u8;
    fn set_vx(&mut self, register_number: u8, value: u16);
    
    // I register interface
    fn get_i(&mut self) -> u16;
    fn set_i(&mut self, value: u16);

    // Special purpose register interfaces
    fn get_delay(&mut self) -> u16;
    fn set_delay(&mut self, value: u16);
    fn dec_delay(&mut self);
    
    fn get_sound(&mut self) -> u16;
    fn set_sound(&mut self, value: u16);
    fn dec_sound(&mut self);

    // Pseudo registers interface
    fn get_pc(&mut self) -> u16;
    fn set_pc(&mut self, value: u16);
    fn inc_pc(&mut self);
    fn dec_pc(&mut self);
    
    fn get_sp(&mut self) -> u16;
    fn set_sp(&mut self, value: u16);
    fn inc_sp(&mut self);
    fn dec_sp(&mut self);
}