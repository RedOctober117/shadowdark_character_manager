use crate::currency::Currency;

/// Represents a generic item. All items in Shadowdark have a name, cost, and
/// slot size, including weapons and armour.
pub trait AbstractItem {
    fn name(&self) -> &str;
    fn cost(&self) -> Currency;
    fn slots(&self) -> u8;
}

#[derive(Clone, Debug)]
/// Represents a concrete item, like a candle or rope.
pub struct Item {
    name: String,
    cost: Currency,
    slots: u8,
}

impl Item {
    pub fn new(name: String, cost: Currency, slots: u8) -> Self {
        Self { name, cost, slots }
    }
}

impl AbstractItem for Item {
    fn name(&self) -> &str {
        &self.name
    }

    fn cost(&self) -> Currency {
        self.cost
    }

    fn slots(&self) -> u8 {
        self.slots
    }
}

/// Enumerates the ranges in Shadowdark.
#[derive(Clone, Copy, Debug)]
pub enum RangeEnum {
    Close,
    Near,
    Far,
}
