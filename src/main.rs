use gilrs::{Gilrs, Button, Event, EventType,Axis};
use inputbot::{KeybdKey, MouseButton, MouseWheel, MouseCursor};
use std::{thread::sleep, time::Duration};
use std::collections::HashMap;

#[macro_use]
extern crate partial_application;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

// Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mouse_speed = 5;
    let scroll_speed = 3;

    let super_key = (0xffeb);
    let copy_key = (KeybdKey::LControlKey, KeybdKey::CKey);
    let paste_key = (KeybdKey::LControlKey, KeybdKey::VKey);

    let commands_map = HashMap::from([
        (Button::DPadDown, (KeybdKey::DownKey)),
        (Button::DPadUp, (KeybdKey::UpKey)),
        (Button::DPadLeft, (KeybdKey::LeftKey)),
        (Button::DPadRight, (KeybdKey::RightKey)),
        (Button::RightTrigger2, (MouseButton::LeftButton)),
        (Button::LeftTrigger2, (MouseButton::RightButton)),
        (Button::RightTrigger, paste_key),
        (Button::LeftTrigger, copy_key),
        (Button::West, (KeybdKey::EnterKey)),
        (Button::North, (KeybdKey::SpaceKey)),
        (Button::South, (KeybdKey::BackspaceKey)),
        (Button::East, super_key),
        (Axis::LeftStickX, partial!(MouseCursor::move_rel => _, 0)),
        (Axis::LeftStickY, partial!(MouseCursor::move_rel => 0, _)),
        (Axis::RightStickY, (MouseWheel::scroll_ver)),
        (Axis::RightStickX, (MouseWheel::scroll_hor)),
    ]);

    let typing_map = commands_map.clone();

    let mut commands_mode = true;

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("Event {:?} from device id {} at {:?}", event, id, time);

            let mapping = match commands_mode {
                true => commands_map,
                false => typing_map,
            };

            match event {
                EventType::ButtonPressed(button,code) => {
                    mapping[button].press();
                },
                EventType::ButtonReleased(button, code) => {
                    mapping[button].release();
                },
                EventType::AxisChanged(axis, value, code) => {
                    let mouse_func = mapping[axis];
                }
                _ => println!("Error"),
            }
        }
    }
}
