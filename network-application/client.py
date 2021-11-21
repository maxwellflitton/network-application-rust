import socket
from enum import Enum


class OperationType(Enum):
    INSERT = (1).to_bytes(4, byteorder='big')
    GET = (2).to_bytes(4, byteorder='big')
    DELETE = (3).to_bytes(4, byteorder='big')


TCP_IP = '127.0.0.1'
TCP_PORT = 8080
BUFFER_SIZE = 1024
MESSAGE = OperationType.INSERT.value

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((TCP_IP, TCP_PORT))
s.send(MESSAGE)
data = s.recv(BUFFER_SIZE)
s.close()
print("received data: ", data)
