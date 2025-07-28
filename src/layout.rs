use sdl2::{rect::Point, render::Canvas, video::Window};

use crate::bar::Bar;

pub struct Layout {
    bars: Vec<Bar>,
}

impl Layout {
    pub fn sample() -> Self {
        let _1 = Bar::sample();
        let Bar {
            pos,
            padding,
            color,
            buttons,
            ..
        } = Bar::sample();
        let _2 = Bar::new(pos.x, pos.y + _1.h as i32, padding, color, buttons.clone());
        let _3 = Bar::new(pos.x, _2.pos.y + _2.h as i32, padding, color, buttons);
        Self {
            bars: vec![_1, _2, _3],
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, hidpi_scale: u32) {
        for b in &self.bars {
            b.draw(canvas, hidpi_scale);
        }
    }

    pub async fn click(&mut self, pos: Point) -> reqwest::Result<()> {
        for b in self.bars.iter_mut().filter(|b| mouse_within_bar(pos.x, pos.y, b))
        {
            b.click(pos).await?;
        }
        Ok(())
    }

    pub fn hover(&mut self, x: i32, y: i32) {
        for b in self.bars.iter_mut() {
            if mouse_within_bar(x, y, b) {
                b.hover(x, y);
            } else {
                println!("unhovering, mouse at ({}, {})", x, y);
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
