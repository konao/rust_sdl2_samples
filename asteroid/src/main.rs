// *****************************************************************
//  Asteroid
//  Simple Application with SDL2 on Rust
//
//  2020/7/9 konao
// *****************************************************************
#![allow(non_snake_case)]
extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rand::Rng;

// このアプリ自身のクレート(lib.rsで公開しているもの)を使う
extern crate asteroid;

use asteroid::Util;
use asteroid::MyShip;
use asteroid::Asteroid;

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
    let D_ROT: f64 = Util::deg2rad(5.0);

    // 宇宙船初期化
    let mut myShip = MyShip::MyShip::new(
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
        let asteroid = Asteroid::Asteroid::gen(
            size,
            width,
            height
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
            if bullet.getValid() {
                // 有効なら進める
                let newx = bullet.getX() + bullet.getVx();
                let newy = bullet.getY() + bullet.getVy();

                if (newx < 0.0) || (newy < 0.0) || (newx > wf) || (newy > hf) {
                    // 画面から外れた．無効にする
                    // （本来は消すべきだが、dropの仕方がよくわからないのでフラグを使って無効にする）
                    bullet.setValid(false);
                } else {
                    bullet.setX(newx);
                    bullet.setY(newy);

                    // 衝突判定
                    let mut newAsteroids = Vec::new();
                    for asteroid in &mut asteroids {
                        if asteroid.getValid() && asteroid.hitTest(newx, newy) {
                            // 小惑星に当たった
                            asteroid.setValid(false);
                            bullet.setValid(false);

                            // 小惑星を分裂させる
                            if asteroid.getSize() >= 6 {
                                let newSize = asteroid.getSize() / 2;
                                for _ in 0..3 {
                                    let newAstroid = Asteroid::Asteroid::new(newSize, asteroid.getX(), asteroid.getY());
                                    newAsteroids.push(newAstroid);
                                }
                            }
                        }
                    }

                    for newAsteroid in newAsteroids {
                        asteroids.push(newAsteroid);
                    }
                }
            }
        }

        // draw bullets
        for bullet in &bullets {
            if bullet.getValid() {
                bullet.draw(&mut canvas);
            }
        }

        // update & draw asteroids
        for asteroid in &mut asteroids {
            if asteroid.getValid() {
                asteroid.update(width, height);
                asteroid.draw(&mut canvas);
            }
        }

        // show backbuffer
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
