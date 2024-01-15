use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let request_str = match str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(_) => return,
    };

    let response = if request_str.starts_with("GET /hello ") {
        "HTTP/1.1 200 OK\r\n\r\nHello, World!"
    } else if request_str.starts_with("GET /bye ") {
        "HTTP/1.1 200 OK\r\n\r\nGoodbye, World!"
    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\nNot Found"
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
