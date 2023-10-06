mod app;
mod security;
use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let host = env::var("HOST").expect("$HOST is not set");

    let port = env::var("PORT")
        .expect("$PORT is not set")
        .parse()
        .expect("$PORT cannot be converted to uint_16");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(Cors::permissive().supports_credentials())
            .configure(app::register_routes)
    })
    .bind((host, port))?
    .run()
    .await
}
