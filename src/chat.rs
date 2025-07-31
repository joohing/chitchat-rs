use sdl2::{pixels::Color, rect::{Point, Rect}, render::{Canvas, TextureCreator}, video::{Window, WindowContext}, ttf::Font};

const SCROLL_STEP: i32 = 40;
const CHAT_AREA_HEIGHT: i32 = 600;
const MESSAGE_HEIGHT: i32 = 120;

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub text: String,
    pub is_own_message: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Contact {
    pub name: String,
    pub addr: String,
    pub history: Vec<Message>,
    pub draft: String,
}

#[derive(Debug)]
pub struct ChatState {
    pub messages: Vec<Message>,
    pub input_text: String,
    pub is_typing: bool,
    pub scroll_offset: i32,
}

impl Message {
    pub fn new(text: String, is_own_message: bool) -> Self {
        Self {
            text,
            is_own_message,
        }
    }

    pub fn draw_bubble(&self, pos: Point, canvas: &mut Canvas<Window>, font: &Font, max_width: u32, texture_creator: &TextureCreator<WindowContext>) {
        let bubble_color = if self.is_own_message {
            Color::RGB(74, 144, 226)
        } else {
            Color::RGB(64, 68, 75)
        };

        let text_color = Color::WHITE;

        let text_surface = font.render(&self.text).solid(text_color).unwrap();
        let text_texture = text_surface.as_texture(texture_creator).unwrap();
        let text_query = text_texture.query();

        let padding = 12;
        let bubble_width = (text_query.width + padding * 2).min(max_width);
        let bubble_height = text_query.height + padding * 2;

        let bubble_x = if self.is_own_message {
            pos.x + max_width as i32 - bubble_width as i32
        } else {
            pos.x
        };

        canvas.set_draw_color(bubble_color);
        canvas.fill_rect(Rect::new(bubble_x, pos.y, bubble_width, bubble_height)).unwrap();

        let text_x = bubble_x + padding as i32;
        let text_y = pos.y + padding as i32;
        canvas.copy(&text_texture, None, Rect::new(text_x, text_y, text_query.width, text_query.height)).unwrap();
    }
}

impl Contact {
    pub fn new(name: String, addr: String) -> Self {
        Self {
            name,
            addr,
            history: vec![],
            draft: "".to_string(),
        }
    }

    pub fn sample() -> Self {
        Self {
            name: "server guy".to_string(),
            addr: "194.163.183.44".to_string(),
            history: vec![],
            draft: "".to_string(),
        }
    }

    pub fn draw(&self, pos: Point, canvas: &mut Canvas<Window>, font: &Font, is_selected: bool, texture_creator: &TextureCreator<WindowContext>) {
        let button_width = 400;
        let button_height = 100;

        let bg_color = if is_selected {
            Color::RGB(74, 144, 226)
        } else {
            Color::RGB(64, 68, 75)
        };

        canvas.set_draw_color(bg_color);
        canvas.fill_rect(Rect::new(pos.x, pos.y, button_width, button_height)).unwrap();

        canvas.set_draw_color(Color::RGB(114, 118, 125));
        canvas.draw_rect(Rect::new(pos.x, pos.y, button_width, button_height)).unwrap();

        let text_color = Color::WHITE;
        let name_surface = font.render(&self.name).solid(text_color).unwrap();
        let name_texture = name_surface.as_texture(texture_creator).unwrap();
        let name_query = name_texture.query();

        let text_x = pos.x + (button_width as i32 - name_query.width as i32) / 2;
        let text_y = pos.y + (button_height as i32 - name_query.height as i32) / 2;

        canvas.copy(&name_texture, None, Rect::new(text_x, text_y, name_query.width, name_query.height)).unwrap();

        canvas.set_draw_color(Color::RGB(67, 181, 129));
        canvas.fill_rect(Rect::new(pos.x + button_width as i32 - 15, pos.y + 5, 8, 8)).unwrap();
    }

    pub fn get_click_rect(&self, pos: Point) -> Rect {
        Rect::new(pos.x, pos.y, 400, 100)
    }
}

impl ChatState {
    /// Take a contact and render the chat so far with them
    pub fn new(contact: Contact) -> Self {
        Self {
            messages: contact.history,
            input_text: contact.draft,
            is_typing: false,
            scroll_offset: 0,
        }
    }

    /// Sample with "server guy" at 194.163.183.44
    pub fn sample() -> Self {
        let mut contacts = Vec::new();
        let mut server_guy = Contact::new("server guy".to_string(), "194.163.183.44".to_string());
        server_guy.history = vec![Message::new("Haha yeah I'm server guy what's up".to_string(), false)];
        contacts.push(server_guy);

        Self {
            messages: Vec::new(),
            input_text: String::new(),
            is_typing: false,
            scroll_offset: 0,
        }
    }

    pub fn handle_char_input(&mut self, ch: char) {
        if ch.is_ascii() && ch != '\r' && ch != '\n' {
            self.input_text.push(ch);
        }
    }

    pub fn handle_backspace(&mut self) {
        self.input_text.pop();
    }

    pub fn scroll_up(&mut self) {
        // TODO don't define these here lol
        let total_messages_height = self.messages.len() as i32 * MESSAGE_HEIGHT;

        // Can't scroll more in negative (upwards) direction than what is required to display the very top of the chat
        let top_of_chat = (CHAT_AREA_HEIGHT - total_messages_height).min(0);
        self.scroll_offset = (self.scroll_offset - SCROLL_STEP).max(top_of_chat);
    }

    pub fn scroll_down(&mut self) {
        let total_messages_height = self.messages.len() as i32 * MESSAGE_HEIGHT;
        let bottom_of_chat = (total_messages_height - CHAT_AREA_HEIGHT).max(0);
        self.scroll_offset = (self.scroll_offset + SCROLL_STEP).min(bottom_of_chat);
    }

    pub fn auto_scroll_to_bottom(&mut self) {
        let total_messages_height = self.messages.len() as i32 * MESSAGE_HEIGHT;

        if total_messages_height > CHAT_AREA_HEIGHT {
            self.scroll_offset = CHAT_AREA_HEIGHT - total_messages_height;
        } else {
            self.scroll_offset = 0;
        }
    }
}
