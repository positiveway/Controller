use std::net::UdpSocket;

pub fn init_host() -> UdpSocket {
    println!("Host initialized");

    UdpSocket::bind("127.0.0.1:12345").expect("failed to bind host socket")
}

const hostname: &str = "127.0.0.1:1234";

pub fn sendMessageWS(socket: &UdpSocket, message:String){
    if message != "" {
        let bytes_n = message.len().to_string() + ";";
        let message = bytes_n + &*message;

        socket.send_to(message.as_bytes(), hostname)
            .expect("Error on send");
    }
}