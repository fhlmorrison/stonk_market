use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream, UdpSocket},
};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod common;
mod engine;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
