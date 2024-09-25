/// Attributes relevant to Shadowdark (N)PCs.
#[derive(Copy, Clone, Debug)]
pub enum StatEnum {
    Strength,
    Dexterity,
    Constitution,
    Wisdom,
    Intelligence,
    Charisma,
}

/// The main representation of all attributes used in shadowdark.
#[derive(Clone, Debug)]
pub struct Stats {
    stat_matrix: Vec<Vec<i16>>,
}

impl Stats {
    /// Initializes the required vec of vecs:
    /// ```
    /// [
    ///   (str)[base, . . .],
    ///   (dex)[base, . . .],
    ///    . . .
    /// ]
    /// ```
    pub fn new() -> Self {
        Self {
            //vec![value;times]
            stat_matrix: vec![vec![0; 1]; 6],
        }
    }

    /// Returns an attributes value by summing every value in the relevant vec.
    pub fn get_stat(&self, attribute: StatEnum) -> i16 {
        let mut sum = 0;
        for i in &self.stat_matrix[Stats::stat_to_index(attribute)] {
            sum += i;
        }

        sum
    }

    pub fn get_stat_modifier(&self, attribute: StatEnum) -> i8 {
        let total = self.get_stat(attribute);
        ((total - 10) / 2) as i8
    }

    /// Converts an `Attribute` enum to a `usize` corresponding to its index
    /// in the attribute matrix.
    pub fn stat_to_index(attribute: StatEnum) -> usize {
        match attribute {
            StatEnum::Strength => 0,
            StatEnum::Dexterity => 1,
            StatEnum::Constitution => 2,
            StatEnum::Wisdom => 3,
            StatEnum::Intelligence => 4,
            StatEnum::Charisma => 5,
        }
    }

    /// Add a modifier to an attribute. Does not take ownership of the
    /// attribute or its modifier.
    pub fn add_stat_modifier(&mut self, modifier: &StatModifier) -> usize {
        self.stat_matrix[Stats::stat_to_index(modifier.stat())].push(modifier.modifier);
        self.stat_matrix[0].len() - 1
    }

    /// Removes an `AttributeModifier` by setting its value in the matrix to 0.
    pub fn remove_stat_modifier(&mut self, modifier: &StatModifier) {
        let target_index = modifier.entry_index.unwrap_or(0);
        match target_index {
            0 => (),
            _ => {
                self.stat_matrix[Stats::stat_to_index(modifier.stat())][target_index] = 0;
            }
        }
    }

    pub fn set_stat_base(&mut self, attribute: StatEnum, value: i16) {
        let attribute_index = Stats::stat_to_index(attribute);
        self.stat_matrix[attribute_index][0] = value;
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an attribute modifier. The modifier is signed, as all modifiers
/// are summed when calculating the total for a given attribute.
/// # Examples
/// ```
/// let mut player_stats = Stats::new();
/// let mut new_stat = StatModifier::new(StatEnum::Strength, 1);
/// new_stat.set_entry_index(player_stats.add_stat_modifier(&new_attribute));
///
/// assert_eq!(player_stats.get_stat(StatEnum::Strength), 1);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct StatModifier {
    stat: StatEnum,
    entry_index: Option<usize>,
    modifier: i16,
}

impl StatModifier {
    /// Returns a new `AttributeModifier` with the `entry_index` set to `None`.
    pub fn new(attribute: StatEnum, modifier: i16) -> Self {
        Self {
            stat: attribute,
            entry_index: None,
            modifier,
        }
    }

    /// Returns the attribute modified.
    pub fn stat(&self) -> StatEnum {
        self.stat
    }

    /// Sets the index at which this `AttributeModifier` put its `modifier`
    pub fn set_entry_index(&mut self, index: usize) {
        self.entry_index = Some(index);
    }
}
