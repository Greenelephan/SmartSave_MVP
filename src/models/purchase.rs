use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub store_id: Uuid,
    pub transaction_id: Uuid,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiptItem {
    pub id: Uuid,
    pub receipt_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShoppingTrip {
    pub id: Uuid,
    pub user_id: Uuid,
    pub store_id: Uuid,
    pub planned_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
