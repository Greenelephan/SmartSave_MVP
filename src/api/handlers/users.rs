use actix_web::{get, put, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct UpdateUser {
    username: Option<String>,
    email: Option<String>,
}

#[get("/users/me")]
async fn get_current_user(pool: web::Data<PgPool>) -> impl Responder {
    //TODO: logic for getting current user info
    HttpResponse::Ok().body("Current user info")
}

#[put("/users/me")]
async fn update_user(
    pool: web::Data<PgPool>,
    user_update: web::Json<UpdateUser>,
) -> impl Responder {
    //TODO: logic for updating user info
    HttpResponse::Ok().body("User profile updated")
}

#[get("/users/settings")]
async fn get_user_settings(pool: web::Data<PgPool>) -> impl Responder {
    // TODO: logic for getting user settings
    HttpResponse::Ok().body("User settings")
}

#[put("/users/settings")]
async fn update_user_settings(
    pool: web::Data<PgPool>,
    settings_update: web::Json<UpdateUser>,
) -> impl Responder {
    // Logic for updating user settings
    HttpResponse::Ok().body("User settings updated")
}
