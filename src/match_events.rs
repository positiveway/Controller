use gilrs::{Gilrs, Button, Event, EventType::*, Axis, Gamepad, GamepadId, EventType};


pub fn match_button(button: &Button) -> &str {
    match button {
        Button::South => "S",
        Button::East => "E",
        Button::North => "N",
        Button::West => "W",
        Button::C => "C",
        Button::Z => "Z",
        Button::LeftTrigger => "L",
        Button::LeftTrigger2 => "L2",
        Button::RightTrigger => "R",
        Button::RightTrigger2 => "R2",
        Button::Select => "Se",
        Button::Start => "St",
        Button::Mode => "M",
        Button::LeftThumb => "LT",
        Button::RightThumb => "RT",
        Button::DPadUp => "DU",
        Button::DPadDown => "DD",
        Button::DPadLeft => "DL",
        Button::DPadRight => "DR",
        Button::Unknown => "U",
    }
}

pub fn match_axis(axis: &Axis) -> &str {
    match axis {
        Axis::LeftStickX => "LX",
        Axis::LeftStickY => "LY",
        Axis::LeftZ => "LZ",
        Axis::RightStickX => "RX",
        Axis::RightStickY => "RY",
        Axis::RightZ => "RZ",
        Axis::DPadX => "DX",
        Axis::DPadY => "DY",
        Axis::Unknown => "U",
    }
}

pub fn match_event(event: &EventType) -> (&str, String, &str) {
    let mut button_or_axis = "No";
    let mut res_value: f32 = 0.0;
    let mut event_type = "";

    match event {
        EventType::AxisChanged(axis, value, code) => {
            event_type = "A";
            res_value = *value;
            button_or_axis = match_axis(axis);
        }
        EventType::ButtonChanged(button, value, code) => {
            event_type = "B";
            res_value = *value;
            button_or_axis = match_button(button);
        }
        EventType::ButtonReleased(button, code) => {
            event_type = "Rl";
            button_or_axis = match_button(button);
        }
        EventType::ButtonPressed(button, code) => {
            event_type = "P";
            button_or_axis = match_button(button);
        }
        EventType::ButtonRepeated(button, code) => {
            event_type = "Rp";
            button_or_axis = match_button(button);
        }
        EventType::Connected => {
            event_type = "C"
        }
        EventType::Disconnected => {
            event_type = "D"
        }
        EventType::Dropped => {
            event_type = "Dr"
        }
    };
    let res_value = res_value.to_string();
    return (button_or_axis, res_value, event_type);
}