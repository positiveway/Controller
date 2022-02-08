use std::net::UdpSocket;

pub fn init_host() -> UdpSocket {
    println!("Host initialized");

    UdpSocket::bind("127.0.0.1:12345").expect("failed to bind host socket")
}

const hostname: &str = "127.0.0.1:1234";

pub fn sendEventsWS(socket: &UdpSocket, events:String) -> std::io::Result<()> {
    socket.send_to(events.as_bytes(), hostname)
        .expect("Error on send");
    Ok(())
}