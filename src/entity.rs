/*
   &T: reference
   T: copy of data if copy trait is implemented, otherwise value
   &mut T: mutable reference
*/

use std::{fmt, usize};

#[derive(Clone, Copy)]
pub struct Stats {
    strength: i8,
    dexterity: i8,
    constitution: i8,
    intelligence: i8,
    wisdom: i8,
    charisma: i8,
}

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

impl Stats {
    pub fn new() -> Self {
        Self {
            strength: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            wisdom: 0,
            charisma: 0,
        }
    }

    fn match_stat(self, stat_name: StatsEnum) -> i8 {
        match stat_name {
            StatsEnum::Strength => self.strength,
            StatsEnum::Dexterity => self.dexterity,
            StatsEnum::Constitution => self.constitution,
            StatsEnum::Intelligence => self.intelligence,
            StatsEnum::Wisdom => self.wisdom,
            StatsEnum::Charisma => self.charisma,
        }
    }

    fn match_stat_ref(&self, stat_name: StatsEnum) -> &i8 {
        match stat_name {
            StatsEnum::Strength => &self.strength,
            StatsEnum::Dexterity => &self.dexterity,
            StatsEnum::Constitution => &self.constitution,
            StatsEnum::Intelligence => &self.intelligence,
            StatsEnum::Wisdom => &self.wisdom,
            StatsEnum::Charisma => &self.charisma,
        }
    }

    fn match_stat_mut(&mut self, stat_name: StatsEnum) -> &mut i8 {
        match stat_name {
            StatsEnum::Strength => &mut self.strength,
            StatsEnum::Dexterity => &mut self.dexterity,
            StatsEnum::Constitution => &mut self.constitution,
            StatsEnum::Intelligence => &mut self.intelligence,
            StatsEnum::Wisdom => &mut self.wisdom,
            StatsEnum::Charisma => &mut self.charisma,
        }
    }

    pub fn get_stat(self, stat_name: StatsEnum) -> i8 {
        Self::match_stat(self, stat_name)
    }

    pub fn set_stat(&mut self, stat_name: StatsEnum, value: i8) {
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

struct Attack {}

struct Talent {}

struct Spell {}

struct Gear {}

enum AlignmentEnum {
    Chaotic,
    Lawful,
    Neutral,
}

enum Diety {}

enum ClassEnum {
    Fighter,
    Priest,
    Thief,
    Wizard,
}

struct Class {}

enum Background {
    Urchin,
    Wanted,
    CultInitiate,
    ThievesGuild,
    Banished,
    Orphaned,
    WizardsApprentice,
    Jeweler,
    Herbalist,
    Barbarian,
    Mercenary,
    Sailor,
    Acolyte,
    Soldier,
    Ranger,
    Scout,
    Minstrel,
    Scholar,
    Noble,
    Chirurgeon,
    Custom,
}

enum AncestryEnum {
    Dwarf,
    Elf,
    Goblin,
    Halfling,
    HalfOrc,
    Human,
}

struct Ancestry {}

struct Entity {
    name: String,
    ancestry: AncestryEnum,
    background: Background,
    stats: Stats,
    class: Class,
    level: i16,
    xp: f32,
    hp: i16,
    ac: i16,
    alignment: AlignmentEnum,
    diety: Diety,
    title: String,
    attacks: Vec<Attack>,
    talents: Vec<Talent>,
    spells: Vec<Spell>,
    gear: Vec<Gear>,
}

impl Entity {
    fn new() -> Self {
        todo!()
    }
}
