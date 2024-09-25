use crate::{
    currency::Currency,
    dice::ToRoll,
    item::{AbstractItem, RangeEnum},
};

/// Represents a weapon in Shadowdark. Includes fields required to add the
/// `AbstractItem` trait. May need to be expanded to allow the addition or
/// removal of `WeaponProperty`s and `ToRoll`s at runtime.
#[derive(Clone, Debug)]
pub struct Weapon {
    name: String,
    cost: Currency,
    slots: u8,

    range: RangeEnum,
    damage: Vec<ToRoll>,
    properties: Vec<WeaponProperty>,
}

impl Weapon {
    pub fn new(
        name: String,
        cost: Currency,
        slots: u8,
        range: RangeEnum,
        damage: Vec<ToRoll>,
        properties: Vec<WeaponProperty>,
    ) -> Self {
        Self {
            name,
            cost,
            slots,
            range,
            damage,
            properties,
        }
    }

    pub fn range(&self) -> RangeEnum {
        self.range
    }

    /// Return a borrowed referenec to the vec of damage rolls. Some weapons
    /// may have more than one `ToRoll`, depending on if they're thrown or
    /// meleed, etc.
    pub fn damage(&self) -> &Vec<ToRoll> {
        &self.damage
    }

    /// Return a borrowed reference to a vec of `WeaponProperty`s.
    pub fn properties(&self) -> &Vec<WeaponProperty> {
        &self.properties
    }
}

impl AbstractItem for Weapon {
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

/// Represents a property of a weapon.
#[derive(Clone, Debug)]
pub struct WeaponProperty {
    name: String,
    description: String,
}

impl WeaponProperty {
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

pub enum WeaponTypeEnum {
    Club,
    Crossbow,
    Mace,
    LongSword,
}
