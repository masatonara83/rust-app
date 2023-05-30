use std::fmt::format;

use actix_web::{
    get,
    http::{
        header::{ContentType, CONTENT_TYPE},
        StatusCode,
    },
    middleware::Logger,
    post, web,
    web::Json,
    App, HttpResponse, HttpServer, Responder, ResponseError,
};
use std::sync::Mutex;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LUG", "debug");
    std::env::set_var("RUST_BUCKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .route("/hello", web::get().to(|| async { "Hello World!!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
