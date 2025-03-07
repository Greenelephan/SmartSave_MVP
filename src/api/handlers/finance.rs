use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
struct Category {
    id: Option<Uuid>,
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Transaction {
    id: Option<Uuid>,
    amount: f64,
    category_id: Uuid,
    description: String,
}

#[get("/categories")]
async fn list_categories(pool: web::Data<PgPool>) -> impl Responder {
    // TODO: logic for getting a list of categories
    HttpResponse::Ok().body("List of categories")
}

#[post("/categories")]
async fn create_category(
    pool: web::Data<PgPool>,
    new_category: web::Json<Category>,
) -> impl Responder {
    // TODO: logic for creating a category
    HttpResponse::Ok().body("Category created")
}

#[put("/categories/{id}")]
async fn update_category(
    pool: web::Data<PgPool>,
    category_id: web::Path<Uuid>,
    category_update: web::Json<Category>,
) -> impl Responder {
    // TODO: logic for updating a category
    HttpResponse::Ok().body("Category updated")
}

#[delete("/categories/{id}")]
async fn delete_category(
    pool: web::Data<PgPool>,
    category_id: web::Path<Uuid>,
) -> impl Responder {
    // TODO: logic for deleting a category
    HttpResponse::Ok().body("Category deleted")
}

#[get("/transactions")]
async fn list_transactions(pool: web::Data<PgPool>) -> impl Responder {
    // TODO: logic for getting a list of transactions
    HttpResponse::Ok().body("List of transactions")
}

#[post("/transactions")]
async fn create_transaction(
    pool: web::Data<PgPool>,
    new_transaction: web::Json<Transaction>,
) -> impl Responder {
    // TODO: logic for creating of a transaction
    HttpResponse::Ok().body("Transaction created")
}

#[put("/transactions/{id}")]
async fn update_transaction(
    pool: web::Data<PgPool>,
    transaction_id: web::Path<Uuid>,
    transaction_update: web::Json<Transaction>,
) -> impl Responder {
    // TODO: logic for updating a transaction
    HttpResponse::Ok().body("Transaction updated")
}

#[delete("/transactions/{id}")]
async fn delete_transaction(
    pool: web::Data<PgPool>,
    transaction_id: web::Path<Uuid>,
) -> impl Responder {
    // TODO: logic for deleting a transaction
    HttpResponse::Ok().body("Transaction deleted")
}
