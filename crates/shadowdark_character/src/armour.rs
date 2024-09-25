use std::vec;

use crate::{currency::Currency, item::AbstractItem};

/// Represents a piece of armour in Shadowdark.
#[derive(Clone, Debug)]
pub struct Armour {
    name: String,
    cost: Currency,
    slots: u8,

    ac: u8,
    properties: Vec<ArmourProperty>,
}

impl Armour {
    pub fn new(name: String, cost: Currency, slots: u8, ac: u8) -> Self {
        Self {
            name,
            cost,
            slots,

            ac,
            properties: vec![],
        }
    }

    pub fn ac(&self) -> u8 {
        self.ac
    }

    /// Property may change total AC. This behavior will need to either be
    /// handled or just displayed clearly to the user. This could be handled by
    /// putting AC into the attribute matrix and putting AttributeModifiers
    /// inside items/item properties where necessary. This makes the most sense considering AC
    /// is 10 + dex modifier.
    pub fn properties(&self) -> &Vec<ArmourProperty> {
        &self.properties
    }
}

impl AbstractItem for Armour {
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

#[derive(Clone, Debug)]
pub struct ArmourProperty {
    name: String,
    description: String,
}

impl ArmourProperty {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

pub enum ArmourTypeEnum {
    Leather,
    Chainmail,
}
