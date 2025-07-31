use sdl2::{pixels::Color, rect::{Point, Rect}, render::{Canvas, TextureCreator}, video::{Window, WindowContext}, ttf::Font};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Message {
    pub text: String,
    pub sender: String,
    pub timestamp: u64,
    pub is_own_message: bool,
}

#[derive(Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub last_message: Option<String>,
    pub is_online: bool,
}

#[derive(Debug)]
pub struct ChatState {
    pub contacts: Vec<Contact>,
    pub selected_contact: Option<usize>,
    pub messages: Vec<Message>,
    pub input_text: String,
    pub is_typing: bool,
    pub scroll_offset: i32,
}

impl Message {
    pub fn new(text: String, sender: String, is_own_message: bool) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            text,
            sender,
            timestamp,
            is_own_message,
        }
    }
    
    pub fn draw_bubble(&self, pos: Point, canvas: &mut Canvas<Window>, font: &Font, max_width: u32, texture_creator: &TextureCreator<WindowContext>) {
        let bubble_color = if self.is_own_message {
            Color::RGB(74, 144, 226) // Blue for own messages
        } else {
            Color::RGB(64, 68, 75)   // Gray for others
        };
        
        let text_color = Color::WHITE;
        
        // Render text
        let text_surface = font.render(&self.text).solid(text_color).unwrap();
        let text_texture = text_surface.as_texture(texture_creator).unwrap();
        let text_query = text_texture.query();
        
        let padding = 12;
        let bubble_width = (text_query.width + padding * 2).min(max_width);
        let bubble_height = text_query.height + padding * 2;
        
        // Position bubble (right align for own messages)
        let bubble_x = if self.is_own_message {
            pos.x + max_width as i32 - bubble_width as i32
        } else {
            pos.x
        };
        
        // Draw bubble background
        canvas.set_draw_color(bubble_color);
        canvas.fill_rect(Rect::new(bubble_x, pos.y, bubble_width, bubble_height)).unwrap();
        
        // Draw text
        let text_x = bubble_x + padding as i32;
        let text_y = pos.y + padding as i32;
        canvas.copy(&text_texture, None, Rect::new(text_x, text_y, text_query.width, text_query.height)).unwrap();
    }
}

impl Contact {
    pub fn new(name: String) -> Self {
        Self {
            name,
            last_message: None,
            is_online: false,
        }
    }
    
    pub fn draw(&self, pos: Point, canvas: &mut Canvas<Window>, font: &Font, is_selected: bool, texture_creator: &TextureCreator<WindowContext>) {
        // Define exact button dimensions (larger for full screen)
        let button_width = 400; // Larger width
        let button_height = 100; // Larger height
        
        let bg_color = if is_selected {
            Color::RGB(74, 144, 226) // Blue when selected
        } else {
            Color::RGB(64, 68, 75)   // Dark gray when not selected
        };
        
        // Draw button background
        canvas.set_draw_color(bg_color);
        canvas.fill_rect(Rect::new(pos.x, pos.y, button_width, button_height)).unwrap();
        
        // Draw button border
        canvas.set_draw_color(Color::RGB(114, 118, 125));
        canvas.draw_rect(Rect::new(pos.x, pos.y, button_width, button_height)).unwrap();
        
        // Draw contact name centered in button
        let text_color = Color::WHITE;
        let name_surface = font.render(&self.name).solid(text_color).unwrap();
        let name_texture = name_surface.as_texture(texture_creator).unwrap();
        let name_query = name_texture.query();
        
        // Center the text in the button
        let text_x = pos.x + (button_width as i32 - name_query.width as i32) / 2;
        let text_y = pos.y + (button_height as i32 - name_query.height as i32) / 2;
        
        canvas.copy(&name_texture, None, 
            Rect::new(text_x, text_y, name_query.width, name_query.height)).unwrap();
        
        // Draw online indicator (small circle in top-right corner)
        let indicator_color = if self.is_online {
            Color::RGB(67, 181, 129) // Green when online
        } else {
            Color::RGB(116, 127, 141) // Gray when offline
        };
        canvas.set_draw_color(indicator_color);
        canvas.fill_rect(Rect::new(pos.x + button_width as i32 - 15, pos.y + 5, 8, 8)).unwrap();
    }
    
    // Return the exact clickable area for this contact
    pub fn get_click_rect(&self, pos: Point) -> Rect {
        Rect::new(pos.x, pos.y, 400, 100)
    }
}

impl ChatState {
    pub fn new() -> Self {
        let mut contacts = Vec::new();
        let mut server_guy = Contact::new("Server Guy".to_string());
        server_guy.is_online = true;
        server_guy.last_message = Some("Ready to chat!".to_string());
        contacts.push(server_guy);
        
        Self {
            contacts,
            selected_contact: None,
            messages: Vec::new(),
            input_text: String::new(),
            is_typing: false,
            scroll_offset: 0,
        }
    }
    
    pub fn select_contact(&mut self, index: usize) {
        if index < self.contacts.len() {
            self.selected_contact = Some(index);
            self.messages.clear();
            self.scroll_offset = 0; // Reset scroll when changing contacts
            
            // Add some sample messages for server guy
            if index == 0 {
                self.messages.push(Message::new(
                    "Hello! I'm the server guy. How can I help you today?".to_string(),
                    "Server Guy".to_string(),
                    false,
                ));
                self.messages.push(Message::new(
                    "You can ask me about server status or just chat!".to_string(),
                    "Server Guy".to_string(),
                    false,
                ));
            }
        }
    }
    
    pub fn add_message(&mut self, text: String, is_own: bool) {
        if let Some(contact_idx) = self.selected_contact {
            let sender = if is_own {
                "You".to_string()
            } else {
                self.contacts[contact_idx].name.clone()
            };
            
            let message = Message::new(text.clone(), sender, is_own);
            self.messages.push(message);
            
            // Update last message for contact
            self.contacts[contact_idx].last_message = Some(text);
        }
    }
    
    pub fn send_message(&mut self) {
        if !self.input_text.trim().is_empty() {
            let text = self.input_text.clone();
            self.add_message(text, true);
            self.input_text.clear();
            
            // Auto-scroll to bottom after sending
            self.auto_scroll_to_bottom();
            
            // Auto-reply from server guy
            if self.selected_contact == Some(0) {
                let responses = [
                    "That's interesting! Tell me more.",
                    "I see! Anything else on your mind?",
                    "Cool! How's your day going?",
                    "Nice! The server is running smoothly, by the way.",
                    "Thanks for chatting with me!",
                ];
                
                let response = responses[self.messages.len() % responses.len()];
                self.add_message(response.to_string(), false);
                
                // Auto-scroll to bottom after reply
                self.auto_scroll_to_bottom();
            }
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
        let chat_area_height = 600; // Larger for full screen
        let message_height = 120; // Updated for larger messages
        let total_messages_height = self.messages.len() as i32 * message_height;
        let min_scroll = (chat_area_height - total_messages_height).min(0);
        self.scroll_offset = (self.scroll_offset - 40).max(min_scroll);
    }
    
    pub fn scroll_down(&mut self) {
        self.scroll_offset = (self.scroll_offset + 40).min(0);
    }
    
    pub fn auto_scroll_to_bottom(&mut self) {
        // Calculate scroll offset to show messages at the bottom
        let chat_area_height = 600; // Larger for full screen
        let message_height = 120; // Updated for larger messages
        let total_messages_height = self.messages.len() as i32 * message_height;
        
        if total_messages_height > chat_area_height {
            // If messages overflow, scroll to show the latest ones
            self.scroll_offset = chat_area_height - total_messages_height;
        } else {
            // If messages fit, keep them at the top
            self.scroll_offset = 0;
        }
    }
}
