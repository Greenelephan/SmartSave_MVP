use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
struct PriceAlert {
    id: Option<Uuid>,
    product_id: Uuid,
    target_price: f64,
}

#[get("/products/search")]
async fn search_products(pool: web::Data<PgPool>, query: web::Query<String>) -> impl Responder {
    // TODO: logic for searching a product on its name
    HttpResponse::Ok().body("Search results")
}

#[get("/products/{id}")]
async fn get_product(pool: web::Data<PgPool>, product_id: web::Path<Uuid>) -> impl Responder {
    // TODO: logic for getting information about a product
    HttpResponse::Ok().body("Product details")
}

#[get("/products/{id}/prices")]
async fn get_product_prices(pool: web::Data<PgPool>, product_id: web::Path<Uuid>) -> impl Responder {
    // TODO: logic for getting a product price history
    HttpResponse::Ok().body("Price history")
}

#[get("/products/popular")]
async fn get_popular_products(pool: web::Data<PgPool>) -> impl Responder {
    // TODO: logic for getting most popular products
    HttpResponse::Ok().body("Popular products")
}

#[post("/price-alerts")]
async fn create_price_alert(
    pool: web::Data<PgPool>,
    alert: web::Json<PriceAlert>,
) -> impl Responder {
    // TODO: logic for creating a price alert
    HttpResponse::Ok().body("Price alert created")
}

#[get("/price-alerts")]
async fn list_price_alerts(pool: web::Data<PgPool>) -> impl Responder {
    // TODO: logic for getting a list of price alerts
    HttpResponse::Ok().body("List of price alerts")
}

#[put("/price-alerts/{id}")]
async fn update_price_alert(
    pool: web::Data<PgPool>,
    alert_id: web::Path<Uuid>,
    alert_update: web::Json<PriceAlert>,
) -> impl Responder {
    // TODO: logic for updating a price alert
    HttpResponse::Ok().body("Price alert updated")
}

#[delete("/price-alerts/{id}")]
async fn delete_price_alert(pool: web::Data<PgPool>, alert_id: web::Path<Uuid>) -> impl Responder {
    // TODO: logic for deleting a price alert
    HttpResponse::Ok().body("Price alert deleted")
}
