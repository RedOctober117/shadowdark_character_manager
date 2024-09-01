// use core::fmt;

#[derive(Clone, Debug)]
pub struct Trait<T, F> {
    identifier: String,
    description: Option<String>,
    value: T,
    modifiers: Vec<TraitModifier<F>>,
}

impl<T, F> Trait<T, F> {
    pub fn new(value: T, identifier: String) -> Self {
        Self {
            identifier: identifier.to_owned(),
            description: None,
            value,
            modifiers: vec![],
        }
    }

    pub fn get_value(self) -> T {
        self.value
    }

    pub fn get_value_ref(&self) -> &T {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn add_modifier(&mut self, value: F) {
        self.modifiers.push(TraitModifier {
            value
        });
    }
}

/// A modifier to a trait. The `trait_identifier` is used to acess the entity's
/// map of traits. `value` should be an offset value for a given Trait. In the
/// case of a real/int value, it should be signed, as all `TraitModifier`s are
/// summed up before reporting final value to user.
#[derive(Clone, Debug)]
pub struct TraitModifier<T> {
    // trait_identifier: String,
    value: T,
}


// #[derive(Clone, Copy)]
// pub enum StatsEnum {
//     Strength = 0,
//     Dexterity = 1,
//     Constitution = 2,
//     Intelligence = 3,
//     Wisdom = 4,
//     Charisma = 5,
// }

// impl fmt::Display for StatsEnum {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             StatsEnum::Strength => write!(f, "Strength"),
//             StatsEnum::Dexterity => write!(f, "Dexterity"),
//             StatsEnum::Constitution => write!(f, "Constitution"),
//             StatsEnum::Intelligence => write!(f, "Intelligence"),
//             StatsEnum::Wisdom => write!(f, "Wisdom"),
//             StatsEnum::Charisma => write!(f, "Charisma"),
//         }
//     }
// }

// impl From<usize> for StatsEnum {
//     fn from(value: usize) -> Self {
//         match value {
//             0 => StatsEnum::Strength,
//             1 => StatsEnum::Dexterity,
//             2 => StatsEnum::Constitution,
//             3 => StatsEnum::Intelligence,
//             4 => StatsEnum::Wisdom,
//             5 => StatsEnum::Charisma,
//             _ => todo!(),
//         }
//     }
// }

// // #[derive(Copy, Clone)]
// // pub struct Stat<T> {
// //     id: u8,
// //     stat_enum: Option<StatsEnum>,
// //     value: Option<T>,
// // }

// // impl<T> Stat<T> {
// //     pub fn new(id_count: &mut u8) -> Self {
// //         let new = Self {
// //             id: *id_count,
// //             stat_enum: None,
// //             value: None,
// //         };
// //         *id_count += 1;
// //         new
// //     }
// // }

// #[derive(Clone)]
// pub struct PlayerStats {
//     strength: Trait<u8>,
//     dexterity: Trait<u8>,
//     constitution: Trait<u8>,
//     intelligence: Trait<u8>,
//     wisdom: Trait<u8>,
//     charisma: Trait<u8>,
// }

// impl PlayerStats {
//     pub fn new(id_counter: &mut u8) -> Self {
//         Self {
//             strength: Trait::new(1, id_counter),
//             dexterity: Trait::new(1, id_counter),
//             constitution: Trait::new(1, id_counter),
//             intelligence: Trait::new(1, id_counter),
//             wisdom: Trait::new(1, id_counter),
//             charisma: Trait::new(1, id_counter),
//         }
//     }

//     fn match_stat(self, stat_name: StatsEnum) -> u8 {
//         match stat_name {
//             StatsEnum::Strength => self.strength.get_value(),
//             StatsEnum::Dexterity => self.dexterity.get_value(),
//             StatsEnum::Constitution => self.constitution.get_value(),
//             StatsEnum::Intelligence => self.intelligence.get_value(),
//             StatsEnum::Wisdom => self.wisdom.get_value(),
//             StatsEnum::Charisma => self.charisma.get_value(),
//         }
//     }

//     fn match_stat_ref(&self, stat_name: StatsEnum) -> &u8 {
//         match stat_name {
//             StatsEnum::Strength => &self.strength.get_value_ref(),
//             StatsEnum::Dexterity => &self.dexterity.get_value_ref(),
//             StatsEnum::Constitution => &self.constitution.get_value_ref(),
//             StatsEnum::Intelligence => &self.intelligence.get_value_ref(),
//             StatsEnum::Wisdom => &self.wisdom.get_value_ref(),
//             StatsEnum::Charisma => &self.charisma.get_value_ref(),
//         }
//     }

//     fn match_stat_mut(&mut self, stat_name: StatsEnum) -> &mut u8 {
//         match stat_name {
//             StatsEnum::Strength => self.strength.get_value_mut(),
//             StatsEnum::Dexterity => self.dexterity.get_value_mut(),
//             StatsEnum::Constitution => self.constitution.get_value_mut(),
//             StatsEnum::Intelligence => self.intelligence.get_value_mut(),
//             StatsEnum::Wisdom => self.wisdom.get_value_mut(),
//             StatsEnum::Charisma => self.charisma.get_value_mut(),
//         }
//     }

//     pub fn get_stat(self, stat_name: StatsEnum) -> u8 {
//         Self::match_stat(self, stat_name)
//     }

//     pub fn set_stat(&mut self, stat_name: StatsEnum, value: u8) {
//         let original_value = Self::match_stat_mut(self, stat_name);
//         *original_value = value;
//     }
// }

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

// impl Default for PlayerStats {
//     fn default() -> Self {
//         todo!()
//         // Self::new()
//     }
// }

// pub struct StatModifier {
//     stat: StatsEnum,
//     value_offset: i8,
// }
