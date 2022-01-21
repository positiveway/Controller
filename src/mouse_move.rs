use std::sync::{Arc, Mutex, MutexGuard};
use std::{thread, thread::sleep, time::Duration};
use gilrs::{Gilrs, Button, Event, EventType::*, Axis, Gamepad, GamepadId, EventType};
use uinput::event::relative::Position::{X, Y};
use uinput::event::relative::Wheel;

use crate::struct_statics::*;


pub fn get_sign(value: f32) -> f32 {
    if value != 0.0 {
        value.signum()
    } else { value }
}


pub struct MoveInfo {
    prev_coords: Coords,
    cur_accel: f32,
    accel_step: f32,
    zero_zone: Coords,
}

impl MoveInfo {
    fn new(accel_step: f32, zero_zone: Option<Coords>) -> Self {
        Self {
            prev_coords: Coords::default(),
            cur_accel: 0.0,
            accel_step,
            zero_zone: zero_zone.unwrap_or(Coords::default()),
        }
    }

    fn is_accel_stop(&self, value: f32, prev_value: f32) -> bool {
        get_sign(value) != get_sign(prev_value)
    }
    fn calc_accel_stop(&mut self, coords: Coords, ignore_x: bool) -> bool {
        let is_stop_x = self.is_accel_stop(coords.x, self.prev_coords.x);
        let is_stop_y = self.is_accel_stop(coords.y, self.prev_coords.y);
        if ignore_x { is_stop_y } else { is_stop_x || is_stop_y }
    }

    fn _in_zero_zone(&self, value: f32, zero_zone_value: f32) -> bool {
        value.abs() <= zero_zone_value.abs()
    }

    fn in_zero_zone(&self, coords: Coords) -> bool {
        self._in_zero_zone(coords.x, self.zero_zone.x)
            && self._in_zero_zone(coords.y, self.zero_zone.y)
    }

    fn update_accel(&mut self, coords: Coords, ignore_x: bool) {
        let is_stop = self.calc_accel_stop(coords, ignore_x);
        if is_stop || self.in_zero_zone(coords) {
            self.cur_accel = 0.0
        } else {
            self.cur_accel += self.accel_step
        }
        self.prev_coords = coords;
    }

    fn apply_accel(&self, speed: &mut f32) {
        if USE_MOUSE_ACCEL {
            *speed *= (1.0 + self.cur_accel)
        };
    }
}

const MOUSE_SPEED: f32 = 1.0;

fn exponential_speed(value: f32) -> f32 {
    const POWER: f32 = 2.0;
    let sign = get_sign(value);
    let mut value = value.abs();

    if value > 0.1 {
        value = (value * 10.0).powf(POWER) / 10.0;
    }
    value *= sign;
    return value;
}

const MOUSE_ACCEL_STEP: f32 = 0.04;
const USE_MOUSE_EXPONENTIAL: bool = true;
const USE_MOUSE_ACCEL: bool = true;

fn calc_mouse_speed(value: f32, move_info: &MoveInfo) -> i32 {
    let mut value =
        if USE_MOUSE_EXPONENTIAL {
            exponential_speed(value)
        } else {
            value
        };
    move_info.apply_accel(&mut value);
    return (value * MOUSE_SPEED).round() as i32;
}

pub fn move_mouse(coords: &MutexGuard<Coords>, move_info: &mut MoveInfo) {
    move_info.update_accel(**coords, false);
    if move_info.in_zero_zone(**coords) {
        return;
    }
    debug!("orig {} {}", coords.x, coords.y);
    let x_force = calc_mouse_speed(coords.x, move_info);
    let y_force = -calc_mouse_speed(coords.y, move_info);
    println!("accel: {}", move_info.cur_accel);
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
        let mut move_info = MoveInfo::new(MOUSE_ACCEL_STEP, None);
        loop {
            let mut mouse_coords = mouse_coords_mutex.lock().unwrap();
            move_mouse(&mouse_coords, &mut move_info);
            drop(mouse_coords);
            sleep(Duration::from_millis(25));
        }
    });
}

const SCROLL_ZEROZONE_X: f32 = 0.3;

fn calc_scroll_direction(value: f32, scroll_direction: ScrollDirection) -> i32 {
    let mut value = get_sign(value);
    value *= -1.0;

    if scroll_direction == ScrollDirection::Horizontal {
        if value.abs() < SCROLL_ZEROZONE_X {
            value = 0.0
        }
    }
    value as i32
}

pub fn scroll_mouse(coords: &MutexGuard<Coords>) {
    debug!("orig {} {}", coords.x, coords.y);
    let x_force = calc_scroll_direction(coords.x, ScrollDirection::Horizontal);
    let y_force = calc_scroll_direction(coords.y, ScrollDirection::Vertical);
    debug!("dir {} {}", x_force, y_force);

    if x_force != 0 {
        fake_device.send(Wheel::Horizontal, x_force);
    }
    if y_force != 0 {
        fake_device.send(Wheel::Vertical, y_force);
    }
    fake_device.synchronize();
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
        let mut move_info =
            MoveInfo::new(MOUSE_ACCEL_STEP,
                          Some(Coords { x: SCROLL_ZEROZONE_X, y: 0.0 }));

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

const TRIGGER_THRESHOLD: f32 = 0.3;

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