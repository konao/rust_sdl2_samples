// *****************************************************************
//  Asteroid
//  Simple Application with SDL2 on Rust
//
//  2020/7/9 konao
// *****************************************************************
#![allow(non_snake_case)]
extern crate sdl2;
extern crate rand;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;
use rand::Rng;

fn deg2rad(x: f64) -> f64 {
    return x * 3.14159265 / 180.0;
}

// ---------------------------------
// 宇宙船
// ---------------------------------
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
    fn new(_x: f64, _y: f64, _radius: f64, _rot: f64, _maxSpeed: f64) -> Self {
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

    fn fire(&self) -> Bullet {
        let x = self.x;
        let y = self.y;
        let vx = self.rotation.cos() * 5.0;
        let vy = self.rotation.sin() * 5.0;

        // println!("new bullet : p=({}, {}), v=({}, {})", x, y, vx, vy);

        return Bullet::new(x, y, vx, vy);
    }
}

// ---------------------------------
//  弾丸
// ---------------------------------
struct Bullet {
    // 位置
    x: f64,
    y: f64,
    vx: f64,    // 速度ベクトル
    vy: f64,    // 速度ベクトル
    valid: bool // 有効ならtrue
}

impl Bullet {
    fn new(_x: f64, _y: f64, _vx: f64, _vy: f64) -> Self {
        return Bullet {
            x: _x,
            y: _y,
            vx: _vx,
            vy: _vy,
            valid: true
        };
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        let p: Point = Point::new(self.x as i32, self.y as i32);

        let _ = canvas.draw_point(p);
    }
}

// ---------------------------------
//  小惑星
// ---------------------------------
struct Asteroid {
    // 位置
    x: f64,
    y: f64,
    vx: f64,    // 速度ベクトル
    vy: f64,
    rot: f64, // 回転角
    drot: f64,    // 回転速度
    size: i32,  // サイズ
    shape: Vec<(f64, f64)>, // 小惑星の形
    valid: bool // 有効ならtrue
}

impl Asteroid {
    fn new(_size: i32, _width: u32, _height: u32) -> Self {
        // 小惑星を生成
        // 位置、速度、形状は乱数で決める
        let mut shape: Vec<(f64, f64)> = Vec::new();

        let mut rng = rand::thread_rng();

        // 位置を生成
        let x = rng.gen::<f64>() * (_width as f64);
        let y = rng.gen::<f64>() * (_height as f64);

        // 速度を生成
        let vx = rng.gen::<f64>() * 4.0 - 2.0;
        let vy = rng.gen::<f64>() * 4.0 - 2.0;

        // 形を生成
        let r = (_size * 2) as f64;
        for i in 0.._size {
            let theta = (i as f64) * 3.14159265 * 2.0 / (_size as f64);
            let x: f64 = (r + (rng.gen::<f64>() * 20.0 - 10.0)) * theta.cos();
            let y: f64 = (r + (rng.gen::<f64>() * 20.0 - 10.0)) * theta.sin();
            shape.push((x, y));
        }

        let rot = rng.gen::<f64>() * 3.14159265 * 2.0;
        let drot = (rng.gen::<f64>() - 0.5) * 3.14159265 / 180.0 * 5.0;

        return Asteroid {
            x: x,
            y: y,
            vx: vx,
            vy: vy,
            rot: rot,
            drot: drot,
            size: _size,
            shape: shape,
            valid: true
        };
    }

    fn update(&mut self, width: u32, height: u32) {
        let wf = (width as i32) as f64;
        let hf = (height as i32) as f64;

        // update position
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

        // update rotation
        let pi2 = 3.14159265 * 2.0;
        self.rot += self.drot;
        self.rot = if self.rot<0.0 {
            self.rot + pi2
        } else if self.rot>pi2 {
            self.rot - pi2
        } else {
            self.rot
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // draw_lines()で3角形を描く
        let mut ps = Vec::new();

        for i in 0..self.shape.len()+1 {
            let j = if i==self.shape.len() { 0 } else { i };
            let x = self.x + self.shape[j].0 * self.rot.cos() - self.shape[j].1 * self.rot.sin();
            let y = self.y + self.shape[j].0 * self.rot.sin() + self.shape[j].1 * self.rot.cos();
            ps.push(Point::new(x as i32, y as i32));
        }

        let _ = canvas.draw_lines(ps.as_ref()); // [Point]から&[Point]を生成する
    }
}

// ---------------------------------
//  メインルーチン
// ---------------------------------
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

    // 弾丸を保持するベクトル
    let mut bullets = Vec::new();

    // 小惑星を保持するベクトル
    let mut asteroids = Vec::new();

    // 小惑星を生成
    let mut rng = rand::thread_rng();
    for _ in 1..10 {
        let size = (((rng.gen::<f64>() * 15.0) as i32) + 3) * 2;
        let ax = (rng.gen::<f64>() * (width as f64)) as u32;
        let ay = (rng.gen::<f64>() * (height as f64)) as u32;
        let asteroid = Asteroid::new(
            size,
            ax,
            ay
        );
        asteroids.push(asteroid);
    }

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
                    bullets.push(myShip.fire());  // 弾発射
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

        // update bullets
        let wf = (width as i32) as f64;
        let hf = (height as i32) as f64;
        for bullet in &mut bullets {
            if bullet.valid {
                // 有効なら進める
                let newx = bullet.x + bullet.vx;
                let newy = bullet.y + bullet.vy;

                if (newx < 0.0) || (newy < 0.0) || (newx > wf) || (newy > hf) {
                    // 画面から外れた．無効にする
                    // （本来は消すべきだが、dropの仕方がよくわからないのでフラグを使って無効にする）
                    bullet.valid = false;
                } else {
                    bullet.x = newx;
                    bullet.y = newy;
                }
            }
        }

        // draw bullets
        for bullet in &bullets {
            if bullet.valid {
                bullet.draw(&mut canvas);
            }
        }

        // update & draw asteroids
        for asteroid in &mut asteroids {
            if asteroid.valid {
                asteroid.update(width, height);
                asteroid.draw(&mut canvas);
            }
        }

        // show backbuffer
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
