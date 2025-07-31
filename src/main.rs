extern crate sdl2;

mod button;
mod bar;
mod chat_client;
mod layout;
mod text;
mod chat;

use chat_client::*;
use layout::Layout;
use chat::ChatState;
use sdl2::pixels::Color;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::path::Path;
use std::time::Duration;

#[tokio::main]
pub async fn main() -> reqwest::Result<()> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let user = User::new("jonathan_er_sej".to_string(), "0.0.0.0".to_string());

    let window = video_subsystem.window("chitchat-rs", 800, 600)
        .position_centered()
        .allow_highdpi()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(44, 47, 51));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    // Enable text input
    video_subsystem.text_input().start();

    let font_context = sdl2::ttf::init().expect("failed to create font context");
    let path = Path::new("fonts/Andale Mono.ttf");
    let font = font_context.load_font(path, 40).expect("couldn't load font"); // 2x larger text
    
    let texture_creator = canvas.texture_creator();

    let (mut render_width, mut render_height) = canvas.window().drawable_size();
    let (mut win_width, mut win_height) = canvas.window().size();
    if render_width / win_width != render_height / win_height { panic!("what the heck?"); }
    let mut hidpi_scale = render_width / win_width;

    let mut layout = Layout::sample(user.clone());
    let mut server_info = ServerInfo::default();
    let mut chat_state = ChatState::new();

    'running: loop {
        // Get current render size (actual drawable pixels)
        let (render_width, render_height) = canvas.window().drawable_size();
        let contact_list_width = render_width / 4; // 25% of screen width
        let chat_area_x = contact_list_width + 20;
        let chat_area_width = render_width - chat_area_x;

        canvas.set_draw_color(Color::RGB(44, 47, 51));
        canvas.clear();
        
        // Draw contact list background (left side)
        canvas.set_draw_color(Color::RGB(47, 49, 54));
        canvas.fill_rect(Rect::new(0, 0, contact_list_width, render_height)).expect("couldn't draw contact list");
        
        // SERVER GUY BUTTON - scaled coordinates
        let button_padding = 20i32;
        let button_x = button_padding;
        let button_y = button_padding;
        let button_w = contact_list_width as i32 - (button_padding * 2);
        let button_h = 100i32; // Larger button height
        

        
        let button_color = if chat_state.selected_contact == Some(0) {
            Color::RGB(74, 144, 226) // Blue when selected
        } else {
            Color::RGB(64, 68, 75)   // Gray when not selected
        };
        
        canvas.set_draw_color(button_color);
        canvas.fill_rect(Rect::new(button_x, button_y, button_w as u32, button_h as u32)).unwrap();
        
        // Draw button border
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_rect(Rect::new(button_x, button_y, button_w as u32, button_h as u32)).unwrap();
        
        // Draw "SERVER GUY" text centered in button
        let button_text = font.render("SERVER GUY").solid(Color::WHITE).unwrap();
        let button_texture = button_text.as_texture(&texture_creator).unwrap();
        let button_query = button_texture.query();
        let text_x = button_x + (button_w - button_query.width as i32) / 2;
        let text_y = button_y + (button_h - button_query.height as i32) / 2;
        canvas.copy(&button_texture, None, 
            Rect::new(text_x, text_y, button_query.width, button_query.height)).unwrap();
        
        // Draw messages if contact is selected
        if let Some(contact_idx) = chat_state.selected_contact {
            let contact_name = &chat_state.contacts[contact_idx].name;
            
            // Calculate chat area dimensions
            let header_height = 70;
            let input_area_height = 80;
            let chat_start_y = header_height;
            let chat_area_bottom = render_height as i32 - input_area_height;
            
            // Draw messages with scroll offset
            let mut y_offset = chat_start_y + chat_state.scroll_offset;
            
            for message in &chat_state.messages {
                let message_height = 120; // Larger message height for bigger text
                if y_offset + message_height > chat_start_y && y_offset < chat_area_bottom {
                    message.draw_bubble(Point::new(chat_area_x as i32, y_offset), &mut canvas, &font, chat_area_width - 40, &texture_creator);
                }
                y_offset += message_height;
            }
            
            // Draw contact name header on top (with background)
            canvas.set_draw_color(Color::RGB(44, 47, 51)); // Match main background
            canvas.fill_rect(Rect::new(chat_area_x as i32, 0, chat_area_width, header_height as u32)).unwrap();
            
            let header_surface = font.render(contact_name).solid(Color::WHITE).unwrap();
            let header_texture = header_surface.as_texture(&texture_creator).unwrap();
            let header_query = header_texture.query();
            canvas.copy(&header_texture, None, 
                Rect::new(chat_area_x as i32 + 20, 20, header_query.width, header_query.height)).unwrap();
        } else {
            // Draw instructions if no contact selected
            let instruction_text = "Click 'Server Guy' to start chatting!";
            let instruction_surface = font.render(instruction_text).solid(Color::RGB(150, 150, 150)).unwrap();
            let instruction_texture = instruction_surface.as_texture(&texture_creator).unwrap();
            let instruction_query = instruction_texture.query();
            let center_x = chat_area_x as i32 + (chat_area_width as i32 - instruction_query.width as i32) / 2;
            let center_y = render_height as i32 / 2;
            canvas.copy(&instruction_texture, None, 
                Rect::new(center_x, center_y, instruction_query.width, instruction_query.height)).unwrap();
        }
        
        // Draw text input area
        let input_area_height = 80;
        let input_y = render_height as i32 - input_area_height;
        canvas.set_draw_color(Color::RGB(64, 68, 75));
        canvas.fill_rect(Rect::new(chat_area_x as i32, input_y, chat_area_width, input_area_height as u32)).expect("couldn't draw input area");
        
        // Draw input text
        let input_display = if chat_state.input_text.is_empty() {
            "Type a message...".to_string()
        } else {
            chat_state.input_text.clone()
        };
        
        let input_color = if chat_state.input_text.is_empty() {
            Color::RGB(114, 118, 125)
        } else {
            Color::WHITE
        };
        
        let input_surface = font.render(&input_display).solid(input_color).unwrap();
        let input_texture = input_surface.as_texture(&texture_creator).unwrap();
        let input_query = input_texture.query();
        canvas.copy(&input_texture, None, 
            Rect::new(chat_area_x as i32 + 20, input_y + 20, input_query.width, input_query.height)).unwrap();
        
        // Draw server status button (floating on top right)
        let status_button_width = 150;
        let status_button_height = 40;
        let status_button_x = (render_width as i32) - status_button_width - 20;
        let status_button_y = 20;
        
        canvas.set_draw_color(Color::RGB(67, 181, 129)); // Green for online
        canvas.fill_rect(Rect::new(status_button_x, status_button_y, status_button_width as u32, status_button_height as u32)).unwrap();
        
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_rect(Rect::new(status_button_x, status_button_y, status_button_width as u32, status_button_height as u32)).unwrap();
        
        let status_text = font.render("Server Online").solid(Color::WHITE).unwrap();
        let status_texture = status_text.as_texture(&texture_creator).unwrap();
        let status_query = status_texture.query();
        let status_text_x = status_button_x + (status_button_width - status_query.width as i32) / 2;
        let status_text_y = status_button_y + (status_button_height - status_query.height as i32) / 2;
        canvas.copy(&status_texture, None, 
            Rect::new(status_text_x, status_text_y, status_query.width, status_query.height)).unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseMotion { x, y, .. } => {
                    layout.hover(x, y);
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    // Get current render size for click detection
                    let (render_width, _) = canvas.window().drawable_size();
                    let contact_list_width = render_width / 4;
                    
                    // SERVER GUY BUTTON - convert drawing coordinates to logical coordinates
                    let button_padding = 20i32;
                    let button_x = button_padding / hidpi_scale as i32;
                    let button_y = button_padding / hidpi_scale as i32;
                    let button_w = (contact_list_width as i32 - (button_padding * 2)) / hidpi_scale as i32;
                    let button_h = 100i32 / hidpi_scale as i32;
                    
                    if x >= button_x && x < button_x + button_w &&
                       y >= button_y && y < button_y + button_h {
                        if chat_state.selected_contact == Some(0) {
                            chat_state.selected_contact = None;
                        } else {
                            chat_state.select_contact(0);
                        }
                    }
                }
                Event::TextInput { text, .. } => {
                    for ch in text.chars() {
                        chat_state.handle_char_input(ch);
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    chat_state.send_message();
                }
                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                    chat_state.handle_backspace();
                }
                Event::MouseWheel { y, .. } => {
                    if chat_state.selected_contact.is_some() {
                        if y > 0 {
                            chat_state.scroll_up();
                        } else if y < 0 {
                            chat_state.scroll_down();
                        }
                    }
                }
                Event::Window { win_event: WindowEvent::Resized(_, _), .. } => {
                    // Window resizing is handled automatically by getting fresh dimensions each frame
                }
                _ => {}
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
