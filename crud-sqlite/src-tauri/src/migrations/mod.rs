use sqlx::{Pool, Sqlite, Error};

pub async fn migrate(pool: &Pool<Sqlite>) -> Result<(), Error> {
    // Create `products` table
    match sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            base_price INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "
    )
    .execute(pool)
    .await
    {
        Ok(_) => println!("Products table created successfully"),
        Err(e) => eprintln!("Error creating products table: {}", e),
    }

    // Create `product_categories` table
    match sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS product_categories (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "
    )
    .execute(pool)
    .await
    {
        Ok(_) => println!("Product categories table created successfully"),
        Err(e) => eprintln!("Error creating product categories table: {}", e),
    }

    Ok(())
}
