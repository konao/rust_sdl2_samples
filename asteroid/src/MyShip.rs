#![allow(non_snake_case)]

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use Util;
use Bullet;

// ---------------------------------
// 宇宙船
// ---------------------------------
pub struct MyShip {
    // 位置
    x: f64,
    y: f64,
    vx: f64,    // 速度ベクトル
    vy: f64,    // 速度ベクトル
    radius: f64,    // 半径
    rotation: f64,  // 回転角
    maxSpeed: f64, // 最大速さ
    explosionAnimCount: i32 // 爆発アニメーションカウント
}

impl MyShip {
    // 第1引数に &self, self を取らないメソッドはクラスメソッド 
    // (Rust用語では Associated Method)とみなされる
    pub fn new(_x: f64, _y: f64, _radius: f64, _rot: f64, _maxSpeed: f64) -> Self {
        return MyShip {
            x: _x,
            y: _y,
            vx: 0.0,
            vy: 0.0,
            radius: _radius,
            rotation: _rot,
            maxSpeed: _maxSpeed,
            explosionAnimCount: 0
        };
    }

    pub fn getX(&self) -> f64 {
        return self.x;
    }

    pub fn getY(&self) -> f64 {
        return self.y;
    }

    pub fn clearExplosionAnimCount(&mut self) {
        self.explosionAnimCount = 0;
    }

    pub fn updateExplosionAnimCount(&mut self) -> i32 {
        self.explosionAnimCount += 1;
        return self.explosionAnimCount;
    }

    pub fn updatePos(&mut self, width: u32, height: u32) {
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

    pub fn updateDir(&mut self) {
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

    pub fn incrRotation(&mut self, d: f64) {
        self.rotation += d;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 0));

        // ３角形の中央
        let x = self.radius * self.rotation.cos() + self.x;
        let y = self.radius * self.rotation.sin() + self.y;

        // 3角形の大きさ
        let tr = self.radius;

        // 3角形の頂点3つの座標を計算
        let mut theta: f64 = self.rotation;
        let p1: Point = Point::new((x+tr*theta.cos()) as i32, (y+tr*theta.sin()) as i32);
        theta = self.rotation + Util::deg2rad(150.0);
        let p2: Point = Point::new((x+tr*theta.cos()) as i32, (y+tr*theta.sin()) as i32);
        theta = self.rotation + Util::deg2rad(210.0);
        let p3: Point = Point::new((x+tr*theta.cos()) as i32, (y+tr*theta.sin()) as i32);

        // draw_lines()で3角形を描く
        let ps = [p1, p2, p3, p1];
        let _ = canvas.draw_lines(ps.as_ref()); // [Point]から&[Point]を生成する
    }

    pub fn drawExplosion(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 0));

        // ３角形の中央
        let x = self.radius * self.rotation.cos() + self.x;
        let y = self.radius * self.rotation.sin() + self.y;

        // 爆発半径
        let er = (self.explosionAnimCount as f64) * 5.0;

        let N = 12;
        for i in 0..N {
            let theta: f64 = Util::deg2rad(360.0 * (i as f64) / (N as f64) + (self.explosionAnimCount as f64) * 2.0);
            let cp: Point = Point::new((x+er*theta.cos()) as i32, (y+er*theta.sin()) as i32);

            let ro1 = Util::deg2rad((self.explosionAnimCount as f64) * 5.0);
            let ro2 = Util::deg2rad((self.explosionAnimCount as f64) * 5.0 + 180.0);
            
            let er2 = 10.0; // 爆発半径2(固定値)
            let p1: Point = Point::new((er2*ro1.cos()) as i32 + cp.x, (er2*ro1.sin()) as i32 + cp.y);
            let p2: Point = Point::new((er2*ro2.cos()) as i32 + cp.x, (er2*ro2.sin()) as i32 + cp.y);

            // draw_lines()で3角形を描く
            let ps = [p1, p2];
            let _ = canvas.draw_lines(ps.as_ref()); // [Point]から&[Point]を生成する
        }
    }

    pub fn fire(&self) -> Bullet::Bullet {
        let x = self.x;
        let y = self.y;
        let vx = self.rotation.cos() * 5.0;
        let vy = self.rotation.sin() * 5.0;

        // println!("new bullet : p=({}, {}), v=({}, {})", x, y, vx, vy);

        return Bullet::Bullet::new(x, y, vx, vy);
    }
}
