use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use gilrs::{Gilrs, Button, Event, EventType::*, Axis, Gamepad, GamepadId, EventType};
use lazy_static::lazy_static;
use uinput::event::relative::Position::{X, Y};
use uinput::event::relative::Wheel;

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

lazy_static! {
    pub static ref TRIGGERS:Vec<Button> = vec![Button::LeftTrigger2, Button::RightTrigger2];
    pub static ref triggers_prev_mutex:Mutex<TriggerButtons> = Mutex::new(TriggerButtons::default());

    pub static ref mouse_coords_mutex:Mutex<Coords> = Mutex::new(Coords::default());
    pub static ref scroll_coords_mutex:Mutex<Coords> = Mutex::new(Coords::default());
}

const TRIGGER_THRESHOLD: f32 = 0.3;
const MOUSE_SPEED: f32 = 1.0;
const MOUSE_ACCEL_STEP: f32 = 0.01;

fn exponential_speed(value: f32) -> f32 {
    const POWER: f32 = 2.1;
    let sign = value.signum();
    let mut value = value.abs();

    if value > 0.1 {
        value = (value * 10.0).powf(POWER) / 10.0;
    }
    value *= sign;
    return value;
}

fn calc_mouse_speed(value: f32, accel: f32) -> i32 {
    let mut value = exponential_speed(value);
    value *= (1.0 + accel);

    return (value * MOUSE_SPEED).round() as i32;
}

pub fn move_mouse(coords: &MutexGuard<Coords>, accel: &mut f32) {
    if coords.x == 0.0 && coords.y == 0.0 {
        *accel = 1.0;
        return;
    }
    *accel += MOUSE_ACCEL_STEP;

    debug!("orig {} {}", coords.x, coords.y);
    let x_force = calc_mouse_speed(coords.x, *accel);
    let y_force = -calc_mouse_speed(coords.y, *accel);
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

const MIN_SCROLL_THRESHOLD: f32 = 0.3;

fn calc_scroll_direction(value: f32, scroll_direction: ScrollDirection) -> i32 {
    if value == 0.0{
        return 0;
    }
    let mut value = value.signum();
    value *= -1.0;

    if scroll_direction == ScrollDirection::Horizontal{
        if value.abs() < MIN_SCROLL_THRESHOLD{
            value = 0.0
        }
    }
    value as i32
}

pub fn scroll_mouse(coords: &MutexGuard<Coords>) {
    debug!("orig {} {}", coords.x, coords.y);
    let x_force = calc_scroll_direction(coords.x,ScrollDirection::Horizontal);
    let y_force = calc_scroll_direction(coords.y,ScrollDirection::Vertical);
    debug!("dir {} {}", x_force, y_force);

    if x_force != 0 {
        fake_device.send(Wheel::Horizontal, x_force);
    }
    if y_force != 0 {
        fake_device.send(Wheel::Vertical, y_force);
    }
    fake_device.synchronize();
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ScrollDirection{
    Horizontal,
    Vertical,
}

fn calc_scroll_interval(value: f32) -> f32 {
    let output_start = FAST_SCROLL_INTERVAL;
    let output_end = SLOW_SCROLL_INTERVAL;
    let precision = 100 as f32;
    let step = (output_end - output_start) / precision;

    let mut value = value.abs();
    value = (value * precision).round();
    let res = output_end - (step * value);
    debug!("Interval: {}", res);
    return res;
}

const FAST_SCROLL_INTERVAL: f32 = 50 as f32;
const SLOW_SCROLL_INTERVAL: f32 = 250 as f32;

pub fn spawn_scroll_thread() {
    thread::spawn(|| {
        let mut scroll_interval = SLOW_SCROLL_INTERVAL;
        loop {
            let mut scroll_coords = scroll_coords_mutex.lock().unwrap();
            if scroll_coords.y == 0.0 {
                scroll_interval = SLOW_SCROLL_INTERVAL;
            }
            scroll_interval = calc_scroll_interval(scroll_coords.y);
            scroll_mouse(&scroll_coords);
            drop(scroll_coords);
            sleep(Duration::from_millis(scroll_interval as u64));
        }
    });
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