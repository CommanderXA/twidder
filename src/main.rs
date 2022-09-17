mod api;
mod models;
mod router;
mod tls;

use std::env;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Server Setup
    let host = env::var("HOST").expect("`HOST` must be set in the `.env` file!");
    let port: u16 = env::var("PORT")
        .expect("`PORT` must be set in the `.env` file!")
        .parse::<u16>()
        .expect("`PORT` must be a postitve number");

    println!("Server runs at `http://{host}:{port}`\nPress `ctrl + c` to stop.");

    HttpServer::new(|| App::new().configure(router::config))
        .bind((host, port))?
        .workers(5) // Many threads
        .run()
        .await
}
