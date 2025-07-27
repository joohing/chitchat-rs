use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>, hidpi_scale: u32);
}

pub trait PosDrawable {
    fn draw(&self, canvas: &mut Canvas<Window>, hidpi_scale: u32, pos: Point);
}

pub trait Hoverable {
    fn hover(&mut self);
    fn mouse_off(&mut self);
}

pub trait Clickable {
    fn click(&mut self);
}

pub trait AsyncClickable {
    async fn click(&mut self);
}

#[derive(Debug, Clone, Copy)]
pub struct Button {
    pub w: u32,
    pub h: u32,
    pub color: Option<Color>,
    orig_color: Option<Color>,
    pub is_hovering: bool,
}

impl Button {
    pub fn new(w: u32, h: u32, color: Option<Color>) -> Button {
        Button {
            w,
            h,
            color,
            orig_color: color,
            is_hovering: false,
        }
    }

    pub fn sample() -> Button {
        Button::new(25, 10, Some(sdl2::pixels::Color::GRAY))
    }
}

impl PosDrawable for Button {
    fn draw(&self, canvas: &mut Canvas<Window>, hidpi_scale: u32, pos: Point) {
        let (s_u, s_i) = (hidpi_scale as u32, hidpi_scale as i32);
        let previous_color = canvas.draw_color();
        if let Some(c) = self.color {
            canvas.set_draw_color(c);
        }
        let res = canvas.fill_rect(Rect::new(pos.x, pos.y, self.w * s_u, self.h * s_u));
        canvas.set_draw_color(previous_color);
        if res.is_err() {
            panic!("{:?}", res.unwrap());
        }
    }
}

impl Hoverable for Button {
    fn hover(&mut self) {
        button_darken_on_hover(self);
    }

    fn mouse_off(&mut self) {
        button_restore_color(self);
    }
}

impl Clickable for Button {
    fn click(&mut self) {
        button_print_on_click(self);
    }
}

pub fn button_print_on_click(b: &mut Button) {
    println!("Clicked!");
}

pub fn button_print_on_hover(b: &mut Button) {
    println!("Hover!");
}

pub fn button_darken_on_hover(b: &mut Button) {
    b.is_hovering = true;
    if let Some(c) = b.color {
        let darkened_color = Color::RGB(
            (c.r as f32 * 0.8) as u8,
            (c.g as f32 * 0.8) as u8,
            (c.b as f32 * 0.8) as u8,
        );
        b.color = Some(darkened_color);
    }
}

pub fn button_restore_color(b: &mut Button) {
    if let Some(c) = b.orig_color {
        b.color = b.orig_color;
    }
}
