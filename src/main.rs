pub mod postgres;
pub mod redis;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let postgres_url = match std::env::var("POSTGRES_URL") {
        Ok(url) => url,
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "POSTGRES_URL is not set")),
    };

    let _postgres = postgres::Postgres::new(&postgres_url);

    let redis_url = match std::env::var("REDIS_URL") {
        Ok(url) => url,
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "REDIS_URL is not set")),
    };

    let _redis = redis::Redis::new(&redis_url);

    let server = HttpServer::new(|| {
        App::new()
            .service(health)
    });

    let server = if let Ok(server) = server.bind(("0.0.0.0", 3000)) {
        server
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to bind server"));
    };

    server.run().await?;

    Ok(())
}