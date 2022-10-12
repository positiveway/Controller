use std::{fs, io, thread::sleep, time::Duration};
use std::env::current_exe;
use std::net::{ToSocketAddrs, UdpSocket};
use std::path::PathBuf;

use gilrs::{Event, EventType::*, Gilrs};

use crate::deadzones::*;
use crate::match_events::*;
use crate::websocket::*;

mod deadzones;
mod match_events;
mod websocket;

fn read_send_events(gilrs: &mut Gilrs, socket: &UdpSocket, hostname: &String) {
    print_deadzones(gilrs, 0);
    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            let (button_or_axis, res_value, event_type, code) = match_event(&event);

            let event_as_str = format!("{event_type}{button_or_axis}{res_value};{code}");
            // println!("{}", &event_as_str);
            send_message_ws(&socket, &event_as_str, &hostname);

            if event == Disconnected {
                println!("Gamepad disconnected");
                return;
            }
        }
        sleep(Duration::from_millis(4)); //4 = USB min latency
    }
}

fn get_filepath() -> io::Result<PathBuf> {
    let mut dir = current_exe()?;
    dir.pop();
    dir.pop();
    dir.pop();
    dir.push("hostname.txt");
    Ok(dir)
}

fn read_hostname() -> String {
    let filename = get_filepath().expect("Hostname file is not found");
    println!("Settings filepath: {}", filename.display());

    let raw_hostname = fs::read_to_string(filename)
        .expect("Cannot read hostname from file");

    let strip_hostname = raw_hostname.trim();

    if strip_hostname == "" {
        panic!("hostname cannot be empty");
    }
    let hostname = format!("{}:{}", strip_hostname, PORT);

    println!("Hostname: {}", hostname);

    hostname.to_socket_addrs().expect("Invalid hostname");
    return hostname;
}

const PORT: &str = "1234";

fn main() {
    let hostname = read_hostname();

    let socket = init_socket();
    let mut gilrs = Gilrs::new().unwrap();

    let mut is_wait_msg_printed = false;
    loop {
        gilrs = Gilrs::new().unwrap();
        let mut gamepads_counter = 0;
        for (id, gamepad) in gilrs.gamepads() {
            gamepads_counter += 1;
            println!("id {}: {} is {:?}", id, gamepad.name(), gamepad.power_info());
        }

        if gamepads_counter == 0 {
            if !is_wait_msg_printed {
                is_wait_msg_printed = true;
                println!("Gamepad is not connected. Waiting...");
            }
        } else if gamepads_counter > 1 {
            println!("Only one gamepad is supported. Disconnect other gamepads");
        } else {
            is_wait_msg_printed = false;
            read_send_events(&mut gilrs, &socket, &hostname)
        }
        sleep(Duration::from_millis(5000));
    }
}
