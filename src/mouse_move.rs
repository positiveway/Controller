use std::sync::MutexGuard;
use gilrs::{Axis, Gamepad, Gilrs};
use uinput::event::relative::Position::{X, Y};

use crate::{fake_device, DEBUG, debug};

#[derive(Clone, Copy, Debug, Default)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
}

const MOUSE_SPEED: f32 = 1.0;
const ACCEL_STEP: f32 = 0.01;
const SCROLL_SPEED: f32 = 3.0;

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