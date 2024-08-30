#![allow(dead_code)]

use crate::entity::*;

pub mod entity;


fn main() {
    let mut stat_block = Stats::new();

    println!("str: {}", stat_block.get_stat(StatsEnum::Strength));

    stat_block.set_stat(StatsEnum::Strength, 4);
    println!("str: {}", stat_block.get_stat(StatsEnum::Strength));

    for index in 0..6 {
        let offset = 15 - StatsEnum::from(index).to_string().len();
        println!("{}: {:>offset$}", StatsEnum::from(index), stat_block.get_stat(StatsEnum::from(index)));
    }
}
