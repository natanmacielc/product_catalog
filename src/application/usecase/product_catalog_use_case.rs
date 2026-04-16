use std::sync::Arc;

use crate::application::dto::create_product_request::CreateProductRequest;
use crate::domain::entity::product::Product;
use crate::infrastructure::cache::cache_provider::CacheProvider;
use crate::infrastructure::database::product_repository::ProductRepository;

pub struct ProductCatalogUseCase {
    repository: Arc<dyn ProductRepository>,
    cache: Arc<dyn CacheProvider>,
}

impl ProductCatalogUseCase {
    pub fn new(
        repository: Arc<dyn ProductRepository>,
        cache: Arc<dyn CacheProvider>,
    ) -> Self {
        Self { repository, cache }
    }

    pub async fn create_product(
        &self,
        request: CreateProductRequest,
    ) -> Result<Product, sqlx::Error> {
        let product: Product = self.repository.create(request).await?;

        let _ = self.cache.del("products:all").await;
        let _ = self.cache.del(&format!("products:{}", product.id)).await;

        Ok(product)
    }

    pub async fn list_products(&self) -> Result<Vec<Product>, sqlx::Error> {
        if let Ok(Some(products)) = self.cache.get_product_list().await {
            return Ok(products);
        }

        let products: Vec<Product> = self.repository.find_all().await?;
        let _ = self.cache.set_product_list(&products, 60).await;

        Ok(products)
    }

    pub async fn get_product(&self, id: i64) -> Result<Option<Product>, sqlx::Error> {
        if let Ok(Some(product)) = self.cache.get_product(id).await {
            return Ok(Some(product));
        }

        let product: Option<Product> = self.repository.find_by_id(id).await?;

        if let Some(ref found_product) = product {
            let _ = self.cache.set_product(found_product, 60).await;
        }

        Ok(product)
    }

    pub async fn search_products(
        &self,
        name: Option<String>,
        brand: Option<String>,
        category: Option<String>,
    ) -> Result<Vec<Product>, sqlx::Error> {
        self.repository.search(name, brand, category).await
    }
}