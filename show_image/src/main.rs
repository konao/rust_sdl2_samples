// *****************************************************************
//  SDL2 on Rust
//
//  PNG画像をテクスチャとして読み込んで表示
//
//  SDL2_imageライブラリをダウンロードして.libをビルド用ディレクトリに、
//  .dllを実行時ディレクトリに置く必要がある
//
//  SDL2_Imageのダウンロード先：
//  https://www.libsdl.org/projects/SDL_image/
//
//  ここからDevelopment Libraries(Windows)の
//  SDL2_image-devel-2.0.5-VC.zip (Visual C++ 32/64-bit)
//  をダウンロード
//
//  lib/x64ディレクトリにあるSDL2_image.libを
//  C:\Users\ユーザー名\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib
//  にコピー（これでビルドが通るようになる）
//
//  また、lib/x64/*.dllを実行時のカレントディレクトリ(=cargo runするディレクトリ)にコピーしておく
//
//  2020/7/3 konao
// *****************************************************************
#![allow(non_snake_case)]
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;
use std::time::Duration;

fn main() {
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();

    let window = video_subsystem
        .window("SDL", 600, 875)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let image_texture = texture_creator.load_texture("assets/elichika.jpg").expect("load image failed");

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

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        
        canvas.copy(&image_texture, None, None).expect("copy texture to canvas failed");

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
