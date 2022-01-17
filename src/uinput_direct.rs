// use lazy_static::lazy_static;
// use std::sync::Mutex;
//
// use uinput::device::{Builder,Device};
// use uinput::event::Keyboard;
// use uinput::event::keyboard::Key;
// use uinput::event::controller::Mouse;
// use uinput::event::controller;
// use uinput::event::controller::Mouse::{Left, Middle, Right};
// use uinput::event::Event::{Controller, Relative};
// use uinput::event::relative::Position::{X, Y};
// use uinput::event::relative::Relative::Position;
//
// type OptKey = Option<Key>;
// type OptMouse = Option<Mouse>;
//
// #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
// pub struct Btn {
//     pub k: OptKey,
//     pub m: OptMouse,
// }
//
// impl Btn {
//     pub fn press(&self){
//         let mut device = fake_device.lock().unwrap();
//
//         if self.k.is_some() {
//             let btn = self.k.unwrap();
//             device.press(&btn);
//         } else {
//             let btn = self.m.unwrap();
//             device.press(&btn);
//         };
//         device.synchronize();
//     }
//
//     pub fn release(&self){
//         let mut device = fake_device.lock().unwrap();
//
//         if self.k.is_some() {
//             let btn = self.k.unwrap();
//             device.release(&btn);
//         } else {
//             let btn = self.m.unwrap();
//             device.release(&btn);
//         };
//         device.synchronize();
//     }
// }
//
// pub type ButtonsVec = Vec<Btn>;
//
// pub fn press_sequence(sequence: &ButtonsVec) {
//     for button in sequence {
//         button.press();
//     }
// }
//
// pub fn release_sequence(sequence: &ButtonsVec) {
//     for button in sequence.into_iter().rev() {
//         button.release();
//     }
// }
//
// fn init_mouse_keyboard() -> Mutex<Device>{
//     let mut _device = Builder::default().unwrap()
//         .name("fakeinputs").unwrap()
//         .event(Keyboard::All).unwrap()
//         .event(Controller(controller::Controller::Mouse(Left))).unwrap()
//         .event(Controller(controller::Controller::Mouse(Right))).unwrap()
//         .event(Controller(controller::Controller::Mouse(Middle))).unwrap()
//         .event(Relative(Position(X))).unwrap()
//         .event(Relative(Position(Y))).unwrap()
//         .create().unwrap();
//     Mutex::new(_device)
// }
//
// lazy_static! {
//         static ref fake_device:Mutex<Device> = init_mouse_keyboard();
// }