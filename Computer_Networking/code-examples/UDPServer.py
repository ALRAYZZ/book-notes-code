from socket import socket, AF_INET, SOCK_DGRAM

serverPort = 12000

serverSocket = socket(AF_INET, SOCK_DGRAM)
serverSocket.bind(('', serverPort))
serverSocket.settimeout(1)

print('The server is ready to receive')
print("Press Ctrl+C to stop the server.")

try:
    while True:
        try:
            message, clientAddress = serverSocket.recvfrom(2048)
            serverSocket.settimeout(None)
            print(f"Received message from {clientAddress}: {message.decode()}")


            modifiedMessage = message.decode().upper()
            serverSocket.sendto(modifiedMessage.encode(), clientAddress)
        except TimeoutError:
            continue

except KeyboardInterrupt:
    print("\nServer is shutting down.")
    serverSocket.close()