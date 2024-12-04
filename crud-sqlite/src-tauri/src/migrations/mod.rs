use rusqlite::{Connection, Result};

pub fn migrate(conn: &Connection) -> Result<()> {
    match conn.execute(
        "
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            base_price INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
    ",
        (),
    ) {
        Ok(_) => println!("Products created successfully"),
        Err(e) => println!("Error creating table: {}", e),
    }

    match conn.execute(
        "
        CREATE TABLE IF NOT EXISTS product_categories (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
    ",
        (),
    ) {
        Ok(_) => println!("Product categories created successfully"),
        Err(e) => println!("Error creating table: {}", e),
    }

    Ok(())
}