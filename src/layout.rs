use sdl2::{rect::Point, render::Canvas, ttf::Font, video::Window};

use crate::button::Buttons;
use crate::bar::Bar;
use crate::chat_client::{User, ServerInfo};
use crate::chat::*;

pub struct Layout {
    user: User,
    bars: Vec<Bar>,
    contacts: Vec<Contact>,
    pub selected_contact: Option<Contact>,
    pub chat_state: Option<ChatState>,
}

impl Layout {
    pub fn new(user: User, bars: Vec<Bar>, contacts: Vec<Contact>) -> Self {
        let selected_contact = None;
        let chat_state = None;
        Self { user, bars, contacts, selected_contact, chat_state }
    }

    pub fn sample(user: User) -> Self {
        let header_bar = Bar::new(
            0,
            0,
            15,
            Some(sdl2::pixels::Color::RGB(54, 57, 63)),
            vec![],
        );

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
            contacts: vec![Contact::sample()],
            selected_contact: None,
            chat_state: None,
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

    pub fn select_contact(&mut self, index: usize) {
        if index < self.contacts.len() {
            self.selected_contact = Some(self.contacts[index].clone());
            let mut chat_state = ChatState::new(self.contacts[index].clone());
            chat_state.messages = self.contacts[index].history.clone();
            chat_state.auto_scroll_to_bottom();
            self.chat_state = Some(chat_state);
            println!("selected contact with name: {}", self.contacts[index].name);
        } else { panic!("index outside contacts len!"); }
    }

    pub fn deselect_contact(&mut self) {
        self.selected_contact = None;
    }

    pub fn add_message(&mut self, text: String, is_own: bool) {
        if let Some(contact) = &self.selected_contact && let Some(_) = &self.chat_state {
            let message = Message::new(text.clone(), is_own);
            self.chat_state.as_mut().unwrap().messages.push(message.clone());
            let c: &mut Contact = self.contacts.iter_mut().find(|c: &&mut Contact| c.addr == contact.addr).unwrap();
            c.history.push(message);
        }
    }

    /// When user presses enter in the text field
    pub fn send_message(&mut self) {
        println!("sending...");
        if let Some(chat_state) = &self.chat_state {
            if chat_state.input_text.trim().is_empty() { println!("empty message"); return; }
            let text = chat_state.input_text.clone();
            self.add_message(text, true);
            self.chat_state.as_mut().unwrap().input_text.clear();
            self.chat_state.as_mut().unwrap().auto_scroll_to_bottom();

            // testing code to have automatic looping responses from server guy
            if self.selected_contact == Some(Contact::sample()) {
                self.receive_message();
            }
        } else { panic!("invalid state - trying to send message without open chat"); }
    }

    pub fn receive_message(&mut self) {
        let responses = [
            "That's interesting! Tell me more.",
            "I see! Anything else on your mind?",
            "Cool! How's your day going?",
            "Nice! The server is running smoothly, by the way.",
            "Thanks for chatting with me!",
        ];

        let response = responses[self.contacts[0].history.len() % responses.len()];
        self.add_message(response.to_string(), false);

        self.chat_state.as_mut().unwrap().auto_scroll_to_bottom();
    }
}

fn mouse_within_bar(mouse_x: i32, mouse_y: i32, bar: &Bar) -> bool {
    let (w, h) = (bar.w, bar.h);
    let (x, y) = (bar.pos.x, bar.pos.y);
    mouse_x >= x && mouse_x <= x + w as i32 && mouse_y >= y && mouse_y <= y + h as i32
}
