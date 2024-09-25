use rusqlite::{Params, Result};
use std::path::Path;

use crate::db_connection::DBConnection;

pub trait Queryable<T, C>
where
    C: AsRef<Path>,
{
    fn select_all(db_conn: &mut DBConnection<C>) -> Result<Vec<T>>;
    fn insert(db_conn: &mut DBConnection<C>, value: T) -> Result<i64>;
    fn execute<P: Params>(
        db_conn: &mut DBConnection<C>,
        statement: &str,
        params: P,
    ) -> Result<usize>;
}
