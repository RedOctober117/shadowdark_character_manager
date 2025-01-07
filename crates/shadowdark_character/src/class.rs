use crate::{
    armour::ArmourTypeEnum, dice::ToRoll, language::LanguageEnum, talent::Talent,
    weapon::WeaponTypeEnum,
};

pub struct Class {
    name: String,
    description: String,

    hit_die: ToRoll,

    available_weapons: Vec<WeaponTypeEnum>,
    available_armour: Vec<ArmourTypeEnum>,

    available_languages: Vec<LanguageEnum>,
    available_talents: Vec<Talent>,
}

impl Class {
    pub fn new(
        name: String,
        description: String,
        hit_die: ToRoll,
        available_weapons: Vec<WeaponTypeEnum>,
        available_armour: Vec<ArmourTypeEnum>,
        available_languages: Vec<LanguageEnum>,
        available_talents: Vec<Talent>,
    ) -> Self {
        Self {
            name,
            description,
            hit_die,
            available_weapons,
            available_armour,
            available_languages,
            available_talents,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&mut self) -> &str {
        &self.description
    }

    pub fn hit_die(&self) -> ToRoll {
        self.hit_die
    }

    pub fn available_weapons(&self) -> &Vec<WeaponTypeEnum> {
        &self.available_weapons
    }

    pub fn available_armour(&self) -> &Vec<ArmourTypeEnum> {
        &self.available_armour
    }

    pub fn available_languages(&self) -> &Vec<LanguageEnum> {
        &self.available_languages
    }

    pub fn available_talents(&self) -> &Vec<Talent> {
        &self.available_talents
    }
}
