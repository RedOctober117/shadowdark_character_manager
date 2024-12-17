use rusqlite::{Connection, Result};
use std::{fs::File, io::Read};

pub struct DBConnection {
    db_path: String,
    connection: Connection,
}

impl DBConnection {
    pub fn connect(path: String) -> Self {
        let conn = match Connection::open(path.clone()) {
            Ok(conn) => conn,
            Err(e) => panic!("Could not open database: {}", e),
        };

        Self {
            db_path: path,
            connection: conn,
        }
    }

    pub fn execute_script(&mut self, script_path: String) -> Result<()> {
        let mut script_buffer = String::new();
        let file = File::open(script_path);

        match file {
            Ok(mut f) => _ = f.read_to_string(&mut script_buffer),
            Err(e) => panic!("Could not open script {}", e),
        }

        self.connection.execute_batch(&script_buffer)
    }

    pub fn mut_conn(&mut self) -> &mut Connection {
        &mut self.connection
    }
}
