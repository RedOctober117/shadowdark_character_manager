use crate::dice::ToRoll;
use std::cmp::{min,max};

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

    pub fn heal(&mut self, value: u16) {
        self.current = min(
            match self.current().checked_add(value) {
                Some(val) => val,
                None => u16::MAX
            },
            self.total()
        )

    }

    pub fn damage(&mut self, value: u16) {
        self.current = max(
            match self.current().checked_sub(value) {
                Some(val) => val,
                None => u16::MIN
            },
            0
        );
        if self.current() == 0 {
            self.state = HpStateEnum::Dying
        };
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dice::Dice;

    #[test]
    fn test_hp_healing() {
        let mut hp = Hp{
            total: 10,
            hit_die: ToRoll::new(Dice::D10, 1),
            current: 5,
            state: HpStateEnum::Alive
        };
        hp.heal(2);
        assert_eq!(hp.current(), 7)
    }

    #[test]
    fn test_hp_healing_max() {
        let mut hp = Hp{
            total: 10,
            hit_die: ToRoll::new(Dice::D10, 1),
            current: 8,
            state: HpStateEnum::Alive
        };
        hp.heal(5);
        assert_eq!(hp.current(), hp.total())
    }

    #[test]
    fn test_hp_damage() {
        let mut hp = Hp{
            total: 10,
            hit_die: ToRoll::new(Dice::D10, 1),
            current: 10,
            state: HpStateEnum::Alive
        };
        hp.damage(5);
        assert_eq!(hp.current(), 5)
    }

    #[test]
    fn test_hp_dying() {
        let mut hp = Hp{
            total: 10,
            hit_die: ToRoll::new(Dice::D10, 1),
            current: 10,
            state: HpStateEnum::Alive
        };
        hp.damage(13);
        assert_eq!(hp.current(), 0);
        assert_eq!(hp.state(), HpStateEnum::Dying);
    }

    #[test]
    fn test_hp_kill() {
        let mut hp = Hp{
            total: 10,
            hit_die: ToRoll::new(Dice::D10, 1),
            current: 10,
            state: HpStateEnum::Alive
        };
        hp.kill();
        assert_eq!(hp.current(), 0);
        assert_eq!(hp.state(), HpStateEnum::Dead);
    }

    #[test]
    fn test_hp_healing_u16overflow() {
        let mut hp = Hp{
            total: u16::max_value(),
            current: u16::max_value(),
            hit_die: ToRoll::new(Dice::D10, 1),
            state: HpStateEnum::Alive
        };
        hp.heal(1);
        assert_eq!(hp.current(), hp.total());
    }
}