/// Enumerates the dice available in Shadowdark.
#[derive(Clone, Copy, Debug)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

/// Represents a roll in Shadowdark, ie., 1d20, 5d6.
#[derive(Clone, Copy, Debug)]
pub struct ToRoll {
    die: Dice,
    times: u8,
}

impl ToRoll {
    /// Create a roll.
    pub fn new(die: Dice, times: u8) -> Self {
        Self { die, times }
    }

    /// Get a formatted `String` of the roll.
    pub fn get_roll(&self) -> String {
        format!("{:?}{:?}", self.times, self.die).to_lowercase()
    }
}
