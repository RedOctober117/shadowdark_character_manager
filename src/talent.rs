use crate::AttributeModifier;

/// Represents a Talent. This will eventually be abstracted to include weapon
/// and armour properties as well.
#[derive(Clone, Debug)]
pub struct Talent {
    // name: String,
    description: String,
    modifiers: Option<Vec<AttributeModifier>>,
}

impl Talent {
    pub fn new(description: String) -> Self {
        Self {
            description,
            modifiers: None,
        }
    }
    pub fn description(&self) -> &str {
        &self.description
    }

    /// If the `Talent` has modifiers, return a vec of `AttributeModifier`s.
    /// Else, `None`.``
    pub fn modifiers(&self) -> Option<&Vec<AttributeModifier>> {
        match &self.modifiers {
            Some(e) => Some(e),
            None => None,
        }
    }

    pub fn add_modifier(&mut self, modifier: AttributeModifier) {
        match &mut self.modifiers {
            None => {
                self.modifiers = Some(vec![modifier]);
            }
            Some(m) => m.push(modifier),
        }
    }
}
