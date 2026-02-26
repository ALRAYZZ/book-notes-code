use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    // Connect to mail server
    let mut stream = TcpStream::connect("smtp.mailtrap.io:2525")?;
    let mut reader = BufReader::new(stream.try_clone()?);

    // Helper to read server response
    let mut response = String::new();
    reader.read_line(&mut response)?;
    println!("Sever: {}", response);

    // Say hello (EHLO)
    send_command(&mut stream, &mut reader, "EHLO localhost\r\n")?;

    // Define Sender
    send_command(&mut stream, &mut reader, "MAIL FROM:<me@example.com\r\n")?;

    // Define recipient
    send_command(&mut stream, &mut reader, "RCPT TO:<friend@example.com\r\n")?;

    // Send data
    send_command(&mut stream, &mut reader, "DATA\r\n")?;

    // Email body
    let message = "Subject: Hello from Rust!\r\n\r\nThis is a test email sent using Rust.\r\n.\r\n";
    send_command(&mut stream, &mut reader, message)?;

    // Quit and close
    send_command(&mut stream, &mut reader, "QUIT\r\n")?;

    Ok(())
}

fn send_command(stream: &mut TcpStream, reader: &mut BufReader<TcpStream>, command: &str) -> std::io::Result<()> {
    stream.write_all(command.as_bytes())?;
    let mut response = String::new();
    reader.read_line(&mut response)?;
    println!("Sent: {} | Received: {}", command.trim(), response.trim());
    Ok(())
}