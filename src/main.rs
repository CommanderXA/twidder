mod api;
mod db;
mod models;
mod router;
mod tls;

use std::env;

use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;

use crate::tls::load_tls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // DB
    db::db_init()
        .await
        .expect("Got error while connecting to the DB");
    db::db_fill().await?;

    // TLS
    let tls_config = load_tls();

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
        App::new().wrap(logger).configure(router::config)
    })
    .bind_rustls((host, port), tls_config)?
    .workers(5) // Multithreaded mode
    .run()
    .await
}
