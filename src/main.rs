#![feature(box_syntax)]
#[macro_use]
extern crate partial_application;

use gilrs::{Gilrs, Button, Event, EventType::*, Axis};
use inputbot::{KeybdKey, MouseButton, GenericButton, MouseWheel, MouseCursor};
use std::{thread::sleep, time::Duration};
use std::collections::{HashMap};

type BoxedButton = Box<dyn GenericButton>;
type ButtonsVec = Vec<BoxedButton>;
type ButtonsMapPair = (Button, ButtonsVec);

const MOUSE_SPEED: i32 = 5;
const SCROLL_SPEED: i32 = 3;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

// Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let super_key = KeybdKey::OtherKey(0xffeb);
    let copy_key: ButtonsVec = vec![box KeybdKey::LControlKey, box KeybdKey::CKey];
    let paste_key: ButtonsVec = vec![box KeybdKey::LControlKey, box KeybdKey::VKey];

    let _commands_map: Vec<ButtonsMapPair> = vec![
        (Button::DPadDown, vec![box KeybdKey::DownKey]),
        (Button::DPadUp, vec![box KeybdKey::UpKey]),
        (Button::DPadLeft, vec![box KeybdKey::LeftKey]),
        (Button::DPadRight, vec![box KeybdKey::RightKey]),
        (Button::RightTrigger2, vec![box MouseButton::LeftButton]),
        (Button::LeftTrigger2, vec![box MouseButton::RightButton]),
        (Button::RightTrigger, paste_key),
        (Button::LeftTrigger, copy_key),
        (Button::West, vec![box KeybdKey::EnterKey]),
        (Button::North, vec![box KeybdKey::SpaceKey]),
        (Button::South, vec![box KeybdKey::BackspaceKey]),
        (Button::East, vec![box super_key]),
    ];
    let mut commands_map: HashMap<Button, ButtonsVec> = HashMap::new();
    for pair in _commands_map {
        let (orig_button, mapped_seq) = pair;
        commands_map.insert(orig_button, mapped_seq);
    }

    let typing_map = box commands_map.clone();

    let mut commands_mode = true;

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("Event {:?} from device id {} at {:?}", event, id, time);


            let mapping = match commands_mode {
                true => &commands_map,
                false => &typing_map,
            };

            match event {
                ButtonPressed(button, code) | ButtonReleased(button, code) => {
                    if mapping.contains_key(&button) {
                        let seq = &mapping[&button];
                        match event {
                            ButtonPressed(..) => {
                                for key in seq {
                                    key.press();
                                }
                            }
                            ButtonReleased(..) => {
                                for key in seq.into_iter().rev() {
                                    key.release();
                                }
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
