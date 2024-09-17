Player:
 - name: String
   - flavoreded
 - race (ancestry): struct
   - flavored
   - provides language(s)
   - talent
 - class: struct
   - flavored
   - gives talent(s)
   - weapon and armour access
   - hit points
 - stats: struct
   - strength
   - dexterity
   - constitution
   - wisdom
   - intelligence
   - charisma
 - equipment
   - items: struct
     - flavored
     - cost
     - slots
   - weapons: extend item
     - cost
     - type
     - range
       - can have multiple ranges
     - damage
       - can change if thrown vs melee for example
     - properties
       - \>= 1 property
       - 2 handed, versatile, etc.
   - armour: extend item
     - cost
     - gear slots
     - ac
     - properties
       - \>= 1 property
       - disadvantage, no swim, etc.
     - is armour just a modifier to a stat? what about armour effects?
 - talents: struct
   - flavor
   - attack modifier
   - stat modifier
   - weapon mastery
   - armour mastery
   - stat modifiers or other effects. like armour?
   - may require choice of other items
 - hp
 - xp
 - title
   - flavored
 - alignment
   - flavored only
 - background
   - flavored
   - may include a list of items to begin with; at dms discretion
 - language
   - flavored
 - spells
   - just weapons?



structure:
```
player:
    name: String
    hp: i16,
    xp: u16,
    alignment: String,
    background: String,
    race: Race,
    class: Class,
    language:
        languages: []Language,
    attributes: [6][_]i8
        [(str)[(base), (modifier), (modifier), . . .],
         (dex)[(base), (modifier), (modifier), . . .], 
        . . .]
    attribute_modifier:
        stat_index: 0..5,
        modifier_index: 0..,
        modifier: i8
    weapon_proficiency:
        []WeaponType (enum)
    armour_proficiency:
        []ArmourType (enum)
    ABSTRACT:
        equipped_armour:
            slots: u8,
            occupied_slots: u8,
            armour: []Armour
        equipped_weapon:
            slots: u8,
            occupied_slots: u8,
            weapons: []Weapon
        backpack:
            slots: u8, (max(str, 10))
            occupied_slots: u8,
            items: []Items,
        talents:
            slots: u8
            occupied_slots: u8,
            talents: []Talent,


```



[
  str[base, 0, -5],
  dex[base, ...],
  cha[base, ...],
  wis[base, ...],
  int[base, ...],
  con[base, ...],
]

11..20
15..20
19..20