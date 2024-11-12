use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            //.route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet))
            .route(
                "/health_check",
                web::get().to(routes::health_check::health_check),
            )
            // A new entry in our routing table for POST /subscriptions requests
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    //removed .await

    Ok(server)
}
