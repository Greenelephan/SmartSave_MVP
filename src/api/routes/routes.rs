use actix_web::web;
use crate::api::handlers::auth;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(auth::register)
            .service(auth::login)
            .service(auth::refresh)
            .service(auth::logout),
    );
}
