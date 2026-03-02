# Go Back N Protocol Example

import socket

# The receiver.
# Is STRICT, if it gets the packet it expects, sends ACK, else, nothing or out of order
# it ingores it and re sends the ACK for the last successfully received packet.
def run_receiver():
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(('localhost', 5005))

    expected_seq = 0
    print("Receiver is running...")

    while True:
        data, addr = sock.recvfrom(1024)
        seq_num = int(data.decode())

        if seq_num == expected_seq:
            print(f"Received packet with sequence number: {seq_num}")
            # Send cumulative ACK
            sock.sendto(str(expected_seq).encode(), addr)
            expected_seq += 1
        else:
            print(f"Got {seq_num}, expected {expected_seq}. Discarding packet.")
            # Re send ACK for the las successfully received packet
            sock.sendto(str(expected_seq - 1).encode(), addr)


# The sender.
def run_sender():
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    # The "retransmission timeout"
    # if we don't get an ACK in 1 second, we will resend the packet.
    sock.settimeout(1)  
    dest = ('localhost', 5005)

    window_size = 4
    base = 0
    next_to_send = 0
    total_packets = 10

    while base < total_packets:
        # Fill the window
        while next_to_send < base + window_size and next_to_send < total_packets:
            print(f"Sending packet {next_to_send}")
            sock.sendto(str(next_to_send).encode(), dest)
            next_to_send += 1

    # Wait for ACKs
    try:
        data, addr = sock.recvfrom(1024)
        ack = int(data.decode())
        print(f"Received ACK for packet {ack}")

        if ack >= base:
            base = ack + 1 # Move the window forward

    except socket.timeout:
        print("Timeout. Going back to base", base, "--")
        next_to_send = base  # The "Go Back N" part: we go back to resend from the base packet.


    print("All packets sent and acknowledged.")