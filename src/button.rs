use sdl2::{pixels::Color, render::Canvas, video::Window, rect::{Rect, Point}};

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}

pub trait Clickable<I, O> {
    fn click(&self, i: I) -> O;
}

pub struct Button {
    pub pos: Point,
    pub w: u32,
    pub h: u32,
    pub color: Option<Color>,
}

impl Button {
    pub fn new(x: i32, y: i32, w: u32, h: u32, color: Option<Color>) -> Button {
        Button { pos: Point::new(x, y), w, h, color }
    }
}

impl Drawable for Button {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        if let Some(c) = self.color {
            canvas.set_draw_color(c);
        }
        let res = canvas.fill_rect(Rect::new(self.pos.x, self.pos.y, self.w, self.h));
        if res.is_err() {
            panic!("{:?}", res.unwrap());
        }
    }
}

impl Clickable<Point, bool> for Button {
    fn click(&self, i: Point) -> bool {
        println!("Clicked!");
        true
    }
}
