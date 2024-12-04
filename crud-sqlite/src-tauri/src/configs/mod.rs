use rusqlite::{Connection, Result};

pub fn new_sqlite_db() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("../db.db3");
    return conn;
}