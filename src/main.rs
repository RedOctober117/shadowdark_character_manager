use attributes::AttributeModifier;
use db_connection::DBConnection;
use rusqlite::Result;

pub mod abstract_inventory;
pub mod ancestry;
pub mod armour;
pub mod attributes;
pub mod class;
pub mod currency;
pub mod db_connection;
pub mod dice;
pub mod hp;
pub mod item;
pub mod language;
pub mod talent;
pub mod weapon;
pub mod xp;

pub fn main() -> Result<()> {
    let db_path = "build.db3";
    let script_path = "build.sqlite";

    let mut connection = DBConnection::connect(db_path);

    connection.execute_script(script_path)?;

    let results = connection.query_as_vec("SELECT * FROM stats_enum".to_string())?;

    println!("{:?}", results);

    Ok(())
}

