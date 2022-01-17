mod uinput_direct;

#[macro_use]
extern crate partial_application;

use gilrs::{Gilrs, Button, Event, EventType::*, Axis};
use std::{thread::sleep, time::Duration};
use std::collections::{HashMap};
use lazy_static::lazy_static;
use cached::proc_macro::cached;
use uinput::Device;

use uinput::event::ButtonsVec;
use uinput::event::keyboard::Key;

type ButtonsMap = HashMap<Button, ButtonsVec>;

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

const MOUSE_SPEED: i32 = 5;
const SCROLL_SPEED: i32 = 3;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

// Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut commands_mode = true;

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("Event {:?} from device id {} at {:?}", event, id, time);

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
                        println!("Unmapped button");
                        break;
                    }
                }

                AxisChanged(axis, value, code) => {}
                _ => println!("Action handling is omitted"),
            }
        }
    }
}
