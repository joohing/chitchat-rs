#![allow(dead_code)]

use crate::button::*;
use crate::chat_client::{User, ServerInfo};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    ttf::Font,
    video::Window,
};

#[derive(Debug, Clone)]
pub struct Bar {
    pub pos: Point,
    pub w: u32,
    pub h: u32,
    pub padding: u32,
    pub color: Option<Color>,
    orig_color: Option<Color>,
    pub is_hovering: bool,
    pub buttons: Vec<Buttons>,
}

impl Bar {
    pub fn new(
        x: i32,
        y: i32,
        padding: u32,
        color: Option<Color>,
        buttons: Vec<Buttons>,
    ) -> Bar {
        let w = if buttons.is_empty() {
            800  // Full width for header
        } else {
            buttons.iter().fold(0, |acc, b| b.get_w_h().0 + acc + padding) + padding
        };
        let h = if buttons.is_empty() {
            60   // Fixed height for header
        } else {
            buttons.iter().fold(0, |acc, b| std::cmp::max(acc, b.get_w_h().1)) + 2 * padding
        };
        Bar {
            pos: Point::new(x, y),
            w,
            h,
            padding,
            color,
            orig_color: color,
            is_hovering: false,
            buttons,
        }
    }

    pub fn sample() -> Bar {
        Bar::new(
            0,
            0,
            1,
            Some(sdl2::pixels::Color::GRAY),
            vec![Buttons::sample(),
                 Buttons::sample(),
                 Buttons::sample()],
        )
    }

    pub fn unhover(&mut self) {
        for b in self.buttons.iter_mut() {
            b.mouse_off();
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, hidpi_scale: u32, font: &Font) {
        let (s_u, s_i) = (hidpi_scale as u32, hidpi_scale as i32);
        let previous_color = canvas.draw_color();
        if let Some(c) = self.color {
            canvas.set_draw_color(c);
        }
        let res = canvas.fill_rect(Rect::new(
            self.pos.x * s_i,
            self.pos.y * s_i,
            self.w * s_u,
            self.h * s_u,
        ));
        if res.is_err() {
            panic!("{:?}", res.unwrap());
        }
        let mut curr_point = Point::new(
            (self.pos.x + self.padding as i32) * s_i,
            (self.pos.y + self.padding as i32) * s_i,
        );
        for b in &self.buttons {
            let curr_color = canvas.draw_color();
            if let Some(c) = b.get_color() {
                canvas.set_draw_color(c);
            }
            b.draw(canvas, s_u, curr_point, font);
            canvas.set_draw_color(curr_color);
            curr_point += Point::new((b.get_w_h().0 + self.padding) as i32 * s_i, 0);
        }
        canvas.set_draw_color(previous_color);
    }

    pub fn hover(&mut self, x: i32, y: i32) {
        sample_hover(x, y, self);
    }

    fn mouse_off(&mut self) {
        todo!()
    }

    pub async fn click(&mut self, mouse_pos: Point, user: &User, server_info: &mut ServerInfo) -> reqwest::Result<()> {
        sample_click(mouse_pos, self, user, server_info).await?;
        Ok(())
    }
}

pub fn mouse_within_button(mouse_x: i32, mouse_y: i32, button_pos: Point, button: &Buttons) -> bool {
    mouse_x >= button_pos.x
        && mouse_x <= button_pos.x + button.get_w() as i32
        && mouse_y >= button_pos.y
        && mouse_y <= button_pos.y + button.get_h() as i32
}

pub fn sample_hover(x: i32, y: i32, bar: &mut Bar) {
    let mut curr_button_location = Point::new(
        bar.pos.x + bar.padding as i32,
        bar.pos.y + bar.padding as i32,
    );
    for b in bar.buttons.iter_mut() {
        let mouse_is_over = mouse_within_button(x, y, curr_button_location, &b);
        let mouse_on = mouse_is_over && !b.get_hovering();
        let mouse_off = !mouse_is_over && b.get_hovering();
        if mouse_on {
            b.hover(x, y);
        } else if mouse_off {
            b.mouse_off();
        }
        curr_button_location += Point::new((b.get_w() + bar.padding) as i32, 0);
    }
}

pub async fn sample_click(mouse_pos: Point, bar: &mut Bar, user: &User, server_info: &mut ServerInfo) -> reqwest::Result<()> {
    let mut curr_button_location = Point::new(
        bar.pos.x + bar.padding as i32,
        bar.pos.y + bar.padding as i32,
    );
    for b in bar.buttons.iter_mut() {
        let mouse_on = mouse_within_button(mouse_pos.x, mouse_pos.y, curr_button_location, &b);
        if mouse_on {
            b.click(mouse_pos, user, server_info).await?;
        }
        curr_button_location += Point::new((b.get_w() + bar.padding) as i32, 0);
    }
    Ok(())
}
