use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use gilrs::{Gilrs, Button, Event, EventType::*, Axis, Gamepad, GamepadId, EventType};
use lazy_static::lazy_static;
use uinput::event::relative::Position::{X, Y};

use crate::{fake_device, DEBUG, debug, ButtonsMap};

#[derive(Clone, Copy, Debug, Default)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TriggerButtons {
    pub left: f32,
    pub right: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum TriggerState {
    Pressed,
    Released,
    NoChange,
}

const MOUSE_SPEED: f32 = 1.0;
const ACCEL_STEP: f32 = 0.01;
const SCROLL_SPEED: f32 = 3.0;

lazy_static! {
    pub static ref TRIGGERS:Vec<Button> = vec![Button::LeftTrigger2, Button::RightTrigger2];
    pub static ref triggers_prev_mutex:Mutex<TriggerButtons> = Mutex::new(TriggerButtons::default());
    pub static ref mouse_coords_mutex:Mutex<Coords> = Mutex::new(Coords::default());
    pub static ref scroll_coords_mutex:Mutex<Coords> = Mutex::new(Coords::default());
}

const TRIGGER_THRESHOLD: f32 = 0.3;


fn get_squared_speed(value: f32, accel: f32) -> i32 {
    const POWER: f32 = 2.1;

    let sign = value / value.abs();
    let mut value = value.abs();

    if value > 0.1 {
        value = (value * 10.0).powf(POWER) / 10.0;
    }
    value *= sign;
    value *= (1.0 + accel);

    return (value.round() as f32 * MOUSE_SPEED) as i32;
}

pub fn move_mouse(coords: &MutexGuard<Coords>, accel: &mut f32) {
    if coords.x == 0.0 && coords.y == 0.0 {
        *accel = 1.0;
        return;
    }
    *accel += ACCEL_STEP;

    debug!("orig {} {}", coords.x, coords.y);
    let x_force = get_squared_speed(coords.x, *accel);
    let y_force = -get_squared_speed(coords.y, *accel);
    debug!("{}", accel);
    debug!("increased {} {}", x_force, y_force);

    if x_force != 0 {
        fake_device.send(X, x_force);
    }
    if y_force != 0 {
        fake_device.send(Y, y_force);
    }
    fake_device.synchronize();
}

pub fn spawn_mouse_thread() {
    thread::spawn(|| {
        let mut accel: f32 = 1.0;
        loop {
            let mut mouse_coords = mouse_coords_mutex.lock().unwrap();
            move_mouse(&mouse_coords, &mut accel);
            drop(mouse_coords);
            sleep(Duration::from_millis(25));
        }
    });
}

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

pub fn detect_trigger_state(value: f32, prev_value: &mut f32) -> TriggerState {
    let state =
        if value > *prev_value && value > TRIGGER_THRESHOLD {
            TriggerState::Pressed
        } else if value < *prev_value && value < TRIGGER_THRESHOLD {
            TriggerState::Released
        } else {
            TriggerState::NoChange
        };
    *prev_value = value;
    return state;
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


fn get_deadzone(gamepad: &Gamepad, axis: Axis) -> f32 {
    gamepad.deadzone(gamepad.axis_code(axis).unwrap()).unwrap()
}

fn get_gamepad(gilrs: &Gilrs, id: usize) -> Gamepad {
    let mut res: Option<Gamepad> = None;
    for (_id, gamepad) in gilrs.gamepads() {
        let _id: usize = _id.into();
        if _id == id {
            res = Some(gamepad);
        }
    };
    res.unwrap()
}

pub fn print_deadzones(gilrs: &Gilrs, id: usize) {
    let gamepad0 = get_gamepad(gilrs, id);
    let mut deadzone = Coords::default();

    deadzone.x = get_deadzone(&gamepad0, Axis::LeftStickX);
    deadzone.y = get_deadzone(&gamepad0, Axis::LeftStickY);
    println!("Left joystick deadzones: ({}, {})", deadzone.x, deadzone.y);

    deadzone.x = get_deadzone(&gamepad0, Axis::RightStickX);
    deadzone.y = get_deadzone(&gamepad0, Axis::RightStickY);
    println!("Right joystick deadzones: ({}, {})", deadzone.x, deadzone.y);
}