// use core::fmt;

use std::collections::HashMap;

pub trait EntityTrait {
    // fn new(key: &'static str) -> Self where Self: Sized;
    fn key(&self) -> &'static str;
    // fn new(key: &'static str, value: T) -> Self;
    // fn modify_value(&mut self, value: T);
    // fn value(self) -> T;
}

pub struct EntityStat<T> {
    key: &'static str,
    value: Box<T>,
}

impl<T> EntityStat<T> {
    pub fn new(key: &'static str, value: T) -> Self {
        Self {
            key,
            value: Box::new(value),
        }
    }

    fn modify_value(&mut self, value: T) {
        self.value = Box::new(value);
    }
    
    fn value(self) -> T {
        *self.value
    }
}

impl<T> EntityTrait for EntityStat<T> {

    fn key(&self) -> &'static str {
        self.key
    }

}

// pub struct EntityStatBlock<StatBlockItem> {
//     key: &'static str,
//     stat_map: HashMap<&'static str, StatBlockItem>,
// }

// impl<StatBlockItem> EntityTrait<StatBlockItem> for EntityStatBlock<StatBlockItem> {
//     fn key(&self) -> &'static str {
//         self.key
//     }

//     fn modify_value(&mut self, _value: T) {
//         unimplemented!();
//     }

//     fn value(self) -> T {
//         self.stat_map
//     }
// }

// impl<T> EntityTrait<T> {
//     pub fn new(key: String, value: T) -> Self {
//         Self { key, value }
//     }

//     pub fn key(&self) -> &String {
//         &self.key
//     }

//     pub fn modify_value(&mut self, value: T) {
//         self.value = value;
//     }

//     pub fn value(self) -> T {
//         self.value
//     }
// }

// pub struct EntityStatBlock {
//     key: String,
//     stat_map: HashMap<String, EntityTrait<u8>>,
//     // strength: EntityTrait<u8>,
//     // dexterity: EntityTrait<u8>,
//     // constitution: EntityTrait<u8>,
//     // intelligence: EntityTrait<u8>,
//     // wisdom: EntityTrait<u8>,
//     // charisma: EntityTrait<u8>,
// }

// impl EntityStatBlock {
//     pub fn new() -> Self {
//         let mut map = HashMap::new();
//         let str_ = EntityTrait::new(String::from("strength"), 1_u8);
//         let dex = EntityTrait::new(String::from("dexterity"), 1_u8);
//         let con = EntityTrait::new(String::from("constitution"), 1_u8);
//         let int = EntityTrait::new(String::from("intelligence"), 1_u8);
//         let wis = EntityTrait::new(String::from("wisdom"), 1_u8);
//         let cha = EntityTrait::new(String::from("charisma"), 1_u8);

//         map.insert(str_.key().to_owned(), str_);
//         map.insert(dex.key().to_owned(), dex);
//         map.insert(con.key().to_owned(), con);
//         map.insert(int.key().to_owned(), int);
//         map.insert(wis.key().to_owned(), wis);
//         map.insert(cha.key().to_owned(), cha);

//         Self {
//             key: String::from("stat_block"),
//             stat_map: map,
//         }
//     }
// }

// impl Default for EntityStatBlock {
//     fn default() -> Self {
//         Self::new()
//     }
// }

pub struct EntityTraitMap {
    trait_map: HashMap<&'static str, Box<dyn EntityTrait>>,
}

impl EntityTraitMap {
    pub fn new() -> Self {
        let mut trait_map: HashMap<&'static str, Box<dyn EntityTrait>> = HashMap::new();

        let str_ = EntityStat::new("strength", 1);
        let dex  = EntityStat::new("dexterity", 1);
        let con  = EntityStat::new("constitution", 1);
        let int  = EntityStat::new("intelligence", 1);
        let wis  = EntityStat::new("wisdom", 1);
        let cha  = EntityStat::new("charisma", 1);

        trait_map.insert(str_.key(), Box::new(str_));
        trait_map.insert(dex.key(), Box::new(dex));
        trait_map.insert(con.key(), Box::new(con));
        trait_map.insert(int.key(), Box::new(int));
        trait_map.insert(wis.key(), Box::new(wis));
        trait_map.insert(cha.key(), Box::new(cha));

        Self {
            trait_map,
        }
    }
}

// pub struct Entity<T> {
//     traits: HashMap<String, Box<dyn EntityTrait<T>>>
// }

// impl<T> Entity<T> {
//     fn new() -> Self {
//         let mut traits = HashMap::new();
//         let trait_1 = EntityStat::new("test", 1);
//         let trait_2 = EntityStat::new("test_2", "a");
        
//         traits.insert(trait_1.key(), trait_1);
//         traits.insert(trait_2.key(), trait_2);
//     }
// }

// A modifier to a trait. The `trait_identifier` is used to acess the entity's
// map of traits. `value` should be an offset value for a given EntityTrait. In the
// case of a real/int value, it should be signed, as all `TraitModifier`s are
// summed up before reporting final value to user.
// #[derive(Clone, Debug)]
// pub struct EntityTraitModifier<T> {
//     value: T,
// }

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
//     strength: EntityTrait<u8>,
//     dexterity: EntityTrait<u8>,
//     constitution: EntityTrait<u8>,
//     intelligence: EntityTrait<u8>,
//     wisdom: EntityTrait<u8>,
//     charisma: EntityTrait<u8>,
// }

// impl PlayerStats {
//     pub fn new(id_counter: &mut u8) -> Self {
//         Self {
//             strength: EntityTrait::new(1, id_counter),
//             dexterity: EntityTrait::new(1, id_counter),
//             constitution: EntityTrait::new(1, id_counter),
//             intelligence: EntityTrait::new(1, id_counter),
//             wisdom: EntityTrait::new(1, id_counter),
//             charisma: EntityTrait::new(1, id_counter),
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
