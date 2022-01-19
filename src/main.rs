mod uinput_direct;
mod mouse_move;

extern crate partial_application;

use gilrs::{Gilrs, Button, Event, EventType::*, Axis, Gamepad, GamepadId};
use std::{thread, thread::sleep, time::Duration};
use std::collections::{HashMap};
use std::process::id;
use std::sync::{Arc, Mutex, MutexGuard};
use lazy_static::lazy_static;
use cached::proc_macro::cached;

use uinput::Device;
use uinput::event::ButtonsVec;
use uinput::event::keyboard::Key;
use crate::mouse_move::{Coords, move_mouse, print_deadzones, spawn_mouse_thread};

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

    static ref fake_device:Device = Device::init_mouse_keyboard();
    static ref mouse_coords_mutex:Mutex<Coords> = Mutex::new(Coords::default());
    static ref scroll_coords_mutex:Mutex<Coords> = Mutex::new(Coords::default());
}

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (id, gamepad) in gilrs.gamepads() {
        println!("id {}: {} is {:?}", id, gamepad.name(), gamepad.power_info());
    }
    print_deadzones(&gilrs, 0);

    spawn_mouse_thread();

    let mut commands_mode = true;
    let mut gilrs = Gilrs::new().unwrap();

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            debug!("{:?} device id {}", event, id);

            let mapping: &ButtonsMap = match commands_mode {
                true => &CommandsMap,
                false => &TypingMap,
            };

            match event {
                ButtonPressed(button, code) | ButtonReleased(button, code) => {
                    if mapping.contains_key(&button) {
                        let seq = &mapping[&button];
                        match event {
                            ButtonPressed(..) => {
                                fake_device.press_sequence(seq);
                            }
                            ButtonReleased(..) => {
                                fake_device.release_sequence(seq);
                            }
                            _ => {}
                        }
                    } else {
                        debug!("Unmapped button");
                        break;
                    }
                }
                AxisChanged(axis, value, code) => {
                    match axis {
                        Axis::LeftStickX | Axis::LeftStickY => {
                            let mut mouse_coords = mouse_coords_mutex.lock().unwrap();
                            if axis == Axis::LeftStickX {
                                mouse_coords.x = value;
                            } else {
                                mouse_coords.y = value;
                            }
                            drop(mouse_coords);
                        }
                        Axis::RightStickX | Axis::RightStickY => {
                            let mut scroll_coords = scroll_coords_mutex.lock().unwrap();
                            if axis == Axis::RightStickX {
                                scroll_coords.x = value;
                            } else {
                                scroll_coords.y = value;
                            }
                            drop(scroll_coords);
                        }
                        _ => {
                            debug!("Unmapped axis");
                            break;
                        }
                    }
                }
                _ => debug!("Action handling is omitted"),
            }
        }
        sleep(Duration::from_millis(25));
    }
}
