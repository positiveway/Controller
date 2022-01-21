use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use lazy_static::lazy_static;
use gilrs::{Gilrs, Button, Event, EventType::*, Axis, Gamepad, GamepadId, EventType};
use uinput::Device;
use uinput::event::ButtonsVec;
use uinput::event::keyboard::Key;

macro_rules! debug {
    ($($arg:tt)*) => {
        if DEBUG{
            println!($($arg)*);
        }
    };
}
pub(crate) use debug;    // <-- the trick

pub type ButtonsMap = HashMap<Button, ButtonsVec>;


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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScrollDirection {
    Horizontal,
    Vertical,
}

pub const DEBUG: bool = false;

pub const MOUSE_SPEED_MULTIPLIER: f32 = 1.0;
pub const MOUSE_ACCEL_STEP: f32 = 0.04;
pub const USE_MOUSE_EXPONENTIAL: bool = true;
pub const USE_MOUSE_ACCEL: bool = true;

pub const SCROLL_ZERO_ZONE_X: f32 = 0.3;
pub const FAST_SCROLL_INTERVAL: f32 = 50 as f32;
pub const SLOW_SCROLL_INTERVAL: f32 = 250 as f32;

pub const TRIGGER_BTN_THRESHOLD: f32 = 0.3;


lazy_static! {
    pub static ref commands_mode_mutex:Mutex<bool> = Mutex::new(true);

    pub static ref fake_device:Device = Device::init_mouse_keyboard();

    pub static ref TRIGGER_BUTTONS:Vec<Button> = vec![Button::LeftTrigger2, Button::RightTrigger2];
    pub static ref triggers_prev_mutex:Mutex<TriggerButtons> = Mutex::new(TriggerButtons::default());

    pub static ref mouse_coords_mutex:Mutex<Coords> = Mutex::new(Coords::default());
    pub static ref scroll_coords_mutex:Mutex<Coords> = Mutex::new(Coords::default());
}

pub fn get_sign(value: f32) -> f32 {
    if value != 0.0 {
        value.signum()
    } else { value }
}