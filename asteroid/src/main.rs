// *****************************************************************
//  Asteroid
//  Simple Application with SDL2 on Rust
//
//  2020/7/9 konao
// *****************************************************************
#![allow(non_snake_case)]
extern crate sdl2;
extern crate rand;

use sdl2::render::TextureCreator;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

// このアプリ自身のクレート(lib.rsで公開しているもの)を使う．
// クレート名はCargo.tomlの[package]のnameで指定したものになる．
extern crate asteroid;

use asteroid::Game;

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

    // init font stuff
    let ttf_context = sdl2::ttf::init().expect("failed to init SDL TTF");
    let font = ttf_context.load_font("assets/arial.ttf", 128).expect("failed to load font");
    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let mut game = Game::Game::new();

    let mut event_pump = sdl2_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Up), ..
                } => {
                    game.upPressed();
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Left), ..
                } => {
                    game.leftPressed();
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Right), ..
                } => {
                    game.rightPressed();
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Space), ..
                } => {
                    game.spacePressed();
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Return), ..
                } => {
                    game.enterPressed();
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => break 'running,
                _ => {}
            }
        }

        // update scene
        game.update(&mut canvas, &font, &texture_creator, width, height);

        // show backbuffer
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
