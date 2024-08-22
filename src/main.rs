use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(serde::Deserialize)]
struct RunInfo {
    code: String,
    lang: String,
    version: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Setting up blacksmith server");

    HttpServer::new(|| {
        App::new()
            .service(get_run)
            .service(post_run)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/run")]
async fn get_run() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[post("/run")]
async fn post_run(info: web::Json<RunInfo>) -> impl Responder {
    HttpResponse::Ok().body(format!("Code: {}, Lang: {}, Version: {}", info.code, info.lang, info.version))
}
