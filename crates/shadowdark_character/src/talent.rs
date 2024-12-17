// use crate::StatModifier;

use db_connection::db_connection::DBConnection;
use db_connection::queryable::Queryable;
use rusqlite::{params, Params, Result};
use std::path::Path;

/// Represents a Talent. This will eventually be abstracted to include weapon
/// and armour properties as well.
#[derive(Clone, Debug)]
pub struct Talent {
    // name: String,
    description: String,
    // modifiers: Option<Vec<StatModifier>>,
}

impl Queryable for Talent {
    fn select_all(db_conn: &mut DBConnection) -> Result<Vec<Self>> {
        let mut results: Vec<Self> = Vec::new();
        let connection = db_conn.mut_conn();

        let mut stmt = connection.prepare("SELECT * FROM talent")?;
        let iter = stmt.query_map([], |row| {
            Ok(results.push(Self {
                description: row.get(1)?,
            }))
        })?;

        iter.for_each(drop);

        Ok(results)
    }

    fn insert(db_conn: &mut DBConnection, value: Self) -> Result<usize> {
        let conn = db_conn.mut_conn();
        conn.execute(
            "INSERT INTO talent (description) VALUES (?1)",
            params![value.description],
        )
    }

    fn execute(db_conn: &mut DBConnection, statement: &str, params: &str) -> Result<usize> {
        let conn = db_conn.mut_conn();

        conn.execute(statement, params![params])
    }
}

impl Talent {
    pub fn new(description: String) -> Self {
        Self {
            description,
            // modifiers: None,
        }
    }
    pub fn description(&self) -> &str {
        &self.description
    }

    // /// If the `Talent` has modifiers, return a vec of `AttributeModifier`s.
    // /// Else, `None`.``
    // pub fn modifiers(&self) -> Option<&Vec<StatModifier>> {
    //     match &self.modifiers {
    //         Some(e) => Some(e),
    //         None => None,
    //     }
    // }

    // pub fn add_modifier(&mut self, modifier: StatModifier) {
    //     match &mut self.modifiers {
    //         None => {
    //             self.modifiers = Some(vec![modifier]);
    //         }
    //         Some(m) => m.push(modifier),
    //     }
    // }
}
