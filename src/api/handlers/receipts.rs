use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
struct ReceiptItem {
    id: Option<Uuid>,
    name: String,
    price: f64,
}

#[post("/receipts/upload")]
async fn upload_receipt(pool: web::Data<PgPool>) -> impl Responder {
    // TODO: logic for receipt image upload
    HttpResponse::Ok().body("Receipt uploaded")
}

#[post("/receipts/process")]
async fn process_receipt(pool: web::Data<PgPool>) -> impl Responder {
    // TODO: logic for receipt image processing with OCR
    HttpResponse::Ok().body("Receipt processed")
}

#[get("/receipts")]
async fn list_receipts(pool: web::Data<PgPool>) -> impl Responder {
    // TODO: logic for getting a list of users receipts
    HttpResponse::Ok().body("List of receipts")
}

#[get("/receipts/{id}")]
async fn get_receipt(pool: web::Data<PgPool>, receipt_id: web::Path<Uuid>) -> impl Responder {
    // TODO: logic for getting a details of a specific receipt
    HttpResponse::Ok().body("Receipt details")
}

#[post("/receipts/{id}/items")]
async fn add_receipt_item(
    pool: web::Data<PgPool>,
    receipt_id: web::Path<Uuid>,
    item: web::Json<ReceiptItem>,
) -> impl Responder {
    // TODO: logic for adding a new item to the receipt
    HttpResponse::Ok().body("Item added to receipt")
}

#[put("/receipts/{id}/items/{itemId}")]
async fn update_receipt_item(
    pool: web::Data<PgPool>,
    receipt_id: web::Path<Uuid>,
    item_id: web::Path<Uuid>,
    item_update: web::Json<ReceiptItem>,
) -> impl Responder {
    // TODO: logic for updating a receipt item
    HttpResponse::Ok().body("Receipt item updated")
}
