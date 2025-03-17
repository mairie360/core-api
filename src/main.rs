pub mod postgres;
pub mod redis;
pub mod schema;
pub mod routes;
pub mod handlers;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use actix_web::middleware::Logger;
use env_logger::Env;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

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

    let postgres = postgres::Postgres::new(&postgres_url);

    let postgres = web::Data::new(postgres);

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    let mut postgres_connection = postgres.get_connection();

    match postgres_connection.run_pending_migrations(MIGRATIONS) {
        Ok(_) => {}
        Err(error) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, error.to_string()));
        }
    }

    let redis_url = match std::env::var("REDIS_URL") {
        Ok(url) => url,
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "REDIS_URL is not set")),
    };

    let redis = redis::Redis::new(&redis_url);

    let redis = web::Data::new(redis);

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(postgres.clone())
            .app_data(redis.clone())
            .wrap(Logger::default())
            .service(health)
            .configure(routes::modules::config)
            .configure(routes::web_instances::config)
            .configure(routes::api_instances::config)
    });

    let server = if let Ok(server) = server.bind(("0.0.0.0", 3000)) {
        server
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to bind server"));
    };

    server.run().await?;

    Ok(())
}