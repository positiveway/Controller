use std::sync::MutexGuard;
use uinput::event::relative::Position::{X, Y};

use crate::{fake_device, DEBUG, debug};

#[derive(Clone, Copy, Debug)]
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