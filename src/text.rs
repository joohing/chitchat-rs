use sdl2::{pixels::Color, rect::Rect, render::Canvas, surface::Surface, ttf::Font, video::Window};

pub struct Text {
    font: Font<'static, 'static>,
    font_size: u8,
    content: String,
    surface: Surface<'static>,
}

impl Text {
    pub fn new(font: Font<'static, 'static>, font_size: u8, content: String) -> Self {
        let surface = font.render(&content).solid(Color::BLACK).unwrap();
        Self { font, font_size, content, surface }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let tc = canvas.texture_creator();
        let texture = self.surface.as_texture(&tc).unwrap();
        let info = texture.query();
        canvas.copy(&texture, None, Rect::new(50, 50, info.width, info.height)).unwrap();
        n
    }
}
