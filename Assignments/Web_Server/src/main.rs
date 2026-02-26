use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    // Usually HTTP is port 80, but we might face permissions issues
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    let (stream, addr) = listener.accept().expect("Failed to accept connection");

    println!("Connection established from {}", addr);

    handle_connection(stream);
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // Read 1st line of the HTTP request
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request received: {}", request_line);
}
