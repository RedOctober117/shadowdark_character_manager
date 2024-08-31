/*
   &T: reference
   T: copy of data if copy trait is implemented, otherwise value
   &mut T: mutable reference
*/

use crate::{stats::*, talent::*};

struct Entity {
    name: String,
    ancestry: AncestryEnum,
    background: Background,
    stats: Stats,
    class: Class,
    level: i16,
    xp: f32,
    hp: i16,
    ac: i16,
    alignment: AlignmentEnum,
    diety: Diety,
    title: String,
    attacks: Vec<Attack>,
    talents: Vec<Talent>,
    spells: Vec<Spell>,
    gear: Vec<Gear>,
}

impl Entity {
    fn new() -> Self {
        todo!()
    }
}

struct Attack { }


struct Spell { }

struct Gear { }

enum AlignmentEnum {
    Chaotic,
    Lawful,
    Neutral,
}

enum Diety {}

enum ClassEnum {
    Fighter,
    Priest,
    Thief,
    Wizard,
}

enum LanguageEnum {
    Common,
    Dwarvish,
    Elvish,
    Giant,
    Goblin,
    Merran,
    Orcish,
    Reptilian,
    Sylvan,
    Thanian,
    Celestial,
    Diabloic,
    Draconic,
    Primordial,
}

struct Class {
    class: ClassEnum,
    languages: Vec<LanguageEnum>,
    talents: Vec<Talent>,
}

enum Background {
    Urchin,
    Wanted,
    CultInitiate,
    ThievesGuild,
    Banished,
    Orphaned,
    WizardsApprentice,
    Jeweler,
    Herbalist,
    Barbarian,
    Mercenary,
    Sailor,
    Acolyte,
    Soldier,
    Ranger,
    Scout,
    Minstrel,
    Scholar,
    Noble,
    Chirurgeon,
    Custom,
}

enum AncestryEnum {
    Dwarf,
    Elf,
    Goblin,
    Halfling,
    HalfOrc,
    Human,
}

struct Ancestry {}


