use rusqlite::{Connection, Result};
use std::path::Path;
use std::{fs::File, io::Read};

pub struct DBConnection<P>
where
    P: AsRef<Path>,
{
    _db_path: P,
    connection: Connection,
}

impl<P: AsRef<Path> + Clone> DBConnection<P> {
    pub fn connect(path: P) -> Self {
        let conn = match Connection::open(path.clone()) {
            Ok(conn) => conn,
            Err(e) => panic!("Could not open database: {}", e),
        };

        Self {
            _db_path: path,
            connection: conn,
        }
    }

    pub fn execute_script(&mut self, script_path: P) -> Result<()> {
        let mut script_buffer = String::new();
        let file = File::open(script_path);

        match file {
            Ok(mut f) => _ = f.read_to_string(&mut script_buffer),
            Err(e) => panic!("Could not open script {}", e),
        }

        self.connection.execute_batch(&script_buffer)
    }
}
