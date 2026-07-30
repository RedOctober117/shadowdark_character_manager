# Design Model

## Guidance

 - sheet "operations" should be restricted to actions strictly required to 
 calculate some roll, modifier, or to change sheet qualities (ie, level up, 
 rest).
 - operations should only include values present elsewhere on the sheet. The 
 player should hold some responsibility in still playing the game. 

As an example, with Turn Undead, the sheet should only provide the items 
required to cast the spell, and the roll to be made by the player. It should 
not provide any function to handle the success or fail clauses.

## Sheet Model

Character sheet key words:
 - name
 - ancestry
 - class
 - level
 - xp
 - alignment 
 - diety
 - title
 - background
 - stats::stat
 - stats::stat::modifier
 - hp
 - ac
 - attacks
 - talents
 - spells
 - gear

Character sheet states:
 - play
 - level-up
 - dying
 - sneaking??

Passing of time:
 - day
 - night

Actions:
 - roll
    - attack
    - damage
    - check
      - spellcasting

## Calculated fields

### From gear/talents
 - ac

### With base inherent value:
 - all stats
 - weapon attack/damage
 - spell attack/damage
 - carry slots

# Layout

## Core traits

> A trait is some part of the character and its sheet, it does not refer to the Talent or gear system.

```toml
name = str
ancestry = <canonical path>
class = <canonical path> # the talents granted by a class will be stored individually in the Talents table
level = int
xp = int
alignment = <canonical path>
deity = <canonical path>
title = <canonical path>
background = <canonical path>
hp = int
```

## Calculated Fields

```toml
armor-class = int #this may change, im not aware atm if anything grants any base ac other than gear or talents

total-slots = int # = str or 10, whichever is highest

[stats] # base + modifiers from gear/talents
strength = int
dexterity = int
constitution = int
intelligence = int
wisdom = int
charisma = int

[[languages]] #some languages are inherent to the class. these are added automatically at character creation. otheres are chosen as a talent. the language table item will have a `from-talent` field specifying where it came from, if applicable
# required fields
name = str
identity = TBA
source = <canonical path>
# optional fields
from = <sheet path> # form of, ie,  `sheet::talents::<identity>`

[[gear]]
# required fields
name = str
identity = TBA
source = <canonical path>
slots = int
cost = <currency>
amount = int
# optional fields
bonus = <bonus>
properties = []

[[talents]]
# required fields
name = str
description = str
source = <canonical path>
# optional fields
bonus = <bonus>

[[spells]]
name = str
description = str
source = <canonical path>
tier = int
caster = <canonical path> || [<canonical path>, ...]
duration = <duration>
range = <canonical path>
cast = <cast>
```

## Field Shapes

### Paths

#### Canonical Paths
Canonical paths refer to items from rule sets. 

Available paths:
 - `core::`

#### Sheet Paths
Sheet paths refer to the sheet itself, containing character information.

Available paths with root `sheet::`:
 - name
 - ancestry
 - class
 - level
 - xp 
 - alignment
 - deity
 - title 
 - background 
 - hp 
 - armor-class
 - stats
 - gear
 - talents
 - spells

### Bonus
```toml
bonus = {
  | value = <value>
  | choose = <choose>
  to = 
    | <sheet path>
    | self
  when = <sheet action path>
}
```

### Cast
```toml
cast = {
  | requires = <path>
  | check = <sheet path>
  | choose = <choose>
}
```

### Choose
```toml
choose = {
  | from = <path>
  | item = <path>
  | amount = int
  | bonus = <bonus>

}
```

### Roll
```toml
{ roll = 
    | int 
    | <calc>
    | [int | <calc>, ...]
  d = int }
```

### Calc
```toml
{ calc = 
    | add = 
      | int
      | [int | calc, ...]
    | sub
    | mult
    | div = 
      value = int | calc
      by = int
      round = 
        | "up"
        | "down"
}
```

### Duration
```toml
duration = {
  | rounds = int
  | days = int
}
```
















every night i am crush by the weight of my parents' own mortality, and the speed at which my own approaches theirs. it feels as though we are neck and neck; the years catching up with them, my body catching up with me. i can feel my dads terror of it. this alone is evident from the tears brought forth by learning of my sporadic sleep paralysis. ive read that starting dialysis is akin to dying once, and i find this to be an apt analogy. not two years ago, i would drive five hours a day to see my ex. some days, its hard to walk, others, its hard to even sit. i cant sleep without pain medication. my hip, or leg, or something aches on end, driving me to stand to receive any amount of solace. i cant even play a game like old school runescape without feeling a complete terror envelope me at the though of wasting my (presumably) shortened life on an artificially inflated grind. and to what end? presumably thats the question of just about everything now. 