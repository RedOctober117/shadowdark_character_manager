/// Represents currency in Shadowdark. Currently supports creation and
/// formatting of currency. No math is supported yet.
#[derive(Clone, Copy, Debug)]
pub struct Currency {
    denomination: CurrencyEnum,
    amount: u32,
}

impl Currency {
    pub fn new(amount: u32, denomination: CurrencyEnum) -> Self {
        Self {
            amount,
            denomination,
        }
    }

    pub fn get_roll(&self) -> String {
        format!("{}{:?}", self.amount, self.denomination).to_lowercase()
    }
}

/// Denominations of currency.
#[derive(Clone, Copy, Debug)]
pub enum CurrencyEnum {
    GP,
    SP,
}
