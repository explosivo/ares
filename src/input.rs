extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Input {
    pub event_pump: sdl2::EventPump,
    pub quit_requested: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
}

impl Input {
    pub fn new(event_pump: sdl2::EventPump) -> Self {
        Input {
            event_pump: event_pump,
            quit_requested: false,
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
        }
    }

    pub fn event_loop(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    self.quit_requested = true;
                },
                Event::KeyDown {keycode: Some(Keycode::W), ..} => {
                    self.up_pressed = true;
                    self.down_pressed = false;
                },
                Event::KeyDown {keycode: Some(Keycode::S), ..} => {
                    self.down_pressed = true;
                    self.up_pressed = false;
                },
                Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    self.left_pressed = true;
                    self.right_pressed = false;
                },
                Event::KeyDown {keycode: Some(Keycode::D), ..} => {
                    self.right_pressed = true;
                    self.left_pressed = false;
                },

                Event::KeyUp {keycode: Some(Keycode::W), ..} => {
                    self.up_pressed = false;
                },
                Event::KeyUp {keycode: Some(Keycode::S), ..} => {
                    self.down_pressed = false;
                },
                Event::KeyUp {keycode: Some(Keycode::A), ..} => {
                    self.left_pressed = false;
                },
                Event::KeyUp {keycode: Some(Keycode::D), ..} => {
                    self.right_pressed = false;
                },

                _ => {}
            }
        }
    }
}
