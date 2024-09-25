use attributes::StatModifier;
use db_connection::db_connection::DBConnection;

pub mod abstract_inventory;

pub mod ancestry;
pub mod armour;
pub mod attributes;
pub mod class;
pub mod currency;
pub mod dice;
pub mod hp;
pub mod item;
pub mod language;
pub mod talent;
pub mod weapon;
pub mod xp;

pub fn main() {
    let path = "db.db3";
    let mut conn = DBConnection::connect(path);

    if let Err(_) = conn.execute_script("build.sqlite") {
        panic!("couldn't execute script");
    }
}

#[cfg(test)]
mod tests {
    use abstract_inventory::AbstractInventory;
    use attributes::{StatEnum, Stats};
    use currency::Currency;
    use dice::Dice;
    use dice::ToRoll;
    use hp::Hp;
    use hp::HpStateEnum;
    use item::Item;
    use xp::Xp;

    use super::*;

    #[test]
    fn test_add_attributes() {
        let mut test_matrix = Stats::new();
        let modifiers = vec![
            StatModifier::new(StatEnum::Strength, 10),
            StatModifier::new(StatEnum::Dexterity, 1),
            StatModifier::new(StatEnum::Charisma, 1),
            StatModifier::new(StatEnum::Intelligence, 1),
            StatModifier::new(StatEnum::Wisdom, 1),
            StatModifier::new(StatEnum::Constitution, 1),
        ];

        for m in modifiers {
            test_matrix.add_stat_modifier(&m);
        }

        assert_eq!(test_matrix.get_stat(StatEnum::Strength), 10);
        assert_eq!(test_matrix.get_stat(StatEnum::Dexterity), 1);
        assert_eq!(test_matrix.get_stat(StatEnum::Charisma), 1);
        assert_eq!(test_matrix.get_stat(StatEnum::Intelligence), 1);
        assert_eq!(test_matrix.get_stat(StatEnum::Wisdom), 1);
        assert_eq!(test_matrix.get_stat(StatEnum::Constitution), 1);

        assert_eq!(test_matrix.get_stat_modifier(StatEnum::Strength), 0);
        assert_eq!(test_matrix.get_stat_modifier(StatEnum::Constitution), -4);
    }

    #[test]
    fn test_remove_attributes() {
        let mut test_matrix = Stats::new();
        let mut modifiers = vec![
            StatModifier::new(StatEnum::Strength, 1),
            StatModifier::new(StatEnum::Dexterity, 1),
            StatModifier::new(StatEnum::Charisma, 1),
            StatModifier::new(StatEnum::Intelligence, 1),
            StatModifier::new(StatEnum::Wisdom, 1),
            StatModifier::new(StatEnum::Constitution, 1),
        ];

        for m in &mut modifiers {
            m.set_entry_index(test_matrix.add_stat_modifier(m));
        }

        for m in modifiers {
            test_matrix.remove_stat_modifier(&m);
        }

        assert_eq!(test_matrix.get_stat(StatEnum::Strength), 0);
        assert_eq!(test_matrix.get_stat(StatEnum::Dexterity), 0);
        assert_eq!(test_matrix.get_stat(StatEnum::Charisma), 0);
        assert_eq!(test_matrix.get_stat(StatEnum::Intelligence), 0);
        assert_eq!(test_matrix.get_stat(StatEnum::Wisdom), 0);
        assert_eq!(test_matrix.get_stat(StatEnum::Constitution), 0);
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

    #[test]
    fn test_add_xp() {
        let mut test_xp = Xp::new(1);
        test_xp.add_xp(5);

        assert_eq!(test_xp.current_xp(), 5);
        assert_eq!(test_xp.lifetime_xp(), 5);
        assert_eq!(test_xp.level(), 1);
    }

    #[test]
    fn test_level_up() {
        let mut test_xp = Xp::new(1);
        test_xp.add_xp(30);

        assert_eq!(test_xp.current_xp(), 0);
        assert_eq!(test_xp.lifetime_xp(), 30);
        assert_eq!(test_xp.level(), 2);
    }

    #[test]
    fn test_hp_damage() {
        let mut test_hp = Hp::new(10, ToRoll::new(Dice::D8, 1));
        test_hp.damage(5);

        assert_eq!(test_hp.current(), 5);
        assert_eq!(test_hp.state(), HpStateEnum::Alive);
    }
}
