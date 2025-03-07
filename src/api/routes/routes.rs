use actix_web::web;
use crate::api::handlers::{auth, users, finance};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(auth::register)
            .service(auth::login)
            .service(auth::refresh)
            .service(auth::logout)
            .service(users::get_current_user)
            .service(users::update_user)
            .service(users::get_user_settings)
            .service(users::update_user_settings)
            .service(finance::list_categories)
            .service(finance::create_category)
            .service(finance::update_category)
            .service(finance::delete_category)
            .service(finance::list_transactions)
            .service(finance::create_transaction)
            .service(finance::update_transaction)
            .service(finance::delete_transaction),
    );
}
