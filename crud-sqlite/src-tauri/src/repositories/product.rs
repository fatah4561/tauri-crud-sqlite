use crate::models::{Product, ProductCategory};
use chrono;
use sqlx::{Error, Pool, Row, Sqlite};

pub struct ProductRepo<'a> {
    pool: &'a Pool<Sqlite>,
}

impl<'a> ProductRepo<'a> {
    pub fn new(pool: &'a Pool<Sqlite>) -> Self {
        ProductRepo { pool }
    }
}

pub trait ProductRepository {
    // async on public trait
    fn get_all(&self) -> impl std::future::Future<Output = Result<Vec<Product>, Error>> + Send;
    fn insert(
        &self,
        product: Product,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
    fn update(&self, product: Product) -> Result<(), Error>;
    fn delete(&self, id: i32) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

impl<'a> ProductRepository for ProductRepo<'a> {
    async fn get_all(&self) -> Result<Vec<Product>, Error> {
        let mut products = vec![];

        let rows = sqlx::query("SELECT id, name, base_price, created_at, updated_at FROM products")
            .fetch_all(&*self.pool)
            .await?;
        for data in rows {
            let created_at: String = data.try_get("created_at")?;
            let updated_at: String = data.try_get("updated_at")?;

            let created_at = chrono::DateTime::parse_from_rfc3339(&created_at)
                .map(|dt| dt.with_timezone(&chrono::Utc)) // Convert to UTC
                .map_err(|e| sqlx::Error::ColumnDecode {
                    index: "created_at".to_string(),
                    source: Box::new(e),
                })?;

            let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at)
                .map(|dt| dt.with_timezone(&chrono::Utc)) // Convert to UTC
                .map_err(|e| sqlx::Error::ColumnDecode {
                    index: "updated_at".to_string(),
                    source: Box::new(e),
                })?;
            products.push(Product {
                id: data.try_get("id")?,
                name: data.try_get("name")?,
                base_price: data.try_get("base_price")?,
                created_at,
                updated_at,
            });
        }

        return Ok(products);
    }

    async fn insert(&self, product: Product) -> Result<(), Error> {
        let created_at = format!("{}", &product.created_at.format("%+"));
        let updated_at = format!("{}", &product.updated_at.format("%+"));

        sqlx::query(
            "
        INSERT INTO products (name, base_price, created_at, updated_at) 
        VALUES ($1, $2, $3, $4)",
        )
        .bind(&product.name)
        .bind(&product.base_price)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(self.pool)
        .await?;
        Ok(())
    }

    fn update(&self, product: Product) -> Result<(), Error> {
        todo!()
    }

    async fn delete(&self, id: i32) -> Result<(), Error> {
        sqlx::query("DELETE FROM products WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
