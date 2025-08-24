use sdl2::{render::Canvas, video::Window, ttf::Font, rect::Rect, pixels::Color, rect::Point, keyboard::TextInputUtil};
use crate::{contacts::*, layout::*, chat::Message};

pub const CONTACT_PADDING: u32 = 10;
const CONTACT_PANE_COLOR: Color = Color::GREY;
const CONTACT_COLOR: Color = Color::RGB(145, 145, 145);
const CONTACT_COLOR_SEL: Color = Color::BLUE;
const CONTACT_PANE_BORDER_COLOR: Color = Color::WHITE;
const CHAT_PANE_COLOR: Color = Color::RGB(100, 100, 100);
const CHAT_PANE_TEXT_COLOR: Color = Color::WHITE;
const CHAT_TITLE_COLOR: Color = Color::RGB(120, 120, 120);
const CHAT_MESSAGE_OWN_COLOR: Color = Color::BLUE;
const CHAT_MESSAGE_COLOR: Color = Color::RGB(180, 180, 180);
const CHAT_MESSAGE_PADDING: u32 = 10;

pub struct SizeInfo {
	pub contact_pane: (u32, u32),
	pub contact: (u32, u32),
	pub plus_button: (u32, u32),
	pub chat_pane: (u32, u32),
	pub chat_message_max_width: u32,
	pub chat_total_height: i32,
	pub chat_title: (u32, u32),
	pub chat_input: (u32, u32),
}

impl SizeInfo {
	pub fn from_canvas(canvas: &Canvas<Window>) -> Self {
		let (w, h)                 = canvas.output_size().expect("could not get output size of canvas");
		let contact_pane           = (w / 5, h);
		let contact                = (w / 5 - CONTACT_PADDING * 2, 50);
		let plus_button            = (25, 25);
		let chat_pane              = (w - contact_pane.0, h);
		let chat_message_max_width = chat_pane.0 - chat_pane.0 / 3;
		let chat_title             = (chat_pane.0, 75);
		let chat_input             = (chat_pane.0, 75);
		let chat_total_height      = 0;
		Self { contact_pane, contact, plus_button, chat_pane, chat_message_max_width, chat_title, chat_input, chat_total_height }
	}

	pub fn update_from_canvas_and_layout(&mut self, canvas: &Canvas<Window>, layout: &Layout, font: &Font, hidpi_scaling: i32) {
		let updated                 = Self::from_canvas(canvas);
		self.contact_pane           = updated.contact_pane;
		self.contact                = updated.contact;
		self.plus_button            = updated.plus_button;
		self.chat_pane              = updated.chat_pane;
		self.chat_message_max_width = updated.chat_message_max_width;
		self.chat_title             = updated.chat_title;
		self.chat_input             = updated.chat_input;
		let chat_total_height = if let Some(ref ct) = layout.selected_contact {
			get_messages_height(&ct.history, self, font, hidpi_scaling)
		} else {
			0
		};
		self.chat_total_height      = chat_total_height;
	}
}

pub fn render_layout(
	canvas: &mut Canvas<Window>,
	layout: &mut Layout,
	font: &Font,
	size_info: &SizeInfo,
	current_input: &str,
	txt_input: &TextInputUtil,
) {
	render_contact_pane(canvas, &layout.contacts, &layout.selected_contact, size_info, font, layout.contact_scroll_state);
	if let Some(ref contact) = layout.selected_contact {
		if !txt_input.is_active() { txt_input.start(); }
		render_chat_pane(canvas, contact, font, size_info, current_input, layout.chat_scroll_state);
	} else {
		if txt_input.is_active() { txt_input.stop(); }
		render_welcome_screen(canvas, font, size_info);
	}
	canvas.present();
}

/// Use the contacts to draw a pane on the left of the screen.
fn render_contact_pane(
	canvas: &mut Canvas<Window>,
	contacts: &Vec<Contact>,
	selected_contact: &Option<Contact>,
	size_info: &SizeInfo,
	font: &Font,
	scroll_state: i32, // TODO use
) {
	let (pane_width, pane_height) = size_info.contact_pane;
	canvas.set_draw_color(CONTACT_PANE_COLOR);
	canvas.fill_rect(Rect::new(0, 0, pane_width, pane_height)).expect("could not draw on canvas in render_contact_pane");
	canvas.set_draw_color(CONTACT_PANE_BORDER_COLOR);
	canvas.draw_rect(Rect::new(0, 0, pane_width, pane_height)).expect("could not draw on canvas in render_contact_pane");

	let mut contact_curr_pos = Point::new(CONTACT_PADDING as i32, CONTACT_PADDING as i32);
	let (_, h) = size_info.contact;
	let pos_increase_each_iter = Point::new(0, (h + CONTACT_PADDING) as i32);
	if let Some(c) = selected_contact {
		for ct in contacts {
			render_contact(canvas, ct, font, &contact_curr_pos, size_info, c.addr == ct.addr);
			contact_curr_pos += pos_increase_each_iter;
		}
	} else {
		for ct in contacts {
			render_contact(canvas, ct, font, &contact_curr_pos, size_info, false);
			contact_curr_pos += pos_increase_each_iter;
		}
	}
}

fn render_contact(
	canvas: &mut Canvas<Window>, 
	contact: &Contact, 
	font: &Font,
	pos: &Point, 
	size_info: &SizeInfo, 
	is_selected: bool
) {
	let (w, h) = size_info.contact;
	let color = if is_selected { CONTACT_COLOR_SEL } else { CONTACT_COLOR };
	canvas.set_draw_color(color);
	canvas.fill_rect(Rect::new(pos.x, pos.y, w, h)).expect("could not fill rect for contact");

	let partial = font.render(contact.name.as_str());
	let solid = partial.solid(CHAT_PANE_TEXT_COLOR).unwrap();
	let tc = canvas.texture_creator();
	let texture = solid.as_texture(&tc).unwrap();
	let text_query = texture.query();

	let (name_x, name_y) = (pos.x + CONTACT_PADDING as i32, pos.y + CONTACT_PADDING as i32);

	canvas.copy(&texture, None, Rect::new(name_x, name_y, text_query.width, text_query.height)).unwrap();
}

/// Use the selected contact to draw a chat pane. Returns the height of all messages stacked in pixels.
fn render_chat_pane(
	canvas: &mut Canvas<Window>,
	selected_contact: &Contact,
	font: &Font,
	size_info: &SizeInfo,
	current_input: &str,
	scroll_state: i32,
) {
	let x_pos = size_info.contact_pane.0 as i32;
	let (pane_width, pane_height) = size_info.chat_pane;
	canvas.set_draw_color(CHAT_PANE_COLOR);
	canvas.fill_rect(Rect::new(x_pos, 0, pane_width, pane_height)).expect("could not fill rect for chat pane");

	let messages_height = render_chat_messages(canvas, &selected_contact.history, font, size_info, scroll_state);
	render_title_bar(canvas, selected_contact, font, size_info);
	render_input_bar(canvas, font, size_info, current_input);
}

/// Returns the final height of the messages, used for scrolling to bottom.
fn render_chat_messages(canvas: &mut Canvas<Window>, history: &Vec<Message>, font: &Font, size_info: &SizeInfo, scroll_state: i32) {
	let mut h = (size_info.chat_title.1 + CHAT_MESSAGE_PADDING) as i32 - scroll_state;
	for message in history {
		h = render_message_bubble(canvas, &message, font, size_info, h);
	}
}

/// Returns the height of the messages after adding the new one to them.
fn render_message_bubble(
	canvas: &mut Canvas<Window>, 
	message: &Message, 
	font: &Font, 
	size_info: &SizeInfo, 
	h: i32
) -> i32 {
	let partial = font.render(&message.content);
	let solid = partial.blended_wrapped(CHAT_PANE_TEXT_COLOR, size_info.chat_message_max_width).unwrap();
	let tc = canvas.texture_creator();
	let texture = solid.as_texture(&tc).unwrap();
	let text_query = texture.query();

	let msg_color = if message.is_own { CHAT_MESSAGE_OWN_COLOR } else { CHAT_MESSAGE_COLOR };
	canvas.set_draw_color(msg_color);

	let (rect_w, rect_h) = (text_query.width + CHAT_MESSAGE_PADDING * 2, text_query.height + CHAT_MESSAGE_PADDING * 2);

	let (canvas_width, _) = canvas.output_size().expect("could not get output size of canvas when trying to render message bubble");
	let x_pos = if message.is_own { (canvas_width - (rect_w + CHAT_MESSAGE_PADDING)) as i32 } else { (size_info.contact_pane.0 + CHAT_MESSAGE_PADDING) as i32 };

	let (text_x, text_y) = (x_pos + CHAT_MESSAGE_PADDING as i32, h + CHAT_MESSAGE_PADDING as i32);
	let (rect_x, rect_y) = (x_pos, h as i32);

	canvas.fill_rect(Rect::new(rect_x, rect_y, rect_w, rect_h)).expect("could not fill rect for message bubble");
	canvas.copy(&texture, None, Rect::new(text_x, text_y, text_query.width, text_query.height)).unwrap();
	h + (rect_h + CHAT_MESSAGE_PADDING) as i32
}

fn render_title_bar(canvas: &mut Canvas<Window>, selected_contact: &Contact, font: &Font, size_info: &SizeInfo) {
	let x_pos = size_info.contact_pane.0 as i32;
	let (width, height) = size_info.chat_title;
	canvas.set_draw_color(CHAT_TITLE_COLOR);
	canvas.fill_rect(Rect::new(x_pos, 0, width, height)).expect("could not fill rect for chat title bar");

	let partial = font.render(selected_contact.name.as_str());
	let solid = partial.solid(CHAT_PANE_TEXT_COLOR).unwrap();
	let tc = canvas.texture_creator();
	let texture = solid.as_texture(&tc).unwrap();
	let text_query = texture.query();
	let (text_x, text_y) = (x_pos + ((width - text_query.width) / 2) as i32, ((height - text_query.height) / 2) as i32);

	canvas.copy(&texture, None, Rect::new(text_x, text_y, text_query.width, text_query.height)).unwrap();
}

fn render_input_bar(
	canvas: &mut Canvas<Window>, 
	font: &Font, 
	size_info: &SizeInfo, 
	current_input: &str,
) {
	let (width, height) = size_info.chat_input;
	let x_pos = size_info.contact_pane.0 as i32;
	let y_pos = (size_info.contact_pane.1 - height) as i32;
	canvas.set_draw_color(CHAT_TITLE_COLOR);
	canvas.fill_rect(Rect::new(x_pos, y_pos, width, height)).expect("could not fill rect for input bar");

	let input_with_cursor_after = current_input.to_owned() + "|";
	let partial = font.render(&input_with_cursor_after);
	let solid = partial.solid(CHAT_PANE_TEXT_COLOR).unwrap();
	let tc = canvas.texture_creator();
	let texture = solid.as_texture(&tc).unwrap();
	let text_query = texture.query();
	let (text_x, text_y) = (x_pos + 20, ((2 * y_pos + height as i32) / 2 - (text_query.height / 2) as i32));

	let rect = Rect::new(text_x, text_y, text_query.width, text_query.height);

	canvas.copy(&texture, None, rect).unwrap();
}

/// Draw a welcome screen, when no contact is selected.
fn render_welcome_screen(canvas: &mut Canvas<Window>, font: &Font, size_info: &SizeInfo) {
	let x_pos = size_info.contact_pane.0 as i32;
	let (pane_width, pane_height) = size_info.chat_pane;
	canvas.set_draw_color(CHAT_PANE_COLOR);
	canvas.fill_rect(Rect::new(x_pos, 0, pane_width, pane_height)).expect("could not fill rect for welcome screen");

	let partial = font.render("Welcome to `chitchat-rs`!");
	let solid = partial.solid(CHAT_PANE_TEXT_COLOR).unwrap();
	let tc = canvas.texture_creator();
	let texture = solid.as_texture(&tc).unwrap();
	let text_query = texture.query();
	let (text_x, text_y) = (x_pos + ((pane_width - text_query.width) / 2) as i32, ((pane_height - text_query.height) / 2) as i32);

	canvas.copy(&texture, None, Rect::new(text_x, text_y, text_query.width, text_query.height)).unwrap();
}

fn get_messages_height(messages: &Vec<Message>, size_info: &SizeInfo, font: &Font, hidpi_scaling: i32) -> i32 {
	let one_message_height_overhead = 4 * CHAT_MESSAGE_PADDING as i32;
	let mut total_height = messages.len() as i32 * one_message_height_overhead;

	for message in messages {
		let (curr_width, curr_height) = font.size_of(&message.content).expect("could not get size of text in get_messages_height");
		let height_of_message = (curr_height * curr_width / size_info.chat_message_max_width) as i32;
		total_height += height_of_message;
	}

	println!("total height of {} messages: {}", messages.len(), total_height);
	total_height / hidpi_scaling
}
