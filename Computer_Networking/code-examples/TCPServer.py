
from socket import socket, AF_INET, SOCK_STREAM 

serverPort = 12000

serverSocket = socket(AF_INET, SOCK_STREAM)
serverSocket.bind(('', serverPort))
serverSocket.listen(1)
serverSocket.settimeout(1)  
print('The server is ready to receive')
print("Press Ctrl+C to stop the server.")

try:
    while True:
        try:
            connectionSocket, addr = serverSocket.accept()
            connectionSocket.settimeout(None)

            sentence = connectionSocket.recv(1024).decode()
            capitalizedSentence = sentence.upper()
            connectionSocket.send(capitalizedSentence.encode())
            print(f"Received message from {addr}: {sentence}")
            connectionSocket.close()
        except TimeoutError:
            continue

except KeyboardInterrupt:
    print("\nServer stopped.")
    serverSocket.close()



serverSocket.close()