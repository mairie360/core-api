use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get the binding address from the environment
    let binding_address = match std::env::var("BINDING_ADDRESS") {
        Ok(address) => address,
        Err(_) => "127.0.0.1".to_string(),
    };

    // Get the binding port from the environment
    let binding_port = match std::env::var("BINDING_PORT") {
        Ok(port) => port,
        Err(_) => "3000".to_string(),
    };

    let binding_port = match binding_port.parse::<u16>() {
        Ok(port) => port,
        Err(_) => 3000,
    };

    let server = HttpServer::new(|| {
        App::new()
            .service(health)
    });

    let server = if let Ok(server) = server.bind((binding_address, binding_port)) {
        server
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to bind server"));
    };

    server.run().await?;

    Ok(())
}