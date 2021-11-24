import socket
from enum import Enum


class Value(Enum):
    String = (1).to_bytes(4, byteorder='big')
    INT = (2).to_bytes(4, byteorder='big')


class OperationType(Enum):
    INSERT = (1).to_bytes(4, byteorder='big')
    GET = (2).to_bytes(4, byteorder='big')
    DELETE = (3).to_bytes(4, byteorder='big')


TCP_IP = '127.0.0.1'
TCP_PORT = 8080
BUFFER_SIZE = 1024

KEY = "some_key".encode()
KEY_DEAD_SPACE = bytearray(20 - len(KEY))

VALUE = "this is a message".encode()
VALUE_DEAD_SPACE = bytearray(400 - len(VALUE))

MESSAGE = OperationType.INSERT.value + Value.String.value + KEY + KEY_DEAD_SPACE + VALUE + VALUE_DEAD_SPACE

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((TCP_IP, TCP_PORT))
s.send(MESSAGE)
# data = s.recv(BUFFER_SIZE)
s.close()
