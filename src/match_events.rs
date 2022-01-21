use std::sync::{Arc, Mutex, MutexGuard};
use gilrs::{Gilrs, Button, Event, EventType::*, Axis, Gamepad, GamepadId, EventType};
use crate::struct_statics::*;
use crate::mouse_move::*;

fn process_mouse_arm(axis: Axis, value: f32) {
    let mut mouse_coords = mouse_coords_mutex.lock().unwrap();
    if axis == Axis::LeftStickX {
        mouse_coords.x = value;
    } else {
        mouse_coords.y = value;
    }
    drop(mouse_coords);
}

fn process_scroll_arm(axis: Axis, value: f32) {
    let mut scroll_coords = scroll_coords_mutex.lock().unwrap();
    if axis == Axis::RightStickX {
        scroll_coords.x = value;
    } else {
        scroll_coords.y = value;
    }
    drop(scroll_coords);
}

pub fn process_axis(axis: Axis, value: f32) {
    match axis {
        Axis::LeftStickX | Axis::LeftStickY => {
            process_mouse_arm(axis, value);
        }
        Axis::RightStickX | Axis::RightStickY => {
            process_scroll_arm(axis, value);
        }
        _ => {
            debug!("Unmapped axis");
            return;
        }
    }
}

pub fn process_btn_press_release(event: EventType, button: Button, mapping: &ButtonsMap) {
    if TRIGGERS.contains(&button) {
        return;
    }
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
        return;
    }
}

pub fn process_btn_change(button: Button, value: f32, mapping: &ButtonsMap) {
    if !TRIGGERS.contains(&button) {
        return;
    }
    if mapping.contains_key(&button) {
        let seq = &mapping[&button];

        let mut triggers_prev_values = triggers_prev_mutex.lock().unwrap();
        let trigger_state =
            if button == Button::LeftTrigger2 {
                detect_trigger_state(value, &mut triggers_prev_values.left)
            } else {
                detect_trigger_state(value, &mut triggers_prev_values.right)
            };

        match trigger_state {
            TriggerState::Pressed => {
                fake_device.press_sequence(seq)
            }
            TriggerState::Released => {
                fake_device.release_sequence(seq)
            }
            TriggerState::NoChange => {}
        }
        drop(triggers_prev_values);
    } else {
        debug!("Unmapped button");
        return;
    }
}