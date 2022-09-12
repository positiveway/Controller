use std::net::{UdpSocket};

pub fn init_host() -> UdpSocket {
    println!("Binding socket...");
    UdpSocket::bind("127.0.0.1:12345").expect("failed to bind host socket")
}


pub fn send_message_ws(socket: &UdpSocket, message: &String, hostname: &String) {
    match socket.send_to(message.as_bytes(), hostname) {
        Err(e) => println!("Error sending event via UDP {}", e),
        Ok(r) => {}
    };
}