use sdl2::{pixels::Color, render::Canvas, video::Window, rect::{Rect, Point}};

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>, hidpi_scale: u32);
}

pub trait PosDrawable {
    fn draw(&self, canvas: &mut Canvas<Window>, hidpi_scale: u32, pos: Point);
}

#[derive(Debug, Clone, Copy)]
pub struct Button {
    pub w: u32,
    pub h: u32,
    pub color: Option<Color>,
    orig_color: Option<Color>,
    pub is_hovering: bool,
    pub hover: Option<fn(&mut Button) -> ()>,
    pub mouse_off: Option<fn(&mut Button) -> ()>,
    pub click: Option<fn(&mut Button) -> ()>,
}

impl Button {
    pub fn new(
        w: u32,
        h: u32,
        color: Option<Color>,
        hover: Option<fn(&mut Button) -> ()>,
        mouse_off: Option<fn(&mut Button) -> ()>,
        click: Option<fn(&mut Button) -> ()>
    ) -> Button {
        Button { w, h, color, orig_color: color, is_hovering: false, hover, mouse_off, click }
    }

    pub fn sample() -> Button {
        Button::new(25, 10,
            Some(sdl2::pixels::Color::GRAY),
            Some(button_darken_on_hover),
            Some(button_restore_color),
            Some(button_print_on_click)
        )
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

pub fn button_print_on_click(b: &mut Button) { println!("Clicked!"); }

pub fn button_print_on_hover(b: &mut Button) { println!("Hover!"); }

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
