pub mod expression;
pub mod roll;

use toml::Table;

fn main() {
    let toml = "
    cast = { roll = [
    1,
    { calc.divide.value = \"sheet::level\", calc.divide.by = 2, calc.divide.round = \"down\" },
], d = 6 }"
        .parse::<Table>()
        .unwrap();
    println!("{}", toml);
}
