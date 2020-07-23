#![allow(non_snake_case)]

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;

// ---------------------------------
//  小惑星
// ---------------------------------
pub struct Asteroid {
    // 位置
    x: f64,
    y: f64,
    vx: f64,    // 速度ベクトル
    vy: f64,
    rot: f64, // 回転角
    drot: f64,    // 回転速度
    size: i32,  // サイズ
    shape: Vec<(f64, f64)>, // 小惑星の形
    meanRadius: f64,    // 半径の平均値（衝突判定用）
    valid: bool // 有効ならtrue
}

impl Asteroid {
    pub fn new(_size: i32, x: f64, y: f64) -> Self {
        use rand::{self, Rng};  // ここにselfがないと下の(1)でエラーになる
        // https://stackoverflow.com/questions/52216426/access-to-external-crates-from-module

        // 小惑星を生成
        // 位置、速度、形状は乱数で決める
        let mut shape: Vec<(f64, f64)> = Vec::new();

        let mut rng = rand::thread_rng();   // (1)

        // 速度を生成
        let vx = rng.gen::<f64>() * 4.0 - 2.0;
        let vy = rng.gen::<f64>() * 4.0 - 2.0;

        // 形を生成
        let r = (_size * 2) as f64;
        let mut rsum: f64 = 0.0;
        for i in 0.._size {
            let theta = (i as f64) * 3.14159265 * 2.0 / (_size as f64);
            let x: f64 = (r + (rng.gen::<f64>() * 20.0 - 10.0)) * theta.cos();
            let y: f64 = (r + (rng.gen::<f64>() * 20.0 - 10.0)) * theta.sin();
            shape.push((x, y));
            rsum += (x*x+y*y).sqrt();
        }
        let meanRadius = rsum/(_size as f64); // 半径の平均値

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
            meanRadius: meanRadius,
            valid: true
        };
    }

    // ランダムに生成
    pub fn gen(_size: i32, _width: u32, _height: u32) -> Self {
        use rand::{self, Rng};

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
        let mut rsum: f64 = 0.0;
        for i in 0.._size {
            let theta = (i as f64) * 3.14159265 * 2.0 / (_size as f64);
            let x: f64 = (r + (rng.gen::<f64>() * 20.0 - 10.0)) * theta.cos();
            let y: f64 = (r + (rng.gen::<f64>() * 20.0 - 10.0)) * theta.sin();
            shape.push((x, y));
            rsum += (x*x+y*y).sqrt();
        }
        let meanRadius = rsum/(_size as f64); // 半径の平均値

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
            meanRadius: meanRadius,
            valid: true
        };
    }

    pub fn getX(&self) -> f64 {
        return self.x;
    }

    pub fn getY(&self) -> f64 {
        return self.y;
    }

    pub fn getSize(&self) -> i32 {
        return self.size;
    }

    pub fn setSize(&mut self, v: i32) -> () {
        self.size = v;
    }

    pub fn getValid(&self) -> bool {
        return self.valid;
    }

    pub fn setValid(&mut self, v: bool) -> () {
        self.valid = v;
    }

    pub fn update(&mut self, width: u32, height: u32) {
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

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
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

    // 衝突判定
    // (tx, ty) ... 衝突判た対象座標
    pub fn hitTest(&self, tx: f64, ty: f64) -> bool {
        let tdx = tx - self.x;
        let tdy = ty - self.y;
        let tr = (tdx*tdx + tdy*tdy).sqrt();

        return if tr < self.meanRadius { true } else { false };
    }
}
