// *****************************************************************
//  SDL2 on Rust
//
//  ボールが回っているように見える幻影
//
//  ボールの透過色の設定は、pngファイル特有のtransparent colorを使っている
//  (JPEGではできない)
//
//  2024/6/25 konao
// *****************************************************************
#![allow(non_snake_case)]
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::time::Duration;
use std::env;

struct Ball {
    x: f64, // 現在の位置
    y: f64,
    cx: f64,    // 中心点
    cy: f64,
    radius: f64,    // 半径
    theta: f64, // 角度
    t: f64, // 時間
    dt: f64, // 変位
    edge_x1: i32,    // 端点の座標1
    edge_y1: i32,
    edge_x2: i32,    // 端点の座標2
    edge_y2: i32
}

impl Ball {
    pub fn new(cx: f64, cy: f64, radius: f64, theta: f64, t: f64) -> Self {
        Self {
            x: cx,
            y: cy,
            cx,
            cy,
            radius,
            theta,
            t,
            dt: 0.01,
            edge_x1: (radius * theta.cos() + cx) as i32,
            edge_y1: (radius * theta.sin() + cy) as i32,
            edge_x2: (-radius * theta.cos() + cx) as i32,
            edge_y2: (-radius * theta.sin() + cy) as i32,
        }
    }

    pub fn update(&mut self) {
        let vx = self.radius * self.theta.cos();
        let vy = self.radius * self.theta.sin();
        self.x = vx * self.t.sin() + self.cx;
        self.y = vy * self.t.sin() + self.cy;
        self.t += self.dt;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()<2 {
        println!("usage {} nBalls", args[0]);
        return;
    }
    let nBallsStr = &args[1];
    let nBalls = nBallsStr.parse::<i32>().unwrap();

    println!("nBalls={}", nBalls);

    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();

    let WINDOW_WIDTH = 800;
    let WINDOW_HEIGHT = 800;
    let IMGW: u32 = 48;

    let window = video_subsystem
        .window("Rotating ball illusion", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let image_texture = texture_creator.load_texture("assets/neptune_transparent.png").expect("load image failed");

    let mut balls: Vec<Ball> = Vec::new();

    let pi2 = 3.14159265 * 2.0;
    for i in 0..nBalls {
        balls.push(Ball::new(
            (WINDOW_WIDTH as f64) / 2.0,    // cx
            (WINDOW_HEIGHT as f64) / 2.0,   // cy
            (WINDOW_WIDTH as f64)/2.5,  // radius
            pi2 / (nBalls as f64) * (i as f64),  // theta
            pi2 / (nBalls as f64) * (i as f64)  // t
        ));
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

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // ボールの軌道の直線を描く
        for i in 0..nBalls {
            let idx = i as usize;
            
            let p1: Point = Point::new(balls[idx].edge_x1, balls[idx].edge_y1);
            let p2: Point = Point::new(balls[idx].edge_x2, balls[idx].edge_y2);
    
            // draw_lines()で直線
            let ps = [p1, p2];
            canvas.set_draw_color(Color::RGB(128, 128, 128));
            let _ = canvas.draw_lines(ps.as_ref()); // [Point]から&[Point]を生成する    
        }

        // ボールを描く
        for i in 0..nBalls {
            let idx = i as usize;
            
            let dest: Rect = Rect::new(balls[idx].x as i32 - (IMGW as i32) / 2, balls[idx].y as i32 - (IMGW as i32) / 2, IMGW, IMGW);
            canvas.copy(&image_texture, None, Some(dest)).expect("copy texture to canvas failed");

            // ボールの位置更新
            balls[idx].update();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
