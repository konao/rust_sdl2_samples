// *****************************************************************
//  SDL2 on Rust
//
//  多数のオブジェクトの制御
//
//  2020/7/3 konao
// *****************************************************************
#![allow(non_snake_case)]

extern crate sdl2;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

// 1個 の 物体（ここでは3角形）
struct Entity {
    // 位置
    cx: f64,
    cy: f64,
    radius: f64,    // 現在の半径
    max_radius: f64,    // 最大半径
    d_radius: f64,  // 半径増減値
    rotation: f64,  // 回転度合
    brightness: f64,    // 明るさの割合(0.0-1.0)
    d_brightness: f64   // 明るさ変位
}

// Entity構造体のメソッドを定義
// Traitを使わなくても、直接構造体のメソッドを定義できる
impl Entity {
    // 第1引数に &self, self を取らないメソッドはクラスメソッド 
    // (Rust用語では Associated Method)とみなされる
    fn new(_x: f64, _y: f64, _radius: f64, _rot: f64, _brightness: f64) -> Entity {
        return Entity {
            cx: _x,
            cy: _y,
            radius: _radius,
            max_radius: _radius,
            d_radius: 5.0,
            rotation: _rot,
            brightness: _brightness,
            d_brightness: 0.05
        };
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, (255.0 * &self.brightness) as u8, 0));

        // ３角形の中央
        let x = self.radius * self.rotation.cos() + self.cx;
        let y = self.radius * self.rotation.sin() + self.cy;

        // 3角形の大きさ
        let tr = 20.0;

        // 3角形の頂点3つの座標を計算
        let mut theta: f64 = 0.0;
        let p1: Point = Point::new((x+tr*theta.cos()) as i32, (y+tr*theta.sin()) as i32);
        theta = 3.141592*2.0*120.0/360.0;
        let p2: Point = Point::new((x+tr*theta.cos()) as i32, (y+tr*theta.sin()) as i32);
        theta = 3.141592*2.0*240.0/360.0;
        let p3: Point = Point::new((x+tr*theta.cos()) as i32, (y+tr*theta.sin()) as i32);

        // draw_lines()で3角形を描く
        let ps = [p1, p2, p3, p1];
        let _ = canvas.draw_lines(ps.as_ref()); // [Point]から&[Point]を生成する
    }

    fn incrRotation(&mut self, d: f64) {
        self.rotation += d;
    }

    fn updateRadius(&mut self) {
        let r = self.radius + self.d_radius;
        if r >= self.max_radius {
            self.radius = self.max_radius;
            self.d_radius = -self.d_radius;
        } else if r <= 0.0 {
            self.radius = 0.0;
            self.d_radius = -self.d_radius;
        } else {
            self.radius = r;
        }
    }

    fn updateBrightness(&mut self) {
        let v = self.brightness + self.d_brightness;
        if v >= 1.0 {
            self.brightness = 1.0;
            self.d_brightness = - self.d_brightness;    // 反転
        } else if v <= 0.0 {
            self.brightness = 0.0;
            self.d_brightness = - self.d_brightness;    // 反転
        } else {
            self.brightness = v;
        }
    }
}

fn main() {
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();

    let w = 960;
    let h = 640;
    let R: f64 = 300.0;

    let window = video_subsystem
        .window("SDL", w, h)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.present();

    // object initialize
    let mut entities = Vec::new();
    let N = 32;
    for i in 0..N {
        let mut ent = Entity::new(
            (w as f64)/2.0, 
            (h as f64)/2.0,
            R,
            3.14159265 * 2.0 * (i as f64) / (N as f64),
            (i as f64) / (N as f64)
        );
        entities.push(ent);
    }

    let mut event_pump = sdl2_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // clear canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    
        // draw objects
        for ent in &mut entities {
            // draw
            ent.draw(&mut canvas);

            // update entity
            ent.incrRotation(3.141592/180.0*2.0);
            ent.updateRadius();
            ent.updateBrightness();
        }

        // show background image
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
