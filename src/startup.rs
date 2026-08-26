use crate::routes::{health_check, signup, transaction};
use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

pub async fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(pool.clone()))
            .route("/health_check", web::get().to(health_check))
            .route("/transactions", web::post().to(transaction))
            .route("signup", web::post().to(signup))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
