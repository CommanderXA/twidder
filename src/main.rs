mod api;
mod router;
mod tls;
mod models;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Server runs at `http://127.0.0.1:8080`\nPress `ctrl + c` to stop.");

    HttpServer::new(|| App::new().configure(router::config))
        .bind(("127.0.0.1", 8080))?
        .workers(5) // Many threads
        .run()
        .await
}
