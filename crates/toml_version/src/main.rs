pub mod parseable;
pub mod path;
pub mod roll;
pub mod sheet;

use toml::Table;

fn main() {
    let toml = "name = \"Chainmail\"
identity = \"?\"
source = \"core::armor::chainmail\"
slots = 2
cost = { unit = \"gp\", value = 60 }
bonus.value = { sum = [13, \"sheet::stats::dexterity::modifier\"] }
bonus.to = \"sheet::armor-class\"
properties = [
    { bonus.value = \"core::roll::disadvantage\", bonus.to = \"swim\" },
    { bonus.value = \"core::roll::disadvantage\", bonus.to = \"stealth\" },
]
amount = 1"
        //     "
        //     cast = { roll = [
        //     1,
        //     { calc.divide.value = \"sheet::level\", calc.divide.by = 2, calc.divide.round = \"down\" },
        // ], d = 6 }"
        .parse::<Table>()
        .unwrap();
    println!("{:?}", toml["bonus"]["value"]);
}
