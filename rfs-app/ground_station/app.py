import socket

def run_ground_station():
    server_address = ("0.0.0.0", 9999)
    
    udp_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    udp_socket.bind(server_address)
    
    print(f"Ground station listening on {server_address[0]}:{server_address[1]}...")

    try:
        while True:
            data, addr = udp_socket.recvfrom(1024)
            print(f"Received message from {addr}: {data.decode('utf-8')}")
    except KeyboardInterrupt:
        print("Ground station shutting down...")
    finally:
        udp_socket.close()


if __name__ == "__main__":
    run_ground_station()
