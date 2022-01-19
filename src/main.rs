mod uinput_direct;
mod mouse_move;
mod match_events;

extern crate partial_application;

use gilrs::{Gilrs, Button, Event, EventType::*, Axis, Gamepad, GamepadId};
use std::{thread, thread::sleep, time::Duration};
use std::collections::{HashMap};
use std::sync::{Arc, Mutex, MutexGuard};
use lazy_static::lazy_static;
use cached::proc_macro::cached;

use uinput::Device;
use uinput::event::ButtonsVec;
use uinput::event::keyboard::Key;
use crate::mouse_move::*;
use crate::match_events::*;

type ButtonsMap = HashMap<Button, ButtonsVec>;

const DEBUG: bool = false;

macro_rules! debug {
    ($($arg:tt)*) => {
        if DEBUG{
            println!($($arg)*);
        }
    };
}
pub(crate) use debug;    // <-- the trick

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

    pub static ref commands_mode_mutex:Mutex<bool> = Mutex::new(true);

    static ref fake_device:Device = Device::init_mouse_keyboard();
}

pub fn get_mapping() -> &'static ButtonsMap {
    let commands_mode = commands_mode_mutex.lock().unwrap();
    match *commands_mode {
        true => &CommandsMap,
        false => &TypingMap,
    }
}

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (id, gamepad) in gilrs.gamepads() {
        println!("id {}: {} is {:?}", id, gamepad.name(), gamepad.power_info());
    }
    print_deadzones(&gilrs, 0);

    spawn_mouse_thread();
    spawn_scroll_thread();

    let mut gilrs = Gilrs::new().unwrap();

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            debug!("{:?} device id {}", event, id);

            let mapping = get_mapping();

            match event {
                ButtonPressed(button, code) | ButtonReleased(button, code) => {
                    process_btn_press_release(event, button, mapping);
                }
                ButtonChanged(button, value, code) => {
                    process_btn_change(button, value, mapping);
                }
                AxisChanged(axis, value, code) => {
                    process_axis(axis, value);
                }
                _ => debug!("Action handling is omitted"),
            }
        }
        sleep(Duration::from_millis(25));
    }
}
