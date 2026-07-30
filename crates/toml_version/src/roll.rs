pub struct Roll<T: Number> {
    roll: T,
    d: Die,
}

pub enum Die {
    D2,
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

pub trait Number: Sized {}

impl Number for i8 {}
