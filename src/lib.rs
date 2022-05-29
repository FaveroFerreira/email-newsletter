use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

pub fn run() -> Result<Server, std::io::Error> {
    let server: Server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(base_endpoint))
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run();

    Ok(server)
}

async fn base_endpoint() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("{ \"status\": \"UP\" }")
}
