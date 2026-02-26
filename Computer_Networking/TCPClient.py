from socket import socket, AF_INET, SOCK_STREAM 

severName = 'servername'
severPort = 12000

clientSocket = socket(AF_INET, SOCK_STREAM)
# Creating TCP connection, a thing we dont do in UDP
clientSocket.connect((severName, severPort))

sentence = input('Input lowercase sentence: ')
clientSocket.send(sentence.encode())

# Receive the modified sentence from the server
modifiedSentence = clientSocket.recv(1024)
print('From Server: ', modifiedSentence.decode())

clientSocket.send("Modified sentence received".encode())

clientSocket.close()