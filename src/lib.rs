mod day08;
mod day11;
mod day14;
pub mod runlenght;

pub trait ReadStr: Sized {
    type Err;
    fn read_str(s: &str) -> Result<Self, Self::Err>;
}
