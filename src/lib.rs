use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct FormData {
    name: String,
    email: String,
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
