#![feature(box_syntax)]

mod uinput_direct;

#[macro_use]
extern crate partial_application;

use gilrs::{Gilrs, Button, Event, EventType::*, Axis};
use inputbot::{KeybdKey, MouseButton, GenericButton, MouseWheel, MouseCursor};
use std::{thread::sleep, time::Duration};
use std::collections::{HashMap};
use lazy_static::lazy_static;
use std::sync::Arc;
use cached::proc_macro::cached;

type ArcButton = Arc<dyn GenericButton + Send + Sync + 'static>;
type ButtonsVec = Vec<ArcButton>;
type ButtonsMapPair = (Button, ButtonsVec);
type ButtonsMap = HashMap<Button, ButtonsVec>;

lazy_static! {
    static ref super_key: ButtonsVec = vec![Arc::new(KeybdKey::OtherKey(0xffeb))];
    static ref copy_key: ButtonsVec = vec![Arc::new(KeybdKey::LControlKey), Arc::new(KeybdKey::CKey)];
    static ref paste_key: ButtonsVec = vec![Arc::new(KeybdKey::LControlKey), Arc::new(KeybdKey::VKey)];

    static ref CommandsMap: ButtonsMap = {
        let _commands_map: Vec<ButtonsMapPair> = vec![
            (Button::DPadDown, vec![Arc::new(KeybdKey::DownKey)]),
            (Button::DPadUp, vec![Arc::new(KeybdKey::UpKey)]),
            (Button::DPadLeft, vec![Arc::new(KeybdKey::LeftKey)]),
            (Button::DPadRight, vec![Arc::new(KeybdKey::RightKey)]),
            (Button::RightTrigger2, vec![Arc::new(MouseButton::LeftButton)]),
            (Button::LeftTrigger2, vec![Arc::new(MouseButton::RightButton)]),
            (Button::RightTrigger, paste_key.to_owned()),
            (Button::LeftTrigger, copy_key.to_owned()),
            (Button::West, vec![Arc::new(KeybdKey::EnterKey)]),
            (Button::North, vec![Arc::new(KeybdKey::SpaceKey)]),
            (Button::South, vec![Arc::new(KeybdKey::BackspaceKey)]),
            (Button::East, super_key.to_owned()),
        ];
        let mut commands_map: ButtonsMap = HashMap::new();
        for pair in _commands_map {
            let (orig_button, mapped_seq) = pair;
            commands_map.insert(orig_button, mapped_seq);
        }
        commands_map
    };

    static ref TypingMap: ButtonsMap = {
        let typing_map = CommandsMap.clone();
        typing_map
    };
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
