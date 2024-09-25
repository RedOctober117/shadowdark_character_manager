/// Attributes relevant to Shadowdark (N)PCs.
#[derive(Copy, Clone, Debug)]
pub enum AttributeEnum {
    Strength,
    Dexterity,
    Constitution,
    Wisdom,
    Intelligence,
    Charisma,
}

/// The main representation of all attributes used in shadowdark.
#[derive(Clone, Debug)]
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
    pub fn get_attribute(&self, attribute: AttributeEnum) -> i16 {
        let mut sum = 0;
        for i in &self.attribute_matrix[Attributes::attribute_to_index(attribute)] {
            sum += i;
        }

        sum
    }

    pub fn get_attribute_modifier(&self, attribute: AttributeEnum) -> i8 {
        let total = self.get_attribute(attribute);
        ((total - 10) / 2) as i8
    }

    /// Converts an `Attribute` enum to a `usize` corresponding to its index
    /// in the attribute matrix.
    pub fn attribute_to_index(attribute: AttributeEnum) -> usize {
        match attribute {
            AttributeEnum::Strength => 0,
            AttributeEnum::Dexterity => 1,
            AttributeEnum::Constitution => 2,
            AttributeEnum::Wisdom => 3,
            AttributeEnum::Intelligence => 4,
            AttributeEnum::Charisma => 5,
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

    pub fn set_attribute_base(&mut self, attribute: AttributeEnum, value: i16) {
        let attribute_index = Attributes::attribute_to_index(attribute);
        self.attribute_matrix[attribute_index][0] = value;
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents an attribute modifier. The modifier is signed, as all modifiers
/// are summed when calculating the total for a given attribute.
/// # Examples
/// ```
/// let mut player_attributes = Attributes::new();
/// let mut new_attribute = AttributeModifier::new(AttributeEnum::Strength, 1);
/// new_attribute.set_entry_index(player_attributes.add_addribute_modifier(&new_attribute));
///
/// assert_eq!(player_attributes.get_attribute(Attrbiute::Strength), 1);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct AttributeModifier {
    attribute: AttributeEnum,
    entry_index: Option<usize>,
    modifier: i16,
}

impl AttributeModifier {
    /// Returns a new `AttributeModifier` with the `entry_index` set to `None`.
    pub fn new(attribute: AttributeEnum, modifier: i16) -> Self {
        Self {
            attribute,
            entry_index: None,
            modifier,
        }
    }

    /// Returns the attribute modified.
    pub fn attribute(&self) -> AttributeEnum {
        self.attribute
    }

    /// Sets the index at which this `AttributeModifier` put its `modifier`
    pub fn set_entry_index(&mut self, index: usize) {
        self.entry_index = Some(index);
    }
}
