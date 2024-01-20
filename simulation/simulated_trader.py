import socket

host = socket.gethostname()
port = 7878                   # The same port as used by the server
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((host, port))
while True:
    req = input("Trade? ")

    if req == "kill":
        break

    s.sendall(str.encode(req))
    data = s.recv(1024)

    print('Received', str(repr(data)))

s.close()
