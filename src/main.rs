use actix_web::{App, HttpServer, Responder, HttpResponse, get};
use std::fs::File;
use std::io::Read;

use krc_rs::parser;
use actix_web::error::DispatchError::H2;
use actix_web::body::{Body};
use actix_web::web::Bytes;
use actix_cors::Cors;

#[get("/")]
async fn root() -> impl Responder {
    let mut buf = String::new();
    File::open("assets/index.html").unwrap().read_to_string(&mut buf);
    HttpResponse::Ok().body(buf)
}

#[get("/lyric")]
async fn lyric() -> impl Responder {
    let mut buf = Vec::<u8>::new();
    File::open("assets/test.krc").unwrap().read_to_end(&mut buf);
    HttpResponse::Ok().body(parser::parse(buf.as_slice(), false).unwrap())
}

#[get("/song")]
async fn song() -> HttpResponse {
    let mut buf = Vec::<u8>::new();
    File::open("assets/test.mp3").unwrap().read_to_end(&mut buf);
    HttpResponse::Ok().content_type("audio/mpeg").await.unwrap().set_body(Body::Bytes(Bytes::from(buf)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin();
        App::new()
            .wrap(cors)
            .service(root)
            .service(lyric)
            .service(song)
    })
        .bind("127.0.0.1:8080").and_then(|it| {
            println!("Server runs at port 8080");
            Ok(it)
        })?
        .run()
        .await
}