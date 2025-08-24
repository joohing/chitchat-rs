// In: some layout and a canvas
// Out: SizeInfo
// Result: layout drawn on canvas.
mod render;

// Relates the contacts and the chat to each other: contains info about their current contents.
// A layout is a contacts pane and a chat pane.
mod layout;

// Contacts pane and contacts, as well as a plus button.
mod contacts;

// Chat pane and messages, as well as input bar and title of chat.
mod chat;

extern crate sdl2;

use sdl2::{EventPump, event::{Event, WindowEvent}, keyboard::{Keycode, TextInputUtil}, render::Canvas, video::Window, ttf::Font};
use std::{path::Path, time::Duration};
use crate::{layout::Layout, render::SizeInfo};

// Events that may get returned from event handlers that are called in the event_loop function.
#[derive(PartialEq)]
enum Events {
	Quit,
	Click,
	Resized,
	Scrolled,
	Text,
	SentMessage,
	Unhandled,
}

/// Creates the long-lived variables and starts the event loop.
pub fn main() {
	let sdl_context = sdl2::init().unwrap();
	let ttf_context = sdl2::ttf::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let txt_input = video_subsystem.text_input();

	let window = video_subsystem
		.window("chitchat", 800, 600)
		.position_centered()
		.resizable()
		.allow_highdpi()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut layout = Layout::sample();
	let font = ttf_context.load_font(Path::new("fonts/Andale Mono.ttf"), 30).expect("could not load font");

	event_loop(&mut layout, &mut canvas, &font, &mut event_pump, &txt_input);
}

fn event_loop(layout: &mut Layout, canvas: &mut Canvas<Window>, font: &Font, event_pump: &mut EventPump, txt_input: &TextInputUtil) {
	let mut size_info = SizeInfo::from_canvas(canvas);
	let mut current_input = String::new();
	let mut hidpi_scaling = (canvas.window().drawable_size().0 / canvas.window().size().0) as i32;
	render::render_layout(canvas, layout, font, &mut size_info, &current_input, txt_input);

	'running: loop {
		for event in event_pump.poll_iter() {
			if Events::Quit == handle_event(layout, canvas, font, &event, &mut size_info, hidpi_scaling, &mut current_input, txt_input) {
				break 'running;
			}
		}
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}

fn handle_event(
	layout: &mut Layout, 
	canvas: &mut Canvas<Window>, 
	font: &Font, 
	event: &sdl2::event::Event, 
	size_info: &mut SizeInfo, 
	hidpi_scaling: i32,
	current_input: &mut String, 
	txt_input: &TextInputUtil
) -> Events {
	size_info.update_from_canvas_and_layout(canvas, layout, font, hidpi_scaling);
	let ret = match event {
		Event::Quit { .. }
		| Event::KeyDown {
			keycode: Some(Keycode::Escape),
			..
		} => Events::Quit,
		Event::MouseWheel { mouse_x, mouse_y, y, .. } => {
			layout.scroll(mouse_x, mouse_y, size_info, y, hidpi_scaling);
			Events::Scrolled
		}
		Event::MouseButtonDown { x, y, .. } => {
			layout.click(x, y, size_info, hidpi_scaling);
			Events::Click
		}
		Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
			layout.send(size_info, current_input);
			*current_input = String::new();
			Events::SentMessage
		}
		Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
			current_input.pop();
			Events::Text
		}
		Event::TextInput { text, .. } => {
			current_input.push_str(text.as_str());
			Events::Text
		}
		_ => Events::Unhandled,
	};
	render::render_layout(canvas, layout, font, size_info, current_input, txt_input);
	ret
}
