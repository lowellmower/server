extern crate server;
use server::ThreadPool;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::fs::File;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| { hand_conn(stream); });
    }
}

fn hand_conn(mut stream: TcpStream) {
    let resp_200 = "HTTP/1.1 200 OK\r\n\r\n";
    let resp_404 = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let get_pigs = b"GET /pigs HTTP/1.1\r\n";

    let (code, file) = if buffer.starts_with(get) {
        (resp_200, "index.html")
    } else if buffer.starts_with(get_pigs) {
        thread::sleep(Duration::from_secs(6));
        (resp_200, "pigs.html")
    } else {
        (resp_404, "404.html")
    };

    let resp = build_response(code.to_string(), file.to_string());
    write_response(stream, resp);
}

fn write_response(mut stream: TcpStream, resp: String) {
    stream.write(resp.as_bytes()).unwrap();
    let _ignore = stream.flush();
}

fn build_response(code: String, file: String) -> String {
    let mut resp_file = File::open(file).unwrap();
    let mut resp_body = String::new();
    resp_file.read_to_string(&mut resp_body).unwrap();
    let response = format!("{}{}", code, resp_body);
    response
}
