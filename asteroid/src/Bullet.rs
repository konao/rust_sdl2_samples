#![allow(non_snake_case)]

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;

// ---------------------------------
//  弾丸
// ---------------------------------
pub struct Bullet {
    // 位置
    x: f64,
    y: f64,
    vx: f64,    // 速度ベクトル
    vy: f64,    // 速度ベクトル
    valid: bool // 有効ならtrue
}

impl Bullet {
    pub fn new(_x: f64, _y: f64, _vx: f64, _vy: f64) -> Self {
        return Bullet {
            x: _x,
            y: _y,
            vx: _vx,
            vy: _vy,
            valid: true
        };
    }
    
    pub fn getX(&self) -> f64 {
        return self.x;
    }

    pub fn setX(&mut self, v: f64) -> () {
        self.x = v;
    }

    pub fn getY(&self) -> f64 {
        return self.y;
    }

    pub fn setY(&mut self, v: f64) -> () {
        self.y = v;
    }

    pub fn getVx(&self) -> f64 {
        return self.vx;
    }

    pub fn setVx(&mut self, v: f64) -> () {
        self.vx = v;
    }

    pub fn getVy(&self) -> f64 {
        return self.vy;
    }

    pub fn setVy(&mut self, v: f64) -> () {
        self.vy = v;
    }

    pub fn getValid(&self) -> bool {
        return self.valid;
    }

    pub fn setValid(&mut self, v: bool) -> () {
        self.valid = v;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        let p = Point::new(self.x as i32, self.y as i32);

        let _ = canvas.draw_point(p);
    }
}

