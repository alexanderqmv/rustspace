use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use std::path::Path;

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    let path = Path::new("src/web/index.html");
    Ok(NamedFile::open(path)?)
}

#[get("/hello")]
async fn hello() -> actix_web::Result<NamedFile> {

    let path = Path::new("src/web/index.html");
    Ok(NamedFile::open(path)?)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
