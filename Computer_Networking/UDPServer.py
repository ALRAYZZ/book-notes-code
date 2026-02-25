from socket import socket, AF_INET, SOCK_DGRAM

serverPort = 12000

serverSocket = socket(AF_INET, SOCK_DGRAM)
serverSocket.bind(('', serverPort))

print('The server is ready to receive')
print("Press Ctrl+C to stop the server.")

try:
    while True:
        message, clientAddress = serverSocket.recvfrom(2048)
        print(f"Received message from {clientAddress}: {message.decode()}")


        modifiedMessage = message.decode().upper()
        serverSocket.sendto(modifiedMessage.encode(), clientAddress)

except KeyboardInterrupt:
    print("\nServer is shutting down.")
    serverSocket.close()