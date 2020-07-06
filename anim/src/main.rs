// *****************************************************************
//  SDL2 on Rust
//
//  アニメーション実験(1)
//  モンスター移動
//
//  カーソルキーで移動
//  スペースでキャラクタ変更
//
//  2020/7/4 konao
// *****************************************************************
#![allow(non_snake_case)]
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;
use sdl2::mouse::MouseButton;
use std::time::Duration;

fn main() {
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();

    let width: u32 = 960;
    let height: u32 = 800;
    let window = video_subsystem
        .window("SDL", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let image_texture = texture_creator.load_texture("assets/Image1.png").expect("load image failed");

    // 1つのキャラクター画像のピクセルサイズ
    let IMGW: u32 = 26;

    // 位置
    let mut x = (width/2) as i32;
    let mut y = (height/2) as i32;

    // 変位
    let mut dx: i32 = 0;
    let mut dy: i32 = 0;

    let mut dirInd: i32 = 0;    // 方向(0-3)
    let mut monster: i32 = 0;   // モンスター種類(0-4)
    let step: i32 = 4;  // 移動ステップ値

    let mut event_pump = sdl2_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Up), ..
                } => {
                    dx = 0;
                    dy = -step;
                    dirInd = 0;
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Down), ..
                } => {
                    dx = 0;
                    dy = step;
                    dirInd = 2;
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Left), ..
                } => {
                    dx = -step;
                    dy = 0;
                    dirInd = 3;
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Right), ..
                } => {
                    dx = step;
                    dy = 0;
                    dirInd = 1;
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Space), ..
                } => {
                    monster += 1;
                    if monster>4 { monster=0; }
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => break 'running,
                // | Event::MouseButtonDown => {
                //     println!("x={}, y={}", event.x, event.y);
                // },
                _ => {}
            }
        }

        // clear canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0)); // black
        canvas.clear();
        
        // update position
        x += dx;
        y += dy;
        x = if x<0 { 0 } else if x>=((width - IMGW) as i32) { (width - IMGW) as i32 } else { x };
        y = if y<0 { 0 } else if y>=((height - IMGW) as i32) { (height - IMGW) as i32 } else { y };

        // draw monster
        let src: Rect = Rect::new(((monster as u32)*(IMGW*4)+(dirInd as u32)*IMGW) as i32, (IMGW*1) as i32, IMGW, IMGW);
        let dest: Rect = Rect::new(x, y, IMGW, IMGW);
        canvas.copy(&image_texture, Some(src), Some(dest)).expect("copy texture to canvas failed");

        canvas.present();

        // get mouse state
        let mstate = event_pump.mouse_state();
        if mstate.left() {
            // left button is pressing
            println!("left button pressed ({}, {})", mstate.x(), mstate.y());
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
