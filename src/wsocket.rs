use std::net::UdpSocket;

pub fn init_host() -> UdpSocket {

    println!("initializing host");

    let socket = UdpSocket::bind("127.0.0.1:12345").expect("failed to bind host socket");
    // socket.connect(hostname).expect("couldn't connect to address");

    socket
}

pub fn sendEventsWS(socket: &UdpSocket, events:String) -> std::io::Result<()> {
    let hostname = "127.0.0.1:1234";

    socket.send_to(events.as_bytes(), hostname)
        .expect("Error on send");

    // let mut buf = [0; 2048];
    // let (amt, _src) = socket.recv_from(&mut buf)?;
    //
    // let echo = str::from_utf8(&buf[..amt]).unwrap();
    // println!("Echo {}", echo);

    Ok(())
}