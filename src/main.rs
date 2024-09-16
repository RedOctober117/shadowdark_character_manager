#![allow(dead_code)]

// use std::ops::Deref;

use crate::{entitytrait::*};

pub mod entity;
pub mod talent;
pub mod entitytrait;

fn main() {

    let mut test_player = Player::new();

    // test_player.add_trait(PlayerTrait::<u16>::new(String::from("trait_1")));

    // let mut entity_traits = EntityTraitMap::new();
    // entity_traits.add_trait(EntityStat::new("test", 1));

    // println!("{:?}", entity_traits.get_trait("test"));
    // let mut stat_block = PlayerStats::new();

    // println!("str: {}", stat_block.get_stat(StatsEnum::Strength));

    // stat_block.set_stat(StatsEnum::Strength, 4);
    // println!("str: {}", stat_block.get_stat(StatsEnum::Strength));

    // for index in 0..6 {
    //     let offset = 15 - StatsEnum::from(index).to_string().len();
    //     println!("{}: {:>offset$}", StatsEnum::from(index), stat_block.get_stat(StatsEnum::from(index)));
    // }
}
