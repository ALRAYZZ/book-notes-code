use std::net::UdpSocket;
use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    // "0.0.0.0:0" tells OS to pick any available port and listen on all interfaces
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    // Set a timeout to avoid thread hanging indefinitely on recv_from
    socket.set_read_timeout(Some(Duration::from_secs(1)))?;

    // Since using UDP, most servers will drop packet or wont respond to unknown clients
    // Lets try Google DNS server which is known to respond to pings on port 53

    let target = "8.8.8.8:53";

    // Minimal 12-byte DNS header (enough to trigger response)
    // This is saying: "Hey are you there?" in DNS query format
    let dns_query = [
        0xAA, 0xAA, // Transaction ID
        0x01, 0x00, // Flags (standard query)
        0x00, 0x01, // Questions (1)
        0x00, 0x00, // Answer RRs
        0x00, 0x00, // Authority RRs
        0x00, 0x00, // Additional RRs
    ];

    // Start recording time
    let start_time = Instant::now();

    // Send ping
    socket.send_to(&dns_query, target)?;

    // Buffer to receive response
    let mut buffer = [0u8; 1024];

    // Receive response
    match socket.recv_from(&mut buffer) {
        Ok((amt, src)) => {
            // Calculate RTT
            let duration = start_time.elapsed();
            println!("UDP Ping to {} successful! RTT: {:?}", target, duration);
        }
        Err(e) => {
            eprintln!("Failed to receive response: {}", e);
        }
    }

    Ok(())
}
