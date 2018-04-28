extern crate gilrs;
extern crate melon;
extern crate rand;
extern crate rpi_led_matrix;

use gilrs::{ev::EventType, Button, Event, Gilrs};
use rand::Rng;
use rpi_led_matrix::{LedColor, LedMatrix, LedMatrixOptions};
use std::{thread, time::Duration};

#[derive(Debug, Default)]
struct ControllerState {
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub start: bool,
    pub select: bool,
    pub right_trigger: bool,
    pub left_trigger: bool,
    pub d_pad_up: bool,
    pub d_pad_down: bool,
    pub d_pad_left: bool,
    pub d_pad_right: bool,
}

const BUTTON_DOWN_VALUE: f32 = 1.0;
const BUTTON_UP_VALUE: f32 = 0.0;

fn main() {
    let mut config = LedMatrixOptions::new();
    config.set_hardware_mapping("adafruit-hat");
    config.set_brightness(40).unwrap();

    let matrix = LedMatrix::new(Some(config)).unwrap();

    let mut canvas = matrix.canvas();

    let mut gilrs = Gilrs::new().unwrap();

    let mut ctrl_state = ControllerState::default();

    loop {
        // Examine new events
        while let Some(Event { event, .. }) = gilrs.next_event() {
            match event {
                EventType::ButtonChanged(button, value, ..) => match button {
                    Button::South => ctrl_state.b = value == BUTTON_DOWN_VALUE,
                    Button::East => ctrl_state.a = value == BUTTON_DOWN_VALUE,
                    Button::North => ctrl_state.x = value == BUTTON_DOWN_VALUE,
                    Button::West => ctrl_state.y = value == BUTTON_DOWN_VALUE,
                    Button::LeftTrigger => ctrl_state.left_trigger = value == BUTTON_DOWN_VALUE,
                    Button::RightTrigger => ctrl_state.right_trigger = value == BUTTON_DOWN_VALUE,
                    Button::Select => ctrl_state.select = value == BUTTON_DOWN_VALUE,
                    Button::Start => ctrl_state.start = value == BUTTON_DOWN_VALUE,
                    Button::DPadUp => {
                        if value == BUTTON_UP_VALUE {
                            ctrl_state.d_pad_up = true;
                            ctrl_state.d_pad_down = !ctrl_state.d_pad_up;
                        } else if value == BUTTON_DOWN_VALUE {
                            ctrl_state.d_pad_down = true;
                            ctrl_state.d_pad_up = !ctrl_state.d_pad_down;
                        } else {
                            ctrl_state.d_pad_up = false;
                            ctrl_state.d_pad_down = false;
                        }
                    }
                    Button::DPadRight => {
                        if value == BUTTON_DOWN_VALUE {
                            ctrl_state.d_pad_right = true;
                            ctrl_state.d_pad_left = !ctrl_state.d_pad_right;
                        } else if value == BUTTON_UP_VALUE {
                            ctrl_state.d_pad_left = true;
                            ctrl_state.d_pad_right = !ctrl_state.d_pad_left;
                        } else {
                            ctrl_state.d_pad_right = false;
                            ctrl_state.d_pad_left = false;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }

            const BASIC_DIFF: u8 = 64;

            let mut basic_color_red = 0x00;
            let mut basic_color_green = 0x00;
            let mut basic_color_blue = 0x00;

            if ctrl_state.a {
                basic_color_red = BASIC_DIFF;
            }

            if ctrl_state.b {
                basic_color_red += BASIC_DIFF;
                basic_color_green += BASIC_DIFF;
            }

            if ctrl_state.x {
                basic_color_blue += BASIC_DIFF;
            }

            if ctrl_state.y {
                basic_color_green += BASIC_DIFF;
            }

            canvas.fill(&LedColor {
                red: basic_color_red,
                green: basic_color_green,
                blue: basic_color_blue,
            });
        }
    }
}
