mod deadzones;
mod match_events;
mod wsocket;

use gilrs::{Gilrs, Button, Event, EventType::*, Axis, Gamepad, GamepadId, EventType};
use std::{thread, thread::sleep, time::Duration};
use std::fmt::Debug;
use crate::wsocket::*;
use crate::deadzones::*;
use crate::match_events::*;


fn main() {
    let gamepad_connected_msg = String::from("gamepadConnected;");

    let mut gilrs = Gilrs::new().unwrap();
    let socket = init_host();

    println!("Waiting for gamepad connection");
    let mut gamepad_disconnected = true;
    while gamepad_disconnected {
        gilrs = Gilrs::new().unwrap();
        // Iterate over all connected gamepads
        for (id, gamepad) in gilrs.gamepads() {
            gamepad_disconnected = false;
            println!("id {}: {} is {:?}", id, gamepad.name(), gamepad.power_info());
        }
        sleep(Duration::from_millis(25));
    }

    print_deadzones(&gilrs, 0);
    sendMessageWS(&socket, gamepad_connected_msg);

    loop {
        let mut message = String::from("");

        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            let device_id = id.to_string();
            let device_id = &device_id[..];
            let (button_or_axis, res_value, event_type) = match_event(&event);

            let event_as_str = format!("{device_id},{event_type},{button_or_axis},{res_value};");
            // println!("{}", &event_as_str);
            // sendMessageWS(&socket, event_as_str);
            message.push_str(&*event_as_str);
        }
        sendMessageWS(&socket, message);
        sleep(Duration::from_millis(4));
    }
}
