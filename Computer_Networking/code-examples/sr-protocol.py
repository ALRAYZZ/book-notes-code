# Selective Repeat Protocol Example

import socket

# The receiver.
# This receiver doesnt throw aways out of order packets,
#  it buffers them until they can be delivered in order.
def run_receiver():
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind(('localhost', 5005))

    expected_seq = 0
    buffer = {} # The memory that GBN doesnt have
    print("Receiver is running...")

    while True:
        data, addr = sock.recvfrom(1024)
        seq_num = int(data.decode())

        # Individual ACK (even out of order)
        print(f"Received packet with sequence number: {seq_num}")
        sock.sendto(str(seq_num).encode(), addr)

        # If its the expected, slide window
        if seq_num == expected_seq:
            print(f"Accepting {seq_num}")
            expected_seq += 1
            # Deliver any buffered packets that now become in order
            while expected_seq in buffer:
                print(f"Releasing {expected_seq} from buffer")
                del buffer[expected_seq]
                expected_seq += 1
        elif seq_num > expected_seq:
            # Buffer out of order packets
            print(f"Buffering out of order packet {seq_num}")
            buffer[seq_num] = True


# The sender.
def run_sender():
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.settimeout(1)
    dest = ('localhost', 5005)

    window_size = 4
    base = 0
    total_packets = 10
    # track which packets in window have been ACKed
    acked = [False] * total_packets
    sent_time = {} # Simplified tracking individual timeouts

    while base < total_packets:
        # Send all packets in window that have not been sent or ACKed
        for i in range(base, min(base + window_size, total_packets)):
            if not acked[i]:
                print(f"Sending/Resending packet {i}")
                sock.sendto(str(i).encode(), dest)

        # Listen for individual ACKs
        try:
            while True:
                data, addr = sock.recvfrom(1024)
                ack_num = int(data.decode())
                print(f"Received ACK for packet {ack_num}")
                acked[ack_num] = True

                # Slide base if odles packet was ACKed
                while base < total_packets and acked[base]:
                    base += 1
        except socket.timeout:
            print("Timeout. Resending unACKed packets in window starting from base", base)