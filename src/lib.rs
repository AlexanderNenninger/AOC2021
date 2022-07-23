mod day08;
mod day11;
mod day12;
mod day14;
mod day15;
mod day16;
mod day17;

pub trait ReadStr: Sized {
    type Err;
    fn read_str(s: &str) -> Result<Self, Self::Err>;
}
