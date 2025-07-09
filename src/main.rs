extern crate sdl2;

mod button;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::mouse::MouseButton;
use std::time::Duration;
use button::*;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let button = Button::new(50, 50, 100, 50, Some(sdl2::pixels::Color::BLACK));
    let buttons = vec![&button];

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    let relevant_buttons = buttons
                        .iter().filter(|b|
                            x <= b.pos.x + b.w as i32
                         && y <= b.pos.y + b.h as i32
                         && x >= b.pos.y
                         && y >= b.pos.y
                        );
                    for b in relevant_buttons {
                        b.click(Point::new(x, y));
                    }
                }
                _ => {}
            }
        }
        button.draw(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
