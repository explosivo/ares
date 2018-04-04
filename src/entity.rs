extern crate sdl2;

use input;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Entity {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    speed: i32,
}

impl Entity {
    pub fn new (x: i32, y: i32, w: u32, h: u32, speed: i32) -> Self {
        Entity {x: x, y: y, w: w, h: h, speed: speed}
    }

    pub fn render(&self, ref mut canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.fill_rect(Rect::new(self.x, self.y, self.w, self.h));
    }
}

pub struct ControllableEntity {
    entity: Entity,
}

impl ControllableEntity {
    pub fn new(x: i32, y: i32, w: u32, h: u32, speed: i32) -> Self {
        ControllableEntity {
            entity: Entity::new(x, y, w, h, speed)
        }
    }

    pub fn update(&mut self, input: &input::Input) {
        if input.up_pressed {
            self.entity.y -= self.entity.speed;
        }
        if input.down_pressed {
            self.entity.y += self.entity.speed;
        }
        if input.left_pressed {
            self.entity.x -= self.entity.speed;
        }
        if input.right_pressed {
            self.entity.x += self.entity.speed;
        }

    }

    pub fn render(&self, ref mut canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(Rect::new(self.entity.x, self.entity.y, self.entity.w, self.entity.h));
    }
}
