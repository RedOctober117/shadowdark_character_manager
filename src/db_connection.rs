use rusqlite::{Connection, Result};
use std::io::Seek;
use std::ops::IndexMut;
use std::os::windows::fs::FileExt;
use std::path::Path;
use std::{fs::File, io::Read};

pub struct DBConnection<P>
where
    P: AsRef<Path>,
{
    db_path: P,
    connection: Connection,
}

impl<P: AsRef<Path> + Clone> DBConnection<P> {
    pub fn connect(path: P) -> Self {
        let conn = match Connection::open(path.clone()) {
            Ok(conn) => conn,
            Err(e) => panic!("Could not open database: {}", e),
        };

        Self {
            db_path: path,
            connection: conn,
        }
    }

    pub fn execute_script(&mut self, script_path: P) -> Result<usize> {
        let mut script_buffer = String::new();
        let file = File::open(script_path);

        match file {
            Ok(mut f) => _ = f.read_to_string(&mut script_buffer),
            Err(e) => panic!("Could not open script {}", e),
        }

        let chars: Vec<_> = script_buffer.char_indices().collect();

        for (index, char) in chars {
            match char {
                '\n' | '\r' => _ = script_buffer.remove(index),
                _ => continue,
            }
        }

        let mut statement = self.connection.prepare(&script_buffer)?;
        statement.execute(())
    }

    pub fn query_as_vec(&mut self, query: String) -> Result<Vec<String>> {
        let mut results: Vec<String> = vec![];
        let mut statement = self.connection.prepare(&query)?;

        let _ = statement.query_map([], |row| {
            results.push(format!("{:?}", *row));
            Ok(())
        })?;

        Ok(results)
    }
}
