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

#[derive(Copy, Clone)]
pub struct Stat<T> {
    id: u8,
    stat_enum: Option<StatsEnum>,
    value: Option<T>,
}

impl<T> Stat<T> {
    pub fn new(id_count: &mut u8) -> Self {
        let new = Self {
            id: *id_count,
            stat_enum: None,
            value: None,            
        };
        *id_count += 1;
        new
    }
}

#[derive(Clone, Copy)]
pub struct PlayerStats {
    strength: Stat<u8>,
    dexterity: Stat<u8>,
    constitution: Stat<u8>,
    intelligence: Stat<u8>,
    wisdom: Stat<u8>,
    charisma: Stat<u8>,
    hp: Stat<u8>,
    xp: Stat<u32>,
}

impl PlayerStats {
    pub fn new(id_counter: &mut u8) -> Self {
        Self {
            strength: Stat::new(id_counter),
            dexterity: Stat::new(id_counter),
            constitution: Stat::new(id_counter),
            intelligence: Stat::new(id_counter),
            wisdom: Stat::new(id_counter),
            charisma: Stat::new(id_counter),
            
            hp: Stat::new(id_counter),
            xp: Stat::new(id_counter),
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

impl Default for PlayerStats {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StatModifier {
    stat: StatsEnum,
    value_offset: i8,
}