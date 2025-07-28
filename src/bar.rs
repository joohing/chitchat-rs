#![allow(dead_code)]

use crate::button::*;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

#[derive(Debug, Clone)]
/// Bunch of synchronous buttons
pub struct Bar {
    pub pos: Point,
    w: u32,
    h: u32,
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
        let w = buttons.iter().fold(0, |acc, b| b.get_w_h().0 + acc + padding) + padding;
        let h = buttons.iter().fold(0, |acc, b| std::cmp::max(acc, b.get_w_h().1)) + 2 * padding;
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
}

impl Bar {
    pub fn draw(&self, canvas: &mut Canvas<Window>, hidpi_scale: u32) {
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
            b.draw(canvas, s_u, curr_point);
            canvas.set_draw_color(curr_color);
            curr_point += Point::new((b.get_w_h().0 + self.padding) as i32 * s_i, 0);
        }
        canvas.set_draw_color(previous_color);
    }
}

impl Bar {
    pub fn hover(&mut self, pos: Point) {
        sample_hover(pos, self);
    }

    fn mouse_off(&mut self) {
        todo!()
    }
}

impl Bar {
    pub async fn click(&mut self, mouse_pos: Point) -> reqwest::Result<()> {
        sample_click(mouse_pos, self).await?;
        Ok(())
    }
}

pub fn mouse_within_button(mouse_x: i32, mouse_y: i32, button_pos: Point, button: &Buttons) -> bool {
    mouse_x >= button_pos.x
        && mouse_x <= button_pos.x + button.get_w() as i32
        && mouse_y >= button_pos.y
        && mouse_y <= button_pos.y + button.get_h() as i32
}

pub fn sample_hover(mouse_pos: Point, bar: &mut Bar) {
    let mut curr_button_location = Point::new(
        bar.pos.x + bar.padding as i32,
        bar.pos.y + bar.padding as i32,
    );
    for b in bar.buttons.iter_mut() {
        let mouse_is_over = mouse_within_button(mouse_pos.x, mouse_pos.y, curr_button_location, &b);
        let mouse_on = mouse_is_over && !b.get_hovering();
        let mouse_off = !mouse_is_over && b.get_hovering();
        if mouse_on {
            b.set_hovering(true);
            b.hover(mouse_pos);
        } else if mouse_off {
            b.set_hovering(false);
            b.mouse_off();
        }
        curr_button_location += Point::new((b.get_w() + bar.padding) as i32, 0);
    }
}

pub async fn sample_click(mouse_pos: Point, bar: &mut Bar) -> reqwest::Result<()> {
    let mut curr_button_location = Point::new(
        bar.pos.x + bar.padding as i32,
        bar.pos.y + bar.padding as i32,
    );
    for b in bar.buttons.iter_mut() {
        let mouse_on = mouse_within_button(mouse_pos.x, mouse_pos.y, curr_button_location, &b);
        if mouse_on {
            b.click(mouse_pos).await?;
        }
        curr_button_location += Point::new((b.get_w() + bar.padding) as i32, 0);
    }
    Ok(())
}
