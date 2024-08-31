use core::{fmt, };

#[derive(Clone, Copy)]
pub enum StatsEnum {
    Strength = 0,
    Dexterity = 1,
    Constitution = 2,
    Intelligence = 3,
    Wisdom = 4,
    Charisma = 5,
}

impl fmt::Display for StatsEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StatsEnum::Strength => write!(f, "Strength"),
            StatsEnum::Dexterity => write!(f, "Dexterity"),
            StatsEnum::Constitution => write!(f, "Constitution"),
            StatsEnum::Intelligence => write!(f, "Intelligence"),
            StatsEnum::Wisdom => write!(f, "Wisdom"),
            StatsEnum::Charisma => write!(f, "Charisma"),
        }
    }
}

impl From<usize> for StatsEnum {
    fn from(value: usize) -> Self {
        match value {
            0 => StatsEnum::Strength,
            1 => StatsEnum::Dexterity,
            2 => StatsEnum::Constitution,
            3 => StatsEnum::Intelligence,
            4 => StatsEnum::Wisdom,
            5 => StatsEnum::Charisma,
            _ => todo!(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Stats {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            strength: 1,
            dexterity: 1,
            constitution: 1,
            intelligence: 1,
            wisdom: 1,
            charisma: 1,
        }
    }
    
    fn match_stat(self, stat_name: StatsEnum) -> u8 {
        match stat_name {
            StatsEnum::Strength => self.strength,
            StatsEnum::Dexterity => self.dexterity,
            StatsEnum::Constitution => self.constitution,
            StatsEnum::Intelligence => self.intelligence,
            StatsEnum::Wisdom => self.wisdom,
            StatsEnum::Charisma => self.charisma,
        }
    }
    
    fn match_stat_ref(&self, stat_name: StatsEnum) -> &u8 {
        match stat_name {
            StatsEnum::Strength => &self.strength,
            StatsEnum::Dexterity => &self.dexterity,
            StatsEnum::Constitution => &self.constitution,
            StatsEnum::Intelligence => &self.intelligence,
            StatsEnum::Wisdom => &self.wisdom,
            StatsEnum::Charisma => &self.charisma,
        }
    }

    fn match_stat_mut(&mut self, stat_name: StatsEnum) -> &mut u8 {
        match stat_name {
            StatsEnum::Strength => &mut self.strength,
            StatsEnum::Dexterity => &mut self.dexterity,
            StatsEnum::Constitution => &mut self.constitution,
            StatsEnum::Intelligence => &mut self.intelligence,
            StatsEnum::Wisdom => &mut self.wisdom,
            StatsEnum::Charisma => &mut self.charisma,
        }
    }

    pub fn get_stat(self, stat_name: StatsEnum) -> u8 {
        Self::match_stat(self, stat_name)
    }

    pub fn set_stat(&mut self, stat_name: StatsEnum, value: u8) {
        let original_value = Self::match_stat_mut(self, stat_name);
        *original_value = value;
    }
}

// impl IntoIterator for Stats {
//     type Item = i8;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         vec![
//             self.strength,
//             self.dexterity,
//             self.constitution,
//             self.intelligence,
//             self.wisdom,
//             self.charisma,
//         ]
//         .into_iter()
//     }
// }

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StatModifier {
    stat: StatsEnum,
    value_offset: i8,
}