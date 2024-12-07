use std::sync::Mutex;
use rusqlite::{Connection, Result};
use std::path::Path;

pub struct DbConnection {
    pub conn: Mutex<Connection>,
}

impl DbConnection {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Initialize tables using schema.sql
        let schema = include_str!("schema.sql");
        conn.execute_batch(schema)?;

        Ok(Self { conn: Mutex::new(conn) })
    }
}
