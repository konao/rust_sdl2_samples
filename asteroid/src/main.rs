// *****************************************************************
//  Asteroid
//  Simple Application with SDL2 on Rust
//
//  2020/7/7 konao
// *****************************************************************
#![allow(non_snake_case)]
extern crate sdl2;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

fn deg2rad(x: f64) -> f64 {
    return x * 3.14159265 / 180.0;
}

// 宇宙船
struct MyShip {
    // 位置
    x: f64,
    y: f64,
    vx: f64,    // 速度ベクトル
    vy: f64,    // 速度ベクトル
    radius: f64,    // 半径
    rotation: f64,  // 回転角
    maxSpeed: f64 // 最大速さ
}

impl MyShip {
    // 第1引数に &self, self を取らないメソッドはクラスメソッド 
    // (Rust用語では Associated Method)とみなされる
    fn new(_x: f64, _y: f64, _radius: f64, _rot: f64, _maxSpeed: f64) -> MyShip {
        return MyShip {
            x: _x,
            y: _y,
            vx: 0.0,
            vy: 0.0,
            radius: _radius,
            rotation: _rot,
            maxSpeed: _maxSpeed
        };
    }

    fn updatePos(&mut self, width: u32, height: u32) {
        let wf = (width as i32) as f64;
        let hf = (height as i32) as f64;

        self.x += self.vx;
        self.y += self.vy;

        self.x = if self.x<0.0 {
            self.x + wf
        } else if self.x>=wf {
            self.x - wf
        } else { 
            self.x 
        };

        self.y = if self.y<0.0 {
            self.y + hf
        } else if self.y>=hf {
            self.y - hf
        } else {
            self.y 
        };
    }

    fn updateDir(&mut self) {
        // 加速度ベクトル計算
        let k = 0.5;    // 加速係数
        let ax = k * self.rotation.cos();
        let ay = k * self.rotation.sin();

        // 速度ベクトル更新
        let vx1 = self.vx + ax;
        let vy1 = self.vy + ay;
        
        // 速さを計算
        let speed = (vx1*vx1 + vy1*vy1).sqrt();

        if (speed >= 0.0) && (speed < self.maxSpeed) {
            // 速度範囲内なら現在値を更新
            self.vx = vx1;
            self.vy = vy1;
        }
    }

    fn incrRotation(&mut self, d: f64) {
        self.rotation += d;
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // ３角形の中央
        let x = self.radius * self.rotation.cos() + self.x;
        let y = self.radius * self.rotation.sin() + self.y;

        // 3角形の大きさ
        let tr = self.radius;

        // 3角形の頂点3つの座標を計算
        let mut theta: f64 = self.rotation;
        let p1: Point = Point::new((x+tr*theta.cos()) as i32, (y+tr*theta.sin()) as i32);
        theta = self.rotation + deg2rad(150.0);
        let p2: Point = Point::new((x+tr*theta.cos()) as i32, (y+tr*theta.sin()) as i32);
        theta = self.rotation + deg2rad(210.0);
        let p3: Point = Point::new((x+tr*theta.cos()) as i32, (y+tr*theta.sin()) as i32);

        // draw_lines()で3角形を描く
        let ps = [p1, p2, p3, p1];
        let _ = canvas.draw_lines(ps.as_ref()); // [Point]から&[Point]を生成する
    }
}

fn main() {
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();

    let width: u32 = 1280;
    let height: u32 = 900;
    let window = video_subsystem
        .window("Asteroid", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // 最大速度
    let MAX_SPEED: f64 = 15.0;

    // 回転変位
    let D_ROT: f64 = deg2rad(5.0);

    // 宇宙船初期化
    let mut myShip = MyShip::new(
        (width/2) as f64,   // x
        (height/2) as f64,  // y
        10.0,   // 半径（宇宙船の大きさ）
        0.0,    // 初期回転角
        MAX_SPEED   // 最大速さ
    );

    let mut event_pump = sdl2_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Up), ..
                } => {
                    myShip.updateDir(); // 加速
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Left), ..
                } => {
                    myShip.incrRotation(-D_ROT);    // 左回転
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Right), ..
                } => {
                    myShip.incrRotation(D_ROT);     // 右回転
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Space), ..
                } => {
                    // fire
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => break 'running,
                _ => {}
            }
        }

        // clear canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0)); // black
        canvas.clear();
        
        // update position
        myShip.updatePos(width, height);

        // draw ship
        myShip.draw(&mut canvas);

        // show backbuffer
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
