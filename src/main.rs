use attributes::{Attrbiute, AttributeModifier, Attributes};

pub mod attributes;
pub mod item;

pub fn main() {
    let mut test_attributes = Attributes::new();
    let mut modifier_1 = AttributeModifier::new(Attrbiute::Strength, 1);
    // let mut modifier_2 = AttributeModifier::new(Attrbiute::Strength, -5);
    modifier_1.set_entry_index(test_attributes.add_attribute_modifier(&modifier_1));
    // modifier_2.set_entry_index(test_attributes.add_attrbute_modifier(&modifier_2));

    println!("{}", test_attributes.get_attribute(Attrbiute::Strength));

    test_attributes.remove_attribute_modifier(&modifier_1);
    println!("{}", test_attributes.get_attribute(Attrbiute::Strength));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_attributes() {
        let mut test_matrix = Attributes::new();
        let modifiers = vec![
            AttributeModifier::new(Attrbiute::Strength, 1),
            AttributeModifier::new(Attrbiute::Dexterity, 1),
            AttributeModifier::new(Attrbiute::Charisma, 1),
            AttributeModifier::new(Attrbiute::Intelligence, 1),
            AttributeModifier::new(Attrbiute::Wisdom, 1),
            AttributeModifier::new(Attrbiute::Constitution, 1),
        ];

        for m in modifiers {
            test_matrix.add_attribute_modifier(&m);
        }

        assert_eq!(test_matrix.get_attribute(Attrbiute::Strength), 1);
        assert_eq!(test_matrix.get_attribute(Attrbiute::Dexterity), 1);
        assert_eq!(test_matrix.get_attribute(Attrbiute::Charisma), 1);
        assert_eq!(test_matrix.get_attribute(Attrbiute::Intelligence), 1);
        assert_eq!(test_matrix.get_attribute(Attrbiute::Wisdom), 1);
        assert_eq!(test_matrix.get_attribute(Attrbiute::Constitution), 1);
    }

    #[test]
    fn test_remove_attributes() {
        let mut test_matrix = Attributes::new();
        let mut modifiers = vec![
            AttributeModifier::new(Attrbiute::Strength, 1),
            AttributeModifier::new(Attrbiute::Dexterity, 1),
            AttributeModifier::new(Attrbiute::Charisma, 1),
            AttributeModifier::new(Attrbiute::Intelligence, 1),
            AttributeModifier::new(Attrbiute::Wisdom, 1),
            AttributeModifier::new(Attrbiute::Constitution, 1),
        ];

        for m in &mut modifiers {
            m.set_entry_index(test_matrix.add_attribute_modifier(m));
        }

        for m in modifiers {
            test_matrix.remove_attribute_modifier(&m);
        }

        assert_eq!(test_matrix.get_attribute(Attrbiute::Strength), 0);
        assert_eq!(test_matrix.get_attribute(Attrbiute::Dexterity), 0);
        assert_eq!(test_matrix.get_attribute(Attrbiute::Charisma), 0);
        assert_eq!(test_matrix.get_attribute(Attrbiute::Intelligence), 0);
        assert_eq!(test_matrix.get_attribute(Attrbiute::Wisdom), 0);
        assert_eq!(test_matrix.get_attribute(Attrbiute::Constitution), 0);
    }
}
