use crate::routes::{health_check, transaction};
use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/transactions", web::post().to(transaction))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
