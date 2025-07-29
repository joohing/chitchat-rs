use sdl2::{pixels::Color, rect::{Point, Rect}, render::Canvas, surface::Surface, ttf::Font, video::Window};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Text {
    font: String,
    font_size: u16,
    content: String,
}

impl Text {
    pub fn new(font: String, font_size: u16, content: String) -> Self {
        Self { font, font_size, content }
    }

    pub fn draw(&self, pos: Point, canvas: &mut Canvas<Window>) {
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context.load_font(Path::new(&self.font), self.font_size).unwrap();
        let partial = font.render(self.content.as_str());
        let surface = partial.solid(Color::BLACK).unwrap();
        let tc = canvas.texture_creator();
        let texture = surface.as_texture(&tc).unwrap();
        let info = texture.query();
        canvas.copy(&texture, None, Rect::new(pos.x, pos.y, info.width, info.height)).unwrap();
        drop(font);
    }
}
