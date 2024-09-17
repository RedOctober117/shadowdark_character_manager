/// Attributes relevant to Shadowdark (N)PCs.
#[derive(Copy, Clone)]
pub enum Attrbiute {
    Strength,
    Dexterity,
    Constitution,
    Wisdom,
    Intelligence,
    Charisma,
}

/// Represents an attribute modifier. The modifier is signed, as all modifiers
/// are summed when calculating the total for a given attribute.
/// # Examples
/// ```
/// let mut player_attributes = Attributes::new();
/// let mut new_attribute = AttributeModifier::new(Attribute::Strength, 1);
/// new_attribute.set_entry_index(player_attributes.add_addribute_modifier(&new_attribute));
///
/// assert_eq!(player_attributes.get_attribute(Attrbiute::Strength), 1);
/// ```
#[derive(Copy, Clone)]
pub struct AttributeModifier {
    attribute: Attrbiute,
    entry_index: Option<usize>,
    modifier: i16,
}

impl AttributeModifier {
    /// Returns a new `AttributeModifier` with the `entry_index` set to `None`.
    pub fn new(attribute: Attrbiute, modifier: i16) -> Self {
        Self {
            attribute,
            entry_index: None,
            modifier,
        }
    }

    /// Returns the attribute modified.
    pub fn attribute(&self) -> Attrbiute {
        self.attribute
    }

    /// Sets the index at which this `AttributeModifier` put its `modifier`
    pub fn set_entry_index(&mut self, index: usize) {
        self.entry_index = Some(index);
    }
}

/// The main representation of all attributes used in shadowdark.
pub struct Attributes {
    attribute_matrix: Vec<Vec<i16>>,
}

impl Attributes {
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
            attribute_matrix: vec![vec![0; 1]; 6],
        }
    }

    /// Returns an attributes value by summing every value in the relevant vec.
    pub fn get_attribute(&self, attribute: Attrbiute) -> i16 {
        let mut sum = 0;
        for i in &self.attribute_matrix[Attributes::attribute_to_index(attribute)] {
            sum += i;
        }

        sum
    }

    /// Converts an `Attribute` enum to a `usize` corresponding to its index
    /// in the attribute matrix.
    pub fn attribute_to_index(attribute: Attrbiute) -> usize {
        match attribute {
            Attrbiute::Strength => 0,
            Attrbiute::Dexterity => 1,
            Attrbiute::Constitution => 2,
            Attrbiute::Wisdom => 3,
            Attrbiute::Intelligence => 4,
            Attrbiute::Charisma => 5,
        }
    }

    /// Add a modifier to an attribute. Does not take ownership of the
    /// attribute or its modifier.
    pub fn add_attribute_modifier(&mut self, modifier: &AttributeModifier) -> usize {
        self.attribute_matrix[Attributes::attribute_to_index(modifier.attribute())]
            .push(modifier.modifier);
        self.attribute_matrix[0].len() - 1
    }

    /// Removes an `AttributeModifier` by setting its value in the matrix to 0.
    pub fn remove_attribute_modifier(&mut self, modifier: &AttributeModifier) {
        let target_index = modifier.entry_index.unwrap_or(0);
        match target_index {
            0 => (),
            _ => {
                self.attribute_matrix[Attributes::attribute_to_index(modifier.attribute())]
                    [target_index] = 0;
            }
        }
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self::new()
    }
}
