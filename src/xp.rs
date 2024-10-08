pub struct Xp {
    current_xp: u16,
    lifetime_xp: u16,
    level: u16,
}

impl Xp {
    pub fn new(level: u16) -> Self {
        Self {
            level,
            current_xp: 0,
            lifetime_xp: 0,
        }
    }
    pub fn current_xp(&self) -> u16 {
        self.current_xp
    }

    pub fn lifetime_xp(&self) -> u16 {
        self.lifetime_xp
    }

    pub fn level(&self) -> u16 {
        self.level
    }

    /// Level 0 -> 1 is at DM's discretion, therefore not our problem.
    pub fn add_xp(&mut self, xp: u16) {
        self.lifetime_xp += xp;
        self.current_xp += xp;

        let level_cap = self.level * 10;
        if self.current_xp >= level_cap {
            self.level += 1;
            self.current_xp = 0;
        }
    }
}
