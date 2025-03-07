use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct RegisterUser {
    username: String,
    password: String,
}

#[post("/auth/register")]
async fn register(user: web::Json<RegisterUser>) -> impl Responder {
    HttpResponse::Ok().body("User registered")
}

#[post("/auth/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("User logged in")
}

#[post("/auth/refresh")]
async fn refresh() -> impl Responder {
    HttpResponse::Ok().body("Token refreshed")
}

#[post("/auth/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("User logged out")
}
