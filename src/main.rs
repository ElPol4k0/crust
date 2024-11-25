use actix_web::{web, App, HttpServer, Responder};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::index))
            .route("/hello/{name}", web::get().to(routes::hello))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
