use std::{env, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use serde::Deserialize;

mod domain {
    pub mod entity {
        pub mod product;
    }
}

mod infrastructure {
    pub mod database {
        pub mod product_repository;
    }
    pub mod cache {
        pub mod redis_cache;
        pub mod cache_provider;
    }
}

mod application {
    pub mod dto {
        pub mod create_product_request;
    }
    pub mod usecase {
        pub mod product_catalog_use_case;
    }
}

use domain::entity::product::Product;
use infrastructure::cache::redis_cache::RedisCache;
use infrastructure::database::product_repository::PostgresProductRepository;
use application::usecase::product_catalog_use_case::ProductCatalogUseCase;
use application::dto::create_product_request::CreateProductRequest;

#[derive(Clone)]
struct AppState {
    service: Arc<ProductCatalogUseCase>,
}

#[derive(Debug, Deserialize)]
struct SearchParams {
    name: Option<String>,
    brand: Option<String>,
    category: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url: String = env::var("REDIS_URL").expect("REDIS_URL must be set");

    let app_host: String = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let app_port: String = env::var("APP_PORT").unwrap_or_else(|_| "3000".to_string());

    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("failed to connect to postgres");

    let repository: Arc<PostgresProductRepository> = Arc::new(PostgresProductRepository::new(pool));
    let cache: Arc<RedisCache> = Arc::new(RedisCache::new(&redis_url).expect("failed to create redis client"));
    let service: Arc<ProductCatalogUseCase> = Arc::new(ProductCatalogUseCase::new(repository, cache));

    let state: AppState = AppState { service };

    let app: Router = build_app(state);

    let bind_address: String = format!("{}:{}", app_host, app_port);
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(&bind_address)
        .await
        .expect("failed to bind tcp listener");

    println!("API running at http://{}", bind_address);

    axum::serve(listener, app)
        .await
        .expect("server failed");
}

fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/products", post(create_product).get(list_products))
        .route("/products/search", get(search_products))
        .route("/products/{id}", get(get_product))
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

async fn create_product(
    State(state): State<AppState>,
    Json(request): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    let product: Product = state
        .service
        .create_product(request)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(product)))
}

async fn list_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    let products: Vec<Product> = state
        .service
        .list_products()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(products))
}

async fn search_products(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    let products: Vec<Product> = state
        .service
        .search_products(params.name, params.brand, params.category)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(products))
}

async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Product>, StatusCode> {
    let product: Option<Product> = state
        .service
        .get_product(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match product {
        Some(product) => Ok(Json(product)),
        None => Err(StatusCode::NOT_FOUND),
    }
}