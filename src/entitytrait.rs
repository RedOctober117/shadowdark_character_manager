use std::{collections::HashMap, sync::Arc};

// pub struct AbstractTrait<'a, T> {
//     key: &'a str,
//     value: Box<T>,
// }

// impl<'a, T> AbstractTrait<'a, T> {

//     pub fn new(key: &'a str, value: T) -> Box<Self> {
//         Box::new(Self {
//             key,
//             value: Box::new(value),
//         })
//     }

//     pub fn key(&self) -> &str {
//         self.key
//     }

//     pub fn value(&self) -> &T {
//         &self.value
//     }

// }

pub trait AbstractTrait {
    // fn new(&self, key: String) -> Self;
    fn key(&self) -> &str;
}

pub struct PlayerTrait<T> {
    key: String,
    value: Option<T>,
}

impl<T> PlayerTrait<T> {
    pub fn new(&self, key: String) -> Arc<Self> {
        Arc::from(Self {
            key,
            value: None,
        })
    }

    pub fn value(&self) -> &Option<T> {
        &self.value
    }
}

impl<T> AbstractTrait for PlayerTrait<T> {
    fn key(&self) -> &str {
        &self.key
    }
}

pub struct Player<'a> {
    stats: HashMap<&'a str, &'a dyn AbstractTrait>,
}

impl<'a> Player<'a> {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn add_trait(&mut self, new_trait: &'a dyn AbstractTrait) {
        self.stats.insert(new_trait.key(), new_trait);
    }
}
