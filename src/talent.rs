
use crate::stats::StatModifier;

pub struct Talent {
    name: Option<String>,
    description: Option<String>,
    modifiers: Vec<StatModifier>,
}