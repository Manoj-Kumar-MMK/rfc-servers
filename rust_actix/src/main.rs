use std::io::Write;
use std::fs;
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use std::collections::HashMap;


async fn handler(req:HttpRequest,body:web::Bytes)  -> HttpResponse  {
    let headers_map=req.headers();
    let mut headers = HashMap::new();
    for (k, v) in headers_map {
        let k = k.as_str().to_owned();
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        headers.entry(k).or_insert_with(Vec::new).push(v)
    }

    let body=String::from_utf8_lossy(&body).into_owned();
    let body_length=body.len();
     
  
  let mut file = fs::OpenOptions::new()
      .write(true)
      .append(true)
      .open("output.txt")
      .unwrap();
    
    write!(file,"<-------------\nRequest-Headers: {:?}\n\nBody : {}\n\nBody length : {}\n--------------->\n", headers,body,body_length);
    let contents=format!("Body : {}\n\nBody length : {}",body,body_length);

      HttpResponse::Ok()
        .content_type("application/json")
        .body(contents)
      }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(handler)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
