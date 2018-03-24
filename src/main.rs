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
    // assign variables
    let resp_200 = "HTTP/1.1 200 OK\r\n\r\n";
    let mut buffer = [0; 512];

    // unwrap stream to the alloted buffer size
    stream.read(&mut buffer).unwrap();

    // visual validation / debugging
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let mut index_file = File::open("./index.html").unwrap();
    let mut index_resp = String::new();
    index_file.read_to_string(&mut index_resp).unwrap();

    // write the response to socket
    stream.write(resp_200.as_bytes()).unwrap();
    stream.write(index_resp.as_bytes()).unwrap();
    let _ignore = stream.flush();
}
