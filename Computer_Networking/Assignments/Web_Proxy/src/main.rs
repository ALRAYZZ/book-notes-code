use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
}

fn handle_client(mut client_stream: TcpStream) {
    let mut buffer = [0; 4096];

    // Read the request from the browser
    client_stream.read(&mut buffer).unwrap();

    // Parse buffer to find destination host
    let mut server_stream = TcpStream::connect("93.184.216.34:80")
        .expect("Failed to connect to origin");

    // Forward the request to the origin server
    server_stream.write(&buffer).unwrap();

    // Read the response from the origin server
    let mut response_buffer = [0; 4096];
    server_stream.read(&mut response_buffer).unwrap();

    // Send that response back to the browser
    client_stream.write_all(&response_buffer).unwrap();
}
