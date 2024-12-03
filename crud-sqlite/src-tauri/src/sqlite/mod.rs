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
    fn get_all_products(&self) -> Result<Vec<Product>>;
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

    fn get_all_products(&self) -> Result<Vec<Product>> {
        let mut stmt =
            self.prepare("SELECT id, name, base_price, created_at, updated_at FROM products")?;
        let mut products = vec![];
        let rows = stmt.query_map([], |row| {
            let created_at: String = row.get(3)?;
            let updated_at: String = row.get(4)?;

            // Parse the strings into DateTime<Utc>
            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at)
                .map(|dt| dt.with_timezone(&chrono::Utc)) // Convert to UTC
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;

            let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at)
                .map(|dt| dt.with_timezone(&chrono::Utc)) // Convert to UTC
                .map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;

            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                base_price: row.get(2)?,
                created_at,
                updated_at,
            })
        })?;

        for row in rows {
            products.push(row?);
        }
        return Ok(products);
    }
}
