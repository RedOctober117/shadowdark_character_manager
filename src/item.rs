pub trait AbstractItem {
    fn name(&self) -> &str;
    fn cost(&self) -> u16;
    fn slots(&self) -> u8;
}

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

#[derive(Clone, Copy)]
pub enum RangeEnum {}

#[derive(Clone, Copy, Debug)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

#[derive(Clone, Copy)]
pub struct ToRoll {
    die: Dice,
    times: u8,
}

impl ToRoll {
    pub fn new(die: Dice, times: u8) -> Self {
        Self { die, times }
    }

    pub fn get_roll(&self) -> String {
        format!("{:?}{:?}", self.times, self.die).to_lowercase()
    }
}

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

pub struct Weapon {
    name: String,
    cost: u16,
    slots: u8,

    range: RangeEnum,
    damage: Vec<ToRoll>,
    properties: Vec<WeaponProperty>,
}

impl Weapon {
    pub fn range(&self) -> RangeEnum {
        self.range
    }

    pub fn damage(&self) -> &Vec<ToRoll> {
        &self.damage
    }

    pub fn properties(&self) -> &Vec<WeaponProperty> {
        &self.properties
    }
}

impl AbstractItem for Weapon {
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
