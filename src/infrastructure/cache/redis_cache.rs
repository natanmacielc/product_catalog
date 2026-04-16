use async_trait::async_trait;
use redis::AsyncCommands;

use crate::{domain::entity::product::Product, infrastructure::cache::cache_provider::CacheProvider};

#[derive(Clone)]
pub struct RedisCache {
    client: redis::Client,
}

impl RedisCache {
    pub fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client: redis::Client = redis::Client::open(redis_url)?;
        Ok(Self { client })
    }

    async fn connection(&self) -> Result<redis::aio::MultiplexedConnection, redis::RedisError> {
        self.client.get_multiplexed_async_connection().await
    }
}

#[async_trait]
impl CacheProvider for RedisCache {
    async fn get_product(&self, id: i64) -> Result<Option<Product>, String> {
        let mut conn: redis::aio::MultiplexedConnection = self.connection().await.map_err(|e| e.to_string())?;
        let key: String = format!("products:{}", id);

        let value: Option<String> = conn.get(key).await.map_err(|e| e.to_string())?;

        match value {
            Some(json) => serde_json::from_str::<Product>(&json)
                .map(Some)
                .map_err(|e| e.to_string()),
            None => Ok(None),
        }
    }

    async fn set_product(&self, product: &Product, ttl_seconds: u64) -> Result<(), String> {
        let mut conn: redis::aio::MultiplexedConnection = self.connection().await.map_err(|e| e.to_string())?;
        let key: String = format!("products:{}", product.id);
        let json: String = serde_json::to_string(product).map_err(|e| e.to_string())?;

        let _: () = conn
            .set_ex(key, json, ttl_seconds)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_product_list(&self) -> Result<Option<Vec<Product>>, String> {
        let mut conn: redis::aio::MultiplexedConnection = self.connection().await.map_err(|e| e.to_string())?;
        let value: Option<String> = conn.get("products:all").await.map_err(|e| e.to_string())?;

        match value {
            Some(json) => serde_json::from_str::<Vec<Product>>(&json)
                .map(Some)
                .map_err(|e| e.to_string()),
            None => Ok(None),
        }
    }

    async fn set_product_list(
        &self,
        products: &[Product],
        ttl_seconds: u64,
    ) -> Result<(), String> {
        let mut conn: redis::aio::MultiplexedConnection = self.connection().await.map_err(|e| e.to_string())?;
        let json: String = serde_json::to_string(products).map_err(|e| e.to_string())?;

        let _: () = conn
            .set_ex("products:all", json, ttl_seconds)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn del(&self, key: &str) -> Result<(), String> {
        let mut conn: redis::aio::MultiplexedConnection = self.connection().await.map_err(|e| e.to_string())?;
        let _: () = conn.del(key).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}