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
