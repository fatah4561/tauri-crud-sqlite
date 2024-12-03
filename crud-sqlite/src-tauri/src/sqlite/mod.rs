use chrono;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub base_price: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct ProductCategory {
    id: i32,
    name: String,
    product_id: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

pub fn new_db() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("./db.db3");
    return conn;
}

pub fn table_migrations(conn: &Connection) -> Result<()> {
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

pub trait ProductRepository {
    fn insert_product(&self, product: Product) -> Result<()>;
}

impl ProductRepository for Connection {
    fn insert_product(&self, product: Product) -> Result<()> {
        let created_at = format!("{}", &product.created_at.format("%+"));
        let updated_at = format!("{}", &product.updated_at.format("%+"));

        self.execute(
            "INSERT INTO products (name, base_price, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            (&product.name, &product.base_price, &created_at, &updated_at),
        )?;
        Ok(())
    }
}

// pub fn run_sqlite() -> Result<()> {
//     let conn = Connection::open("./db.db3")?;

//     conn.execute(
//         "CREATE TABLE person (
//             id   INTEGER PRIMARY KEY,
//             name TEXT NOT NULL,
//             data BLOB
//         )",
//         (), // empty list of parameters.
//     )?;
//     let me = Person {
//         id: 0,
//         name: "Steven".to_string(),
//         data: None,
//     };
//     conn.execute(
//         "INSERT INTO person (name, data) VALUES (?1, ?2)",
//         (&me.name, &me.data),
//     )?;

//     let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
//     let person_iter = stmt.query_map([], |row| {
//         Ok(Person {
//             id: row.get(0)?,
//             name: row.get(1)?,
//             data: row.get(2)?,
//         })
//     })?;

//     for person in person_iter {
//         println!("Found person {:?}", person.unwrap());
//     }
//     Ok(())
// }
