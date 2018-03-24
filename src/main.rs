use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection Established");
        hand_conn(stream);
    }
}

fn hand_conn(mut stream: TcpStream) {
    let resp_200 = "HTTP/1.1 200 OK\r\n\r\n";
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    stream.write(resp_200.as_bytes()).unwrap();
    stream.flush();
}
