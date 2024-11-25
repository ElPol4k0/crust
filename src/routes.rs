use actix_web::{web, Responder, HttpResponse};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Lightweight Webserver")
}

pub async fn hello(name: web::Path<(String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}