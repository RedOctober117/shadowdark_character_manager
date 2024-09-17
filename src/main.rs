use attributes::{AttributeEnum, AttributeModifier, Attributes};

pub mod armour;
pub mod attributes;
pub mod currency;
pub mod dice;
pub mod inventory;
pub mod item;
pub mod talent;
pub mod weapon;

pub fn main() {
    let mut test_attributes = Attributes::new();
    let mut modifier_1 = AttributeModifier::new(AttributeEnum::Strength, 1);
    // let mut modifier_2 = AttributeModifier::new(Attrbiute::Strength, -5);
    modifier_1.set_entry_index(test_attributes.add_attribute_modifier(&modifier_1));
    // modifier_2.set_entry_index(test_attributes.add_attrbute_modifier(&modifier_2));

    println!("{}", test_attributes.get_attribute(AttributeEnum::Strength));

    test_attributes.remove_attribute_modifier(&modifier_1);
    println!("{}", test_attributes.get_attribute(AttributeEnum::Strength));
}

#[cfg(test)]
mod tests {
    use currency::Currency;
    use inventory::AbstractInventory;
    use item::Item;

    use super::*;

    #[test]
    fn test_add_attributes() {
        let mut test_matrix = Attributes::new();
        let modifiers = vec![
            AttributeModifier::new(AttributeEnum::Strength, 10),
            AttributeModifier::new(AttributeEnum::Dexterity, 1),
            AttributeModifier::new(AttributeEnum::Charisma, 1),
            AttributeModifier::new(AttributeEnum::Intelligence, 1),
            AttributeModifier::new(AttributeEnum::Wisdom, 1),
            AttributeModifier::new(AttributeEnum::Constitution, 1),
        ];

        for m in modifiers {
            test_matrix.add_attribute_modifier(&m);
        }

        assert_eq!(test_matrix.get_attribute(AttributeEnum::Strength), 10);
        assert_eq!(test_matrix.get_attribute(AttributeEnum::Dexterity), 1);
        assert_eq!(test_matrix.get_attribute(AttributeEnum::Charisma), 1);
        assert_eq!(test_matrix.get_attribute(AttributeEnum::Intelligence), 1);
        assert_eq!(test_matrix.get_attribute(AttributeEnum::Wisdom), 1);
        assert_eq!(test_matrix.get_attribute(AttributeEnum::Constitution), 1);

        assert_eq!(
            test_matrix.get_attribute_modifier(AttributeEnum::Strength),
            0
        );
        assert_eq!(
            test_matrix.get_attribute_modifier(AttributeEnum::Constitution),
            -4
        );
    }

    #[test]
    fn test_remove_attributes() {
        let mut test_matrix = Attributes::new();
        let mut modifiers = vec![
            AttributeModifier::new(AttributeEnum::Strength, 1),
            AttributeModifier::new(AttributeEnum::Dexterity, 1),
            AttributeModifier::new(AttributeEnum::Charisma, 1),
            AttributeModifier::new(AttributeEnum::Intelligence, 1),
            AttributeModifier::new(AttributeEnum::Wisdom, 1),
            AttributeModifier::new(AttributeEnum::Constitution, 1),
        ];

        for m in &mut modifiers {
            m.set_entry_index(test_matrix.add_attribute_modifier(m));
        }

        for m in modifiers {
            test_matrix.remove_attribute_modifier(&m);
        }

        assert_eq!(test_matrix.get_attribute(AttributeEnum::Strength), 0);
        assert_eq!(test_matrix.get_attribute(AttributeEnum::Dexterity), 0);
        assert_eq!(test_matrix.get_attribute(AttributeEnum::Charisma), 0);
        assert_eq!(test_matrix.get_attribute(AttributeEnum::Intelligence), 0);
        assert_eq!(test_matrix.get_attribute(AttributeEnum::Wisdom), 0);
        assert_eq!(test_matrix.get_attribute(AttributeEnum::Constitution), 0);
    }

    #[test]
    fn test_new_inventory() {
        let test_inv: AbstractInventory<Item> = AbstractInventory::new(1);
        assert_eq!(test_inv.used_slots(), 0);
        assert_eq!(test_inv.free_slots(), 1);
        assert_eq!(test_inv.capacity(), 1);
    }

    #[test]
    fn test_add_item_to_inv() {
        let mut test_inv: AbstractInventory<Item> = AbstractInventory::new(2);
        let item_1 = Item::new(
            String::from("item_1"),
            Currency::new(1, currency::CurrencyEnum::GP),
            1,
        );

        assert!(test_inv.add_item(item_1).is_ok());
        assert_eq!(test_inv.free_slots(), 1);
    }

    #[test]
    fn test_overfill_inv() {
        let mut test_inv: AbstractInventory<Item> = AbstractInventory::new(2);
        let item_1 = Item::new(
            String::from("item_1"),
            Currency::new(1, currency::CurrencyEnum::GP),
            3,
        );

        if test_inv.add_item(item_1).is_ok() {
            panic!()
        }
    }
}
