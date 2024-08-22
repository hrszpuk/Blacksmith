use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Setting up blacksmith server");

    HttpServer::new(|| {
        App::new()
            .service(home)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}
