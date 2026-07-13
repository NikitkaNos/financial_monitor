use crate::routes::{health_check, transaction};
use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgPool;

pub async fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health_check", web::get().to(health_check))
            .route("/transactions", web::post().to(transaction))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
