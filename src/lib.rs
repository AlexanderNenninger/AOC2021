mod day07;

pub trait ReadStr: Sized {
    type Err;
    fn read_str(s: &str) -> Result<Self, Self::Err>;
}
