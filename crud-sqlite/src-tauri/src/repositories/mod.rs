use chrono;
use rusqlite::{params, Connection, Result};
use crate::models::{Product, ProductCategory};

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
