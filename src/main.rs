extern crate sdl2;

mod button;
mod bar;
mod chat_client;

use sdl2::pixels::Color;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::path::Path;
use std::time::Duration;
use button::*;
use bar::*;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("chitchat-rs", 800, 600)
        .position_centered()
        .allow_highdpi()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut bar = Bar::sample();

    let font_context = sdl2::ttf::init().expect("failed to create font context");
    let path = Path::new("fonts/Andale Mono.ttf");
    let font = font_context.load_font(path, 30).expect("couldn't load font");
    let sk_rizz = font.render("skibidi rizz");
    let sk_surface = sk_rizz.solid(Color::BLACK).expect("couldn't render solid font");
    let texture_creator = canvas.texture_creator();
    let sk_texture = sk_surface.as_texture(&texture_creator).expect("couldn't convert sk_surface to texture");
    let txt_info = sk_texture.query();

    let (mut render_width, mut render_height) = canvas.window().drawable_size();
    let (mut win_width, mut win_height) = canvas.window().size();
    if render_width / win_width != render_height / win_height { panic!("what the heck?"); }
    let mut hidpi_scale = render_width / win_width;

    'running: loop {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        canvas.copy(&sk_texture, None, Rect::new(50, 50, txt_info.width, txt_info.height)).expect("couldn't copy texture");
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseMotion { x, y, .. } => {
                    if let Some(f) = bar.hover { f(Point::new(x, y), &mut bar); }
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    if let Some(f) = bar.click { f(Point::new(x, y), &mut bar); }
                }
                Event::Window { win_event: WindowEvent::Resized(_, _), .. } => {
                    (render_width, render_height) = canvas.window().drawable_size();
                    (win_width, win_height) = canvas.window().size();
                    if render_width / win_width != render_height / win_height { panic!("what the heck?"); }
                    hidpi_scale = render_width / win_width;
                }
                _ => {}
            }
        }

        bar.draw(&mut canvas, hidpi_scale);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
