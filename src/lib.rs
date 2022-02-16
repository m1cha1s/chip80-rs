pub mod prelude;

pub mod memory;
pub mod registers;
pub mod keyboard;
pub mod display;
pub mod sound;
pub mod delay;
pub mod cpu;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}