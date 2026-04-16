use async_trait::async_trait;

use crate::domain::entity::product::Product;

#[async_trait]
pub trait CacheProvider: Send + Sync {
    async fn get_product_list(&self) -> Result<Option<Vec<Product>>, String>;
    async fn get_product(&self, id: i64) -> Result<Option<Product>, String>;

    async fn set_product(&self, value: &Product, ttl_seconds: u64) -> Result<(), String>;
    async fn set_product_list(&self, value: &[Product], ttl_seconds: u64) -> Result<(), String>;

    async fn del(&self, key: &str) -> Result<(), String>;
}