use std::net::UdpSocket;

pub fn init_host() -> UdpSocket {
    println!("Binding socket...");
    UdpSocket::bind("127.0.0.1:12345").expect("failed to bind host socket")
}

const PORT: &str = ":1234";

pub fn send_message_ws(socket: &UdpSocket, message: String, hostname: &String) {
    let full_address = hostname.to_owned() + PORT;
    socket.send_to(message.as_bytes(), full_address)
        .expect("Error sending event via UDP");
}