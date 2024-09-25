use core::fmt;

use crate::item::AbstractItem;

/// Represents an abstract inventory. armour, weapons, talents, and backpacks
/// all have this structure.
#[derive(Clone, Debug)]
pub struct AbstractInventory<T>
where
    T: AbstractItem,
{
    capacity: u8,
    used_slots: u8,
    items: Vec<T>,
}

impl<T: AbstractItem> AbstractInventory<T> {
    /// Builds a new `AbstractInventory` and instantiates an empty vec.
    pub fn new(total_slots: u8) -> Self {
        Self {
            capacity: total_slots,
            used_slots: 0,
            items: vec![],
        }
    }

    /// Returns the capacity of the inventory.
    pub fn capacity(&self) -> u8 {
        self.capacity
    }

    /// Returns the number of used slots.
    pub fn used_slots(&self) -> u8 {
        self.used_slots
    }

    /// Returns a borrowed reference to the stored items.
    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    /// Returns the number of free slots.
    pub fn free_slots(&self) -> u8 {
        self.capacity - self.used_slots
    }

    /// Adds an item to the inventory, if there are enough slots. Else,
    /// `InventoryIsFullError`.
    pub fn add_item(&mut self, item: T) -> Result<(), InventoryIsFullError> {
        if self.free_slots() < item.slots() {
            Err(InventoryIsFullError)
        } else {
            self.items.push(item);
            self.used_slots += 1;
            Ok(())
        }
    }
}

/// Error to signal that an `AbstractInventory` is full.
#[derive(Clone, Debug)]
pub struct InventoryIsFullError;

impl fmt::Display for InventoryIsFullError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inventory is full!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::Item;
    use crate::currency::{Currency, CurrencyEnum};

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
            Currency::new(1, CurrencyEnum::GP),
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
            Currency::new(1, CurrencyEnum::GP),
            3,
        );

        if test_inv.add_item(item_1).is_ok() {
            panic!()
        }
    }
}