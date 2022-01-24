mod uinput_direct;
mod mouse_move;
mod match_events;
mod struct_statics;
mod wsocket;

extern crate partial_application;

use gilrs::{Gilrs, Button, Event, EventType::*, Axis, Gamepad, GamepadId, EventType};
use std::{thread, thread::sleep, time::Duration};
use std::collections::{HashMap};
use std::fmt::Debug;
use std::sync::{Arc, Mutex, MutexGuard};
use lazy_static::lazy_static;
use cached::proc_macro::cached;

use uinput::Device;
use uinput::event::ButtonsVec;
use uinput::event::keyboard::Key;
use crate::struct_statics::*;
use crate::mouse_move::*;
use crate::match_events::*;
use crate::wsocket::*;



lazy_static! {
    static ref copy_key: ButtonsVec = vec![Key::LeftControl,Key::C];
    static ref paste_key: ButtonsVec = vec![Key::LeftControl, Key::V];

    static ref CommandsMap: ButtonsMap = HashMap::from([
        (Button::DPadDown, vec![Key::Down]),
        (Button::DPadUp, vec![Key::Up]),
        (Button::DPadLeft, vec![Key::Left]),
        (Button::DPadRight, vec![Key::Right]),
        (Button::RightTrigger2, vec![Key::LeftMouse]),
        (Button::LeftTrigger2, vec![Key::RightMouse]),
        (Button::RightTrigger, paste_key.to_vec()),
        (Button::LeftTrigger, copy_key.to_vec()),
        (Button::West, vec![Key::Enter]),
        (Button::North, vec![Key::Space]),
        (Button::South, vec![Key::BackSpace]),
        (Button::East, vec![Key::LeftMeta]),
    ]);

    static ref TypingMap: ButtonsMap = {
        let typing_map = CommandsMap.clone();
        typing_map
    };
}

pub fn get_mapping() -> &'static ButtonsMap {
    let commands_mode = commands_mode_mutex.lock().unwrap();
    match *commands_mode {
        true => &CommandsMap,
        false => &TypingMap,
    }
}

fn match_button(button:&Button) -> &str{
    match button {
        Button::South => "S",
        Button::East => "E",
        Button::North => "N",
        Button::West => "W",
        Button::C => "C",
        Button::Z => "Z",
        Button::LeftTrigger => "L",
        Button::LeftTrigger2 => "L2",
        Button::RightTrigger => "R",
        Button::RightTrigger2 => "R2",
        Button::Select => "Se",
        Button::Start => "St",
        Button::Mode => "M",
        Button::LeftThumb => "LT",
        Button::RightThumb => "RT",
        Button::DPadUp => "DU",
        Button::DPadDown => "DD",
        Button::DPadLeft => "DL",
        Button::DPadRight => "DR",
        Button::Unknown => "U",
    }
}

fn match_axis(axis:&Axis) -> &str{
    match axis {
        Axis::LeftStickX => "LX",
        Axis::LeftStickY => "LY",
        Axis::LeftZ => "LZ",
        Axis::RightStickX => "RX",
        Axis::RightStickY => "RY",
        Axis::RightZ => "RZ",
        Axis::DPadX => "DX",
        Axis::DPadY => "DY",
        Axis::Unknown => "U",
    }
}

fn match_event(event:&EventType) -> (&str, String, &str){
    let mut button_or_axis = "No";
    let mut res_value:f32 = 0.0;
    let mut event_type = "";

    match event {
        EventType::AxisChanged(axis,value,  code) => {
            event_type = "A";
            res_value = *value;
            button_or_axis = match_axis(axis);
        },
        EventType::ButtonChanged(button,value,  code) => {
            event_type = "B";
            res_value = *value;
            button_or_axis = match_button(button);
        },
        EventType::ButtonReleased(button,  code) => {
            event_type = "Rl";
            button_or_axis = match_button(button);

        },
        EventType::ButtonPressed(button,  code) => {
            event_type = "P";
            button_or_axis = match_button(button);
        },
        EventType::ButtonRepeated(button,  code) => {
            event_type = "Rp";
            button_or_axis = match_button(button);

        },
        EventType::Connected => {
            event_type = "C"
        },
        EventType::Disconnected => {
            event_type = "D"
        },
        EventType::Dropped => {
            event_type = "Dr"
        },
    };
    let res_value = res_value.to_string();
    return (button_or_axis, res_value, event_type);
}


fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (id, gamepad) in gilrs.gamepads() {
        println!("id {}: {} is {:?}", id, gamepad.name(), gamepad.power_info());
    }
    print_deadzones(&gilrs, 0);

    // spawn_mouse_thread();
    // spawn_scroll_thread();

    let mut gilrs = Gilrs::new().unwrap();
    let socket = init_host();

    loop {
        let mut message = String::from("");

        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            let device_id = id.to_string();
            let device_id = &device_id[..];
            let (button_or_axis, res_value, event_type) = match_event(&event);

            let event_as_str = format!("{device_id},{res_value},{event_type},{button_or_axis};");
            debug!("{}", {&event_as_str});
            message.push_str(&*event_as_str);
            // sendEventsWS(&socket, event_as_str).unwrap();
            // sendEventsWS(&socket, String::from("ax")).unwrap();

            // debug!("{:?} device id {}", event, id);
        }
        if message != ""{
            // message.push_str(lots_of_spaces);
            sendEventsWS(&socket, message).unwrap();
        }
        sleep(Duration::from_millis(25));
    }
}
