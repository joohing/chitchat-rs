use sdl2::{pixels::Color, rect::{Point, Rect}, render::Canvas, video::Window, ttf::Font};
use crate::chat_client::ServerInfo;

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

    pub fn draw(&self, pos: Point, canvas: &mut Canvas<Window>, font: &Font) {
        let partial = font.render(self.content.as_str());
        let surface = partial.solid(Color::WHITE).unwrap();
        let tc = canvas.texture_creator();
        let texture = surface.as_texture(&tc).unwrap();
        let info = texture.query();
        canvas.copy(&texture, None, Rect::new(pos.x, pos.y, info.width, info.height)).unwrap();
    }

    pub fn draw_server_info(server_info: &ServerInfo, pos: Point, canvas: &mut Canvas<Window>, font: &Font) {
        let text = format!("Player count: {}, Status: {}", server_info.player_count, server_info.status);
        let color = match server_info.status.as_str() {
            "online" => Color::RGB(67, 181, 129),  // Green for online
            "offline" => Color::RGB(250, 166, 26),  // Orange for offline  
            _ => Color::RGB(237, 66, 69),           // Red for error
        };
        let partial = font.render(&text);
        let surface = partial.solid(color).unwrap();
        let tc = canvas.texture_creator();
        let texture = surface.as_texture(&tc).unwrap();
        let info = texture.query();
        canvas.copy(&texture, None, Rect::new(pos.x, pos.y, info.width, info.height)).unwrap();
    }
}
