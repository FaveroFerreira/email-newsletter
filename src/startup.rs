use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub fn run(pg_pool: Pool<Postgres>, listener: TcpListener) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(pg_pool);

    let server: Server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
