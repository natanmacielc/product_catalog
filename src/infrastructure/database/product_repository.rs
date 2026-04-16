use async_trait::async_trait;
use sqlx::{PgPool, Row};

use crate::domain::entity::product::Product;
use crate::application::dto::create_product_request::CreateProductRequest;

#[async_trait::async_trait]
pub trait ProductRepository: Send + Sync {
    async fn create(&self, request: CreateProductRequest) -> Result<Product, sqlx::Error>;
    async fn find_all(&self) -> Result<Vec<Product>, sqlx::Error>;
    async fn find_by_id(&self, id: i64) -> Result<Option<Product>, sqlx::Error>;
    async fn search(
        &self,
        name: Option<String>,
        brand: Option<String>,
        category: Option<String>,
    ) -> Result<Vec<Product>, sqlx::Error>;
}

pub struct PostgresProductRepository {
    pool: PgPool,
}

impl PostgresProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for PostgresProductRepository {
    async fn create(&self, request: CreateProductRequest) -> Result<Product, sqlx::Error> {
        let row: sqlx::postgres::PgRow = sqlx::query(
            r#"
            INSERT INTO products (name, brand, category, price_cents)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, brand, category, price_cents
            "#,
        )
        .bind(request.name)
        .bind(request.brand)
        .bind(request.category)
        .bind(request.price_cents)
        .fetch_one(&self.pool)
        .await?;

        Ok(Product {
            id: row.get("id"),
            name: row.get("name"),
            brand: row.get("brand"),
            category: row.get("category"),
            price_cents: row.get("price_cents"),
        })
    }

    async fn find_all(&self) -> Result<Vec<Product>, sqlx::Error> {
        let rows: Vec<sqlx::postgres::PgRow> = sqlx::query(
            r#"
            SELECT id, name, brand, category, price_cents
            FROM products
            ORDER BY id
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let products: Vec<Product> = rows
            .into_iter()
            .map(|row| Product {
                id: row.get("id"),
                name: row.get("name"),
                brand: row.get("brand"),
                category: row.get("category"),
                price_cents: row.get("price_cents"),
            })
            .collect();

        Ok(products)
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Product>, sqlx::Error> {
        let row: Option<sqlx::postgres::PgRow> = sqlx::query(
            r#"
            SELECT id, name, brand, category, price_cents
            FROM products
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| Product {
            id: row.get("id"),
            name: row.get("name"),
            brand: row.get("brand"),
            category: row.get("category"),
            price_cents: row.get("price_cents"),
        }))
    }

    async fn search(
        &self,
        name: Option<String>,
        brand: Option<String>,
        category: Option<String>,
    ) -> Result<Vec<Product>, sqlx::Error> {
        let rows: Vec<sqlx::postgres::PgRow> = sqlx::query(
            r#"
            SELECT id, name, brand, category, price_cents
            FROM products
            WHERE ($1::TEXT IS NULL OR name ILIKE '%' || $1 || '%')
              AND ($2::TEXT IS NULL OR brand ILIKE '%' || $2 || '%')
              AND ($3::TEXT IS NULL OR category ILIKE '%' || $3 || '%')
            ORDER BY id
            "#,
        )
        .bind(name)
        .bind(brand)
        .bind(category)
        .fetch_all(&self.pool)
        .await?;

        let products: Vec<Product> = rows
            .into_iter()
            .map(|row| Product {
                id: row.get("id"),
                name: row.get("name"),
                brand: row.get("brand"),
                category: row.get("category"),
                price_cents: row.get("price_cents"),
            })
            .collect();

        Ok(products)
    }
}