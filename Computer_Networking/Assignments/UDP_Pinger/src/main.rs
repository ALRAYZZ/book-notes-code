use std::net::UdpSocket;
use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    // "0.0.0.0:0" tells OS to pick any available port and listen on all interfaces
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Set a timeout to avoid thread hanging indefinitely on recv_from
    socket.set_read_timeout(Some(Duration::from_secs(1)))?;

    // Server address and message to send
    let server_addr = "127.0.0.1:12345";
    let message = b"PING";

    // Start recording time
    let start_time = Instant::now();

    // Send ping
    socket.send_to(message, server_addr)?;

    // Buffer to receive response
    let mut buffer = [0u8; 1024];

    // Receive response
    match socket.recv_from(&mut buffer) {
        Ok((amt, src)) => {
            // Calculate RTT
            let duration = start_time.elapsed();
            println!("Reeceived {} bytes from {}: time={:?}", amt, src, duration);
        }
        Err(e) => {
            eprintln!("Failed to receive response: {}", e);
        }
    }

    Ok(())
}
