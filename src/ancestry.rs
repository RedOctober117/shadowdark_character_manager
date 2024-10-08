use crate::{attributes::AttributeModifier, language::LanguageEnum};

pub struct Ancestry {
    name: String,
    description: String,
    languages: Vec<LanguageEnum>,
    modifiers: Vec<AttributeModifier>,
}

impl Ancestry {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            languages: vec![],
            modifiers: vec![],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn languages(&self) -> &Vec<LanguageEnum> {
        &self.languages
    }

    pub fn modifiers(&self) -> &Vec<AttributeModifier> {
        &self.modifiers
    }

    pub fn add_language(&mut self, language: LanguageEnum) {
        self.languages.push(language);
    }

    pub fn add_modifier(&mut self, modifier: AttributeModifier) {
        self.modifiers.push(modifier);
    }
}
