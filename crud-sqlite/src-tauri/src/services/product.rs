use crate::repositories::product::ProductRepo;
use sqlx::{Pool, Sqlite};

pub struct ProductService<'a> {
    pub pool: &'a Pool<Sqlite>,
    pub product_repository:  ProductRepo<'a>,
}

impl<'a> ProductService<'a> {
    pub fn new (pool: &'a Pool<Sqlite>) -> Self {
        let product_repository = ProductRepo::new(pool);
        ProductService { pool, product_repository }
    }
} 