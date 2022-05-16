//use std::io::Write;
//use std::fs;
use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
fn main() -> std::io::Result<()>{
  HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
        .bind(("127.0.0.1", 4003))?
        .run()
        .await
}
