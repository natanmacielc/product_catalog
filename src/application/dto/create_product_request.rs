use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub brand: String,
    pub category: String,
    pub price_cents: i64,
}