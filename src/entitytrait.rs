use std::collections::HashMap;


pub struct AbstractTrait<'a, T> {
    key: &'a str,
    value: Box<T>,
}

impl<'a, T> AbstractTrait<'a, T> {

    pub fn new(key: &'a str, value: T) -> Box<Self> {
        Box::new(Self {
            key,
            value: Box::new(value),
        })
    }

    pub fn key(&self) -> &str {
        self.key
    }

    pub fn value(&self) -> &T {
        &self.value
    }

}

pub trait EntityStat { }

pub struct Player {
    stats: HashMap<&'static str, Box<dyn EntityStat>>,
}
