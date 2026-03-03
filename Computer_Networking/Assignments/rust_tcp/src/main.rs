struct Header {
    seq: u32,
    ack: u32,
    flags: u8,
}

impl Header {
    fn to_bytes(&self) -> [u8; 9] {
        let mut buf = [0u8; 9];
        buf[..4].copy_from_slice(&self.seq.to_be_bytes());
        buf[4..8].copy_from_slicee(&self.ack.to_be_bytes());
        buf[8] = self.flags;
        buf
    }

    fn from_bytes(buf: &[u8]) -> Self {
        let seq = u32::from_be_bytes(buf[..4].try_into().unwrap());
        let ack = u32::from_be_bytes(buf[4..8].try_into().unwrap());
        let flags = buf[8];

        Header { seq, ack, flags }
    }
}

const SYN: u8 = 1;
const ACK: u8 = 2;
const FIN: u8 = 4;

enum State {
    Listen,
    SynSent,
    SynReceived,
    Established,
}

fn run_server() {
    let socket = UdpSocket::bind("127.0.0.1:4000").expect("bind failed");
    println!("[SERVER] Server listening on 4000");

    let mut state = State::Listen;
    let mut buf = [0u8; 1024];

    loop {
        let (size, src) = socket.recv_from(&mut buf).unwrap();
        let header = Header::from_bytes(&buf[..size]);

        match state {
            State::Listen => {
                if header.flags & SYN != 0 {
                    println!("[SERVER] Received SYN");

                    let reply = Header {
                        seq: 100,
                        ack: header.seq + 1,
                        flags: SYN | ACK,
                    };

                    socket.send_to(&reply.to_bytes(), src).unwrap();
                    println!("[SERVER] Sent SYN-ACK");

                    state = State::SynReceived;
                }
            }

            State::SynReceived => {
                if header.flags & ACK != 0 {
                    println!("[SERVER] Received ACK");
                    state = State::Established;
                    println!("[SERVER] Connection established");
                    break;
                }
            }

            _ => {}
        }
    }
}

fn run_client() {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("bind failed");
    socket.set_read_timeout(Some(Duration::from_secs(5))).unwrap();

    let server_addr = "127.0.0.1:4000";

    let mut state = State::SynSent;

    let syn = Header {
        seq: 1,
        ack: 0,
        flags: SYN,
    };

    socket.send_to(&syn.to_bytes(), server_addr).unwrap();
    println!("[CLIENT] Sent SYN");

    let mut buf = [0u8; 1024];

    let (size, _) = socket.recv_from(&mut buf).unwrap();
    let header = Header::from_bytes(&buf[..size]);

    if header.flags & SYN != 0 && header.flags & ACK != 0 {
        println!("[CLIENT] Received SYN-ACK");

        let ack = Header {
            seq: 2,
            ack: header.seq + 1,
            flags: ACK,
        };

        socket.send_to(&ack.to_bytes(), server_addr).unwrap();
        println!("[CLIENT] Sent ACK");

        state = State::Established;
        println!("[CLIENT] Connection established");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Use: server | client");
        return;
    }

    // Allows for same exec to run client logic or server logic
    match args[1].as_str() {
        "server" => run_server(),
        "client" => run_client(),
        _ => println!("Invalid argument"),
    }
}
