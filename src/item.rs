/// Represents a generic item. All items in Shadowdark have a name, cost, and
/// slot size, including weapons and armour.
pub trait AbstractItem {
    fn name(&self) -> &str;
    fn cost(&self) -> u16;
    fn slots(&self) -> u8;
}

/// Represents a concrete item, like a candle or rope.
pub struct Item {
    name: String,
    cost: u16,
    slots: u8,
}

impl AbstractItem for Item {
    fn name(&self) -> &str {
        &self.name
    }

    fn cost(&self) -> u16 {
        self.cost
    }

    fn slots(&self) -> u8 {
        self.slots
    }
}

/// Enumerates the ranges in Shadowdark.
#[derive(Clone, Copy)]
pub enum RangeEnum {
    Close,
    Near,
    Far,
}
