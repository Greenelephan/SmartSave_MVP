use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env;

mod api;
mod config;
mod models;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    unsafe {
        env::set_var("RUST_LOG", "actix_web=info");
    }

    // Initialization of the Configuration and connection to the DB
    let config = config::Config::from_env().unwrap();
    let pool = config.db_pool().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
