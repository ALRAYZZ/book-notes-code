struct Header {
    seq: u32,
    ack: u32,
    flags: u8,
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

fn main() {
    println!("Hello, world!");
}
