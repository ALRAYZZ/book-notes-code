use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::fs;

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

    // Determine path
    // request_line is "GET /file.html HTTP/1.1"
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let path = parts[1];

    // Serve index.html if no file is specified
    let filename = if path == "/" { "index.html" } else { &path[1..] };

    // Read file requested and handle potential 404
    let (status_line, contents) = match fs::read_to_string(&filename) {
        Ok(body) => ("HTTP/1.1 200 OK", body), // status_line, contents get populated here
        Err(_) => ("HTTP/1.1 404 NOT FOUND", String::from("<h1>404 Not Found</h1>")),
    };

    // Send response
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
