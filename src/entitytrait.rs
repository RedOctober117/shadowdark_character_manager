use std::{borrow::Borrow, collections::HashMap};

pub struct Player {
    traits: HashMap<String, Box<Retrievable>>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            traits: HashMap::new(),
        }
    }

    pub fn add_trait(&mut self, key: String, new_trait: Retrievable) {
        let boxed_trait = Box::from(new_trait);
        self.traits.insert(key, boxed_trait);
    }

    pub fn get_trait(&self, key: String) -> Option<&Retrievable> {
        self.traits.get(&key).map(|v| &**v)
    }
}

// pub trait EntityTrait {
//     fn retrievable(&self) -> impl Fn();
// }

pub trait Retrievable {
    fn get(&self) -> impl Fn();
}

pub struct EntityStat<T> {
    name: String,
    value: Box<T>,
}

impl<T> EntityStat<T> {
    pub fn new(name: String, value: T) -> Self {
        Self {
            name,
            value: Box::from(value),
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }
}

impl<T: Fn()> Retrievable for EntityStat<T> {
    fn get(&self) -> impl Fn() {
        self.get()
    }
}