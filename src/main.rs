use actix_web::{App, HttpRequest, HttpServer, Responder, web};

async fn greetings(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greetings))
            .route("/{name}", web::get().to(greetings))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
