use sdl2::{rect::Point, render::Canvas, ttf::Font, video::Window};

use crate::button::Buttons;
use crate::bar::Bar;
use crate::chat_client::{User, ServerInfo};

pub struct Layout {
    user: User,
    bars: Vec<Bar>,
}

impl Layout {
    pub fn new(user: User, bars: Vec<Bar>) -> Self {
        Self { user, bars }
    }

    pub fn sample(user: User) -> Self {
        // Header bar with app title and server button in top right
        let header_bar = Bar::new(
            0,
            0,
            15,
            Some(sdl2::pixels::Color::RGB(54, 57, 63)),
            vec![],
        );

        // Server button bar positioned in top right
        let server_button_bar = Bar::new(
            640,
            10,
            8,
            Some(sdl2::pixels::Color::RGB(54, 57, 63)),
            vec![Buttons::sample_send(user.clone())],
        );

        Self {
            user,
            bars: vec![header_bar, server_button_bar],
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, hidpi_scale: u32, font: &Font) {
        for b in &self.bars {
            b.draw(canvas, hidpi_scale, font);
        }
    }

    pub async fn click(&mut self, pos: Point, server_info: &mut ServerInfo) -> reqwest::Result<()> {
        for b in self.bars.iter_mut().filter(|b| mouse_within_bar(pos.x, pos.y, b))
        {
            b.click(pos, &self.user, server_info).await?;
        }
        Ok(())
    }

    pub fn hover(&mut self, x: i32, y: i32) {
        for b in self.bars.iter_mut() {
            if mouse_within_bar(x, y, b) {
                b.hover(x, y);
            } else {
                b.unhover();
            }
        }
    }
}

fn mouse_within_bar(mouse_x: i32, mouse_y: i32, bar: &Bar) -> bool {
    let (w, h) = (bar.w, bar.h);
    let (x, y) = (bar.pos.x, bar.pos.y);
    mouse_x >= x && mouse_x <= x + w as i32 && mouse_y >= y && mouse_y <= y + h as i32
}
