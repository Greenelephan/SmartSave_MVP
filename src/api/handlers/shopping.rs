use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
struct ShoppingTrip {
    id: Option<Uuid>,
    name: String,
    date: chrono::NaiveDate,
}

#[post("/shopping-trips")]
async fn create_shopping_trip(
    pool: web::Data<PgPool>,
    trip: web::Json<ShoppingTrip>,
) -> impl Responder {
    // TODO: logic for creating a shopping trip plan
    HttpResponse::Ok().body("Shopping trip created")
}

#[get("/shopping-trips")]
async fn list_shopping_trips(pool: web::Data<PgPool>) -> impl Responder {
    // TODO: logic for getting a list of shipping trips
    HttpResponse::Ok().body("List of shopping trips")
}

#[get("/shopping-trips/{id}")]
async fn get_shopping_trip(pool: web::Data<PgPool>, trip_id: web::Path<Uuid>) -> impl Responder {
    // TODO: logic for getting shopping trip details
    HttpResponse::Ok().body("Shopping trip details")
}

#[put("/shopping-trips/{id}")]
async fn update_shopping_trip(
    pool: web::Data<PgPool>,
    trip_id: web::Path<Uuid>,
    trip_update: web::Json<ShoppingTrip>,
) -> impl Responder {
    // TODO: logic for updating a shopping trip
    HttpResponse::Ok().body("Shopping trip updated")
}

#[delete("/shopping-trips/{id}")]
async fn delete_shopping_trip(pool: web::Data<PgPool>, trip_id: web::Path<Uuid>) -> impl Responder {
    // TODO: logic for deleting a shopping trip
    HttpResponse::Ok().body("Shopping trip deleted")
}
