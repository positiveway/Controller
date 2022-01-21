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


lazy_static! {
    pub static ref commands_mode_mutex:Mutex<bool> = Mutex::new(true);

    pub static ref fake_device:Device = Device::init_mouse_keyboard();

    pub static ref TRIGGERS:Vec<Button> = vec![Button::LeftTrigger2, Button::RightTrigger2];
    pub static ref triggers_prev_mutex:Mutex<TriggerButtons> = Mutex::new(TriggerButtons::default());

    pub static ref mouse_coords_mutex:Mutex<Coords> = Mutex::new(Coords::default());
    pub static ref scroll_coords_mutex:Mutex<Coords> = Mutex::new(Coords::default());
}

