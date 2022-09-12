use gilrs::{Axis, Button, Event, EventType::*, EventType};

pub fn match_button(button: &Button) -> &str {
    match button {
        Button::South => "a",
        Button::East => "b",
        Button::North => "c",
        Button::West => "d",
        Button::C => "e",
        Button::Z => "f",
        Button::LeftTrigger => "g",
        Button::LeftTrigger2 => "h",
        Button::RightTrigger => "i",
        Button::RightTrigger2 => "j",
        Button::Select => "k",
        Button::Start => "l",
        Button::Mode => "m",
        Button::LeftThumb => "n",
        Button::RightThumb => "o",
        Button::DPadUp => "p",
        Button::DPadDown => "q",
        Button::DPadLeft => "r",
        Button::DPadRight => "s",
        Button::Unknown => "t",
    }
}

pub fn match_axis(axis: &Axis) -> &str {
    match axis {
        Axis::LeftStickX => "u",
        Axis::LeftStickY => "v",
        Axis::LeftZ => "w",
        Axis::RightStickX => "x",
        Axis::RightStickY => "y",
        Axis::RightZ => "z",
        Axis::DPadX => "0",
        Axis::DPadY => "1",
        Axis::Unknown => "2",
    }
}

pub fn match_event(event: &EventType) -> (&str, String, &str, String) {
    let mut button_or_axis = "";
    let mut res_value: f32 = 0.0;
    let mut event_type = "";
    let mut res_code = String::from("");

    match event {
        AxisChanged(axis, value, code) => {
            event_type = "a";
            res_value = *value;
            button_or_axis = match_axis(axis);
            res_code = code.to_string()
        }
        ButtonChanged(button, value, code) => {
            event_type = "b";
            res_value = *value;
            button_or_axis = match_button(button);
            res_code = code.to_string()
        }
        ButtonReleased(button, code) => {
            event_type = "c";
            button_or_axis = match_button(button);
            res_code = code.to_string()
        }
        ButtonPressed(button, code) => {
            event_type = "d";
            button_or_axis = match_button(button);
            res_code = code.to_string()
        }
        ButtonRepeated(button, code) => {
            event_type = "e";
            button_or_axis = match_button(button);
            res_code = code.to_string()
        }
        Connected => {
            event_type = "f"
        }
        Disconnected => {
            event_type = "g"
        }
        Dropped => {
            event_type = "h"
        }
    };
    let res_value = res_value.to_string();
    return (button_or_axis, res_value, event_type, res_code);
}