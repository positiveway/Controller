mod uinput_direct;

#[macro_use]
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
use uinput::event::relative::Position::{X, Y};

type ButtonsMap = HashMap<Button, ButtonsVec>;

const DEBUG: bool = false;

macro_rules! debug {
    ($($arg:tt)*) => {
        if DEBUG{
            println!($($arg)*);
        }
    };
}

#[derive(Clone, Copy, Debug)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

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
}

const MOUSE_SPEED: f32 = 10.0;
const SCROLL_SPEED: f32 = 3.0;

pub fn move_mouse(coords: &MutexGuard<Coords>) {
    let x_force = coords.x * MOUSE_SPEED;
    let y_force = coords.y * -MOUSE_SPEED;
    if x_force != 0.0 {
        fake_device.send(X, x_force as i32);
    }
    if y_force != 0.0 {
        fake_device.send(Y, y_force as i32);
    }
    fake_device.synchronize();
}

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (id, gamepad) in gilrs.gamepads() {
        println!("id {}: {} is {:?}", id, gamepad.name(), gamepad.power_info());
    }
    let coords_mutex = Arc::new(Mutex::new(Coords { x: 0.0, y: 0.0 }));
    let coords_mutex_clone = Arc::clone(&coords_mutex);

    thread::spawn(move || {
        loop {
            let mut coords = coords_mutex_clone.lock().unwrap();
            if coords.x != 0.0 || coords.y != 0.0 {
                debug!("{} {}", coords.x, coords.y);
            }
            move_mouse(&coords);
            drop(coords);
            sleep(Duration::from_millis(50));
        }
    });

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
                            let mut coords = coords_mutex.lock().unwrap();
                            match axis {
                                Axis::LeftStickX => {
                                    coords.x = value;
                                }
                                Axis::LeftStickY => {
                                    coords.y = value;
                                }
                                _ => {}
                            }
                            drop(coords);
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
    }
}
