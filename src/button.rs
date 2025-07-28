#![allow(dead_code)]

use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::chat_client::User;

#[derive(Debug, Clone)]
pub enum Buttons {
    Button(Button),
    SendButton(SendButton),
}

#[derive(Debug, Clone, Copy)]
pub struct Button {
    pub w: u32,
    pub h: u32,
    pub color: Option<Color>,
    orig_color: Option<Color>,
    pub is_hovering: bool,
}

#[derive(Debug, Clone)]
/// Sends some content when clicked
pub struct SendButton {
    pub w: u32,
    pub h: u32,
    pub color: Option<Color>,
    orig_color: Option<Color>,
    pub text_content: String,
    pub is_hovering: bool,
    pub send_to: User,
}

impl Buttons {
    pub fn new(w: u32, h: u32, color: Option<Color>) -> Buttons {
        Buttons::Button(Button {
            w,
            h,
            color,
            orig_color: color,
            is_hovering: false,
        })
    }

    pub fn sample() -> Buttons {
        Buttons::new(25, 10, Some(sdl2::pixels::Color::GRAY))
    }

    fn set_hovering(&mut self, s: bool) {
        match self {
            Buttons::Button(b) => b.is_hovering = s,
            Buttons::SendButton(b) => b.is_hovering = s,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, hidpi_scale: u32, pos: Point) {
        let (s_u, _) = (hidpi_scale as u32, hidpi_scale as i32);
        let previous_color = canvas.draw_color();
        if let Some(c) = self.get_color() {
            canvas.set_draw_color(c);
        }
        let (w, h) = self.get_w_h();
        let res = canvas.fill_rect(Rect::new(pos.x, pos.y, w * s_u, h * s_u));
        canvas.set_draw_color(previous_color);
        if res.is_err() {
            panic!("{:?}", res.unwrap());
        }
    }

    pub fn get_w(&self) -> u32 {
        match self {
            Buttons::SendButton(b) => b.w,
            Buttons::Button(b) => b.w,
        }
    }

    pub fn get_h(&self) -> u32 {
        match self {
            Buttons::SendButton(b) => b.h,
            Buttons::Button(b) => b.h,
        }
    }

    pub fn get_w_h(&self) -> (u32, u32) {
        match self {
            Buttons::SendButton(b) => (b.w, b.h),
            Buttons::Button(b) => (b.w, b.h),
        }
    }

    pub fn get_color(&self) -> Option<Color> {
        match self {
            Buttons::Button(b) => b.color,
            Buttons::SendButton(b) => b.color,
        }
    }

    pub fn set_color(&mut self, darkened_color: Option<Color>) {
        match self {
            Buttons::Button(b) => b.color = darkened_color,
            Buttons::SendButton(b) => b.color = darkened_color,
        }
    }

    pub fn get_orig_color(&self) -> Option<Color> {
        match self {
            Buttons::Button(b) => b.orig_color,
            Buttons::SendButton(b) => b.orig_color,
        }
    }

    pub fn get_hovering(&self) -> bool {
        match self {
            Buttons::Button(b) => b.is_hovering,
            Buttons::SendButton(b) => b.is_hovering,
        }
    }

    pub async fn click(&mut self, pos: Point) -> reqwest::Result<()> {
        match self {
            Buttons::Button(b) => Ok(b.click(pos)),
            Buttons::SendButton(b) => Ok(b.click().await?),
        }
    }
}

impl SendButton {
    async fn click(&mut self) -> reqwest::Result<()> {
        sample_async_click(self).await?;
        Ok(())
    }
}

impl Buttons {
    pub fn hover(&mut self, _: i32, _: i32) {
        button_darken_on_hover(self);
    }

    pub fn mouse_off(&mut self) {
        button_restore_color(self);
    }
}

impl Button {
    pub fn click(&mut self, _: Point) {
        button_print_on_click(self);
    }
}

pub fn button_print_on_click(_: &mut Button) {
    println!("Clicked!");
}

pub fn button_print_on_hover(_: &mut Button) {
    println!("Hover!");
}

pub fn button_darken_on_hover(b: &mut Buttons) {
    b.set_hovering(true);
    if let Some(c) = b.get_color() {
        let darkened_color = Color::RGB(
            (c.r as f32 * 0.8) as u8,
            (c.g as f32 * 0.8) as u8,
            (c.b as f32 * 0.8) as u8,
        );
        b.set_color(Some(darkened_color));
    }
}

pub fn button_restore_color(b: &mut Buttons) {
    b.set_hovering(false);
    if b.get_color().is_some() {
        b.set_color(b.get_orig_color());
    }
}

pub async fn sample_async_click(_: &mut SendButton) -> reqwest::Result<()> {
    let _ = reqwest::get("194.163.183.44:8000/api/playercount").await?;
    Ok(())
}
