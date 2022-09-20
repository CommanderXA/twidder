mod api;
mod db;
mod models;
mod router;
mod tls;

use std::env;

use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

use crate::tls::load_tls;

#[derive(Debug, Clone)]
pub struct AppState {
    db_conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    // DataBase
    let conn = db::db_init()
        .await
        .expect("Got error while connecting to the DB");
    Migrator::up(&conn, None)
        .await
        .expect("Error occured during migrations");
    db::db_fill().await?;

    // TLS
    let tls_config = load_tls();

    // Build App State
    let state = AppState { db_conn: conn };

    // Server Setup
    let host = env::var("HOST").expect("`HOST` must be set in the `.env` file!");
    let port: u16 = env::var("PORT")
        .expect("`PORT` must be set in the `.env` file!")
        .parse::<u16>()
        .expect("`PORT` must be a postitve number");

    println!("Server runs at `https://{host}:{port}`\nPress `ctrl + c` to stop.");

    // Server
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(logger)
            .default_service(web::to(api::errors::not_found))
            .configure(router::config)
    })
    .bind_rustls((host, port), tls_config)?
    .workers(5) // Multithreaded mode
    .run()
    .await
}
