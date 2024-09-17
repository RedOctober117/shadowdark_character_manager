use crate::dice::ToRoll;

/// Use an enum to implement status like unconscious, etc.
pub struct Hp {
    total: u16,
    current: u16,
    hit_die: ToRoll,
    state: HpStateEnum,
}

impl Hp {
    pub fn new(total: u16, hit_die: ToRoll) -> Self {
        Self {
            total,
            current: total,
            hit_die,
            state: HpStateEnum::Alive,
        }
    }

    pub fn current(&self) -> u16 {
        self.current
    }

    pub fn total(&self) -> u16 {
        self.total
    }

    pub fn hit_die(&self) -> ToRoll {
        self.hit_die
    }

    pub fn state(&self) -> HpStateEnum {
        self.state
    }

    pub fn damage(&mut self, damage: u16) {
        self.current -= damage;
        if self.current == 0 {
            self.state = HpStateEnum::Dying;
        }
    }

    pub fn kill(&mut self) {
        self.current = 0;
        self.state = HpStateEnum::Dead;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HpStateEnum {
    Alive,
    Dying,
    Dead,
}
