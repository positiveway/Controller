mod deadzones;
mod match_events;
mod wsocket;

use gilrs::{Gilrs, Button, Event, EventType::*, Axis, Gamepad, GamepadId, EventType};
use std::{thread, thread::sleep, time::Duration};
use crate::wsocket::*;
use crate::deadzones::*;
use crate::match_events::*;


fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let socket = init_host();

    let mut gamepads_counter = 0;
    for (id, gamepad) in gilrs.gamepads() {
        gamepads_counter += 1;
        println!("id {}: {} is {:?}", id, gamepad.name(), gamepad.power_info());
    }

    if gamepads_counter == 0{
        println!("Connect gamepad and relaunch program");
        return;
    } else if gamepads_counter > 1 {
        println!("Only one gamepad is supported. Disconnect other gamepads");
        return;
    }

    print_deadzones(&gilrs, 0);
    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            let (button_or_axis, res_value, event_type) = match_event(&event);

            let event_as_str = format!("{event_type},{button_or_axis},{res_value}");
            // println!("{}", &event_as_str);
            sendMessageWS(&socket, event_as_str);
        }
        sleep(Duration::from_millis(4)); //4 = USB min latency
    }
}
