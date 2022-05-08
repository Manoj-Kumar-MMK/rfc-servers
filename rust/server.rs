use std::io::Write;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:4002").unwrap();
    println!("Server started on port {}", "4002");
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {

    let mut file = fs::OpenOptions::new()
      .write(true)
      .append(true)
      .open("output.txt")
      .unwrap();
      
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let val:&str =&String::from_utf8_lossy(&buffer[..]).trim_matches(char::from(0)).to_string();
    let split:Vec<&str> = val.split("\r\n\r\n").collect();
    let headers=split[0];
    let body=split[1];
    let body_length=body.len();
   
    write!(file,"<-------------\nRequest-Headers: {}\n\nBody : {}\n\nBody length : {}\n--------------->\n", headers,body,body_length);
    let contents=format!("Body : {}\n\nBody length : {}",body,body_length);
    let response=format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

