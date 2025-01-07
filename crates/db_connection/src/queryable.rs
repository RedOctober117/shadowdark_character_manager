use rusqlite::Result;

use crate::db_connection::DBConnection;

pub trait Queryable {
    fn select_all(db_conn: &mut DBConnection) -> Result<Vec<Self>>
    where
        Self: Sized;
    fn insert(db_conn: &mut DBConnection, value: Self) -> Result<usize>;
    fn execute(db_conn: &mut DBConnection, statement: &str, params: &str) -> Result<usize>;
}
