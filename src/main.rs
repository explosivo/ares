extern crate sdl2;
extern crate gl;

pub mod input;
pub mod entity;

use sdl2::pixels::Color;
use std::time::Duration;

static NAME: &'static str = "ares";
static VERSION: &'static str = "experimental";

fn get_title() -> String {
    (NAME.to_owned() + " - " + &VERSION.to_owned())
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window(&get_title(), 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut input = input::Input::new(sdl.event_pump().unwrap());

    let entity = entity::Entity::new(128, 128, 32, 32, 0);
    let mut player = entity::ControllableEntity::new(10, 10, 64, 64, 2);

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        input.event_loop();

        if input.quit_requested {
            break 'running
        }

        player.update(&input);
        player.render(&mut canvas);

        entity.render(&mut canvas);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
