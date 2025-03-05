use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalProduct {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricePoint {
    pub id: Uuid,
    pub product_id: Uuid,
    pub store_id: Uuid,
    pub price: f64,
    pub date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceAlert {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub target_price: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
