use crate::button::*;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

pub enum Bars {
    Bar(Bar),
    AsyncBar(AsyncBar),
}

#[derive(Debug, Clone)]
pub struct Bar {
    pub pos: Point,
    w: u32,
    h: u32,
    pub padding: u32,
    pub color: Option<Color>,
    orig_color: Option<Color>,
    pub is_hovering: bool,
    pub hover: Option<fn(Point, &mut Bar) -> ()>,
    pub mouse_off: Option<fn(Point, &mut Bar) -> ()>,
    pub click: Option<fn(Point, &mut Bar) -> ()>,
    pub buttons: Vec<Button>,
}

#[derive(Debug, Clone)]
pub struct AsyncBar {
    pub pos: Point,
    w: u32,
    h: u32,
    pub padding: u32,
    pub color: Option<Color>,
    orig_color: Option<Color>,
    pub is_hovering: bool,
    pub buttons: Vec<Button>,
}

impl Bar {
    pub fn new(
        x: i32,
        y: i32,
        padding: u32,
        color: Option<Color>,
        hover: Option<fn(Point, &mut Bar) -> ()>,
        mouse_off: Option<fn(Point, &mut Bar) -> ()>,
        click: Option<fn(Point, &mut Bar) -> ()>,
        buttons: Vec<Button>,
    ) -> Bar {
        let w = buttons.iter().fold(0, |acc, b| b.w + acc + padding) + padding;
        let h = buttons.iter().fold(0, |acc, b| std::cmp::max(acc, b.h)) + 2 * padding;
        Bar {
            pos: Point::new(x, y),
            w,
            h,
            padding,
            color,
            orig_color: color,
            is_hovering: false,
            hover,
            mouse_off,
            click,
            buttons,
        }
    }

    pub fn sample() -> Bar {
        Bar::new(
            0,
            0,
            1,
            Some(sdl2::pixels::Color::GRAY),
            Some(sample_hover),
            None,
            Some(sample_click),
            vec![Button::sample(), Button::sample(), Button::sample()],
        )
    }
}

impl Drawable for Bar {
    fn draw(&self, canvas: &mut Canvas<Window>, hidpi_scale: u32) {
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
            if let Some(c) = b.color {
                canvas.set_draw_color(c);
            }
            b.draw(canvas, s_u, curr_point);
            canvas.set_draw_color(curr_color);
            curr_point += Point::new((b.w + self.padding) as i32 * s_i, 0);
        }
        canvas.set_draw_color(previous_color);
    }
}

pub fn mouse_within_button(mouse_x: i32, mouse_y: i32, button_pos: Point, button: &Button) -> bool {
    mouse_x >= button_pos.x
        && mouse_x <= button_pos.x + button.w as i32
        && mouse_y >= button_pos.y
        && mouse_y <= button_pos.y + button.h as i32
}

pub fn sample_hover(mouse_pos: Point, bar: &mut Bar) {
    let mut curr_button_location = Point::new(
        bar.pos.x + bar.padding as i32,
        bar.pos.y + bar.padding as i32,
    );
    for mut b in bar.buttons.iter_mut() {
        let mouse_is_over = mouse_within_button(mouse_pos.x, mouse_pos.y, curr_button_location, &b);
        let mouse_on = mouse_is_over && !b.is_hovering;
        let mouse_off = !mouse_is_over && b.is_hovering;
        if mouse_on {
            b.is_hovering = true;
            b.hover();
        } else if mouse_off {
            b.is_hovering = false;
            b.mouse_off();
        }
        curr_button_location += Point::new((b.w + bar.padding) as i32, 0);
    }
}

pub fn sample_click(mouse_pos: Point, bar: &mut Bar) {
    let mut curr_button_location = Point::new(
        bar.pos.x + bar.padding as i32,
        bar.pos.y + bar.padding as i32,
    );
    for mut b in bar.buttons.iter_mut() {
        let mouse_on = mouse_within_button(mouse_pos.x, mouse_pos.y, curr_button_location, &b);
        if mouse_on {
            b.click();
        }
        curr_button_location += Point::new((b.w + bar.padding) as i32, 0);
    }
}

pub async fn sample_async_click(mouse_pos: Point, bar: &mut Bar) {
    let mut curr_button_location = Point::new(
        bar.pos.x + bar.padding as i32,
        bar.pos.y + bar.padding as i32,
    );
    for mut b in bar.buttons.iter_mut() {
        let mouse_on = mouse_within_button(mouse_pos.x, mouse_pos.y, curr_button_location, &b);
        if mouse_on {
            b.click();
        }
        curr_button_location += Point::new((b.w + bar.padding) as i32, 0);
    }
}
