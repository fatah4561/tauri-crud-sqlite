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
    pub id: i32,
    pub name: String,
    pub product_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}