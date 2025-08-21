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
		Self { contact_pane, contact, plus_button, chat_pane, chat_message_max_width, chat_title, chat_input }
	}

	pub fn update_from_canvas(&mut self, canvas: &Canvas<Window>) {
		let updated                 = Self::from_canvas(canvas);
		self.contact_pane           = updated.contact_pane;
		self.contact                = updated.contact;
		self.plus_button            = updated.plus_button;
		self.chat_pane              = updated.chat_pane;
		self.chat_message_max_width = updated.chat_message_max_width;
		self.chat_title             = updated.chat_title;
		self.chat_input             = updated.chat_input;
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
	render_contact_pane(canvas, &layout.contacts, &layout.selected_contact, size_info, font);
	if let Some(ref contact) = layout.selected_contact {
		if !txt_input.is_active() { txt_input.start(); }
		render_chat_pane(canvas, contact, font, size_info, current_input, txt_input);
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
	font: &Font
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
	canvas.fill_rect(Rect::new(pos.x, pos.y, w, h));

	let partial = font.render(contact.name.as_str());
	let solid = partial.solid(CHAT_PANE_TEXT_COLOR).unwrap();
	let tc = canvas.texture_creator();
	let texture = solid.as_texture(&tc).unwrap();
	let text_query = texture.query();

	let (name_x, name_y) = (pos.x + CONTACT_PADDING as i32, pos.y + CONTACT_PADDING as i32);

	canvas.copy(&texture, None, Rect::new(name_x, name_y, text_query.width, text_query.height)).unwrap();
}

/// Use the selected contact to draw a chat pane.
fn render_chat_pane(
	canvas: &mut Canvas<Window>,
	selected_contact: &Contact,
	font: &Font,
	size_info: &SizeInfo,
	current_input: &str,
	txt_input: &TextInputUtil,
) {
	let x_pos = size_info.contact_pane.0 as i32;
	let (pane_width, pane_height) = size_info.chat_pane;
	canvas.set_draw_color(CHAT_PANE_COLOR);
	canvas.fill_rect(Rect::new(x_pos, 0, pane_width, pane_height));

	render_title_bar(canvas, selected_contact, font, size_info);

	// TODO how to scroll?
	render_chat_messages(canvas, &selected_contact.history, font, size_info);

	render_input_bar(canvas, selected_contact, font, size_info, current_input, txt_input);
}

fn render_chat_messages(canvas: &mut Canvas<Window>, history: &Vec<Message>, font: &Font, size_info: &SizeInfo) {
	let mut h = size_info.chat_title.1 + CHAT_MESSAGE_PADDING;
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
	h: u32
) -> u32 {
	let partial = font.render(&message.content);
	let solid = partial.blended_wrapped(CHAT_PANE_TEXT_COLOR, size_info.chat_message_max_width).unwrap();
	let tc = canvas.texture_creator();
	let texture = solid.as_texture(&tc).unwrap();
	let text_query = texture.query();

	let msg_color = if message.is_own { CHAT_MESSAGE_OWN_COLOR } else { CHAT_MESSAGE_COLOR };
	canvas.set_draw_color(msg_color);

	let (canvas_width, canvas_height) = canvas.output_size().expect("could not get output size of canvas when trying to render message bubble");
	let x_pos = if message.is_own { (canvas_width - (text_query.width + CHAT_MESSAGE_PADDING * 2)) as i32 } else { (size_info.contact_pane.0 + CHAT_MESSAGE_PADDING) as i32 };

	let (rect_x, rect_y) = (x_pos, h as i32);
	let (rect_w, rect_h) = (text_query.width + CHAT_MESSAGE_PADDING * 2, text_query.height + CHAT_MESSAGE_PADDING * 2);
	let (text_x, text_y) = (x_pos + CHAT_MESSAGE_PADDING as i32, (h + CHAT_MESSAGE_PADDING) as i32);

	canvas.fill_rect(Rect::new(rect_x, rect_y, rect_w, rect_h));
	canvas.copy(&texture, None, Rect::new(text_x, text_y, text_query.width, text_query.height)).unwrap();
	h + rect_h + CHAT_MESSAGE_PADDING
}

fn render_title_bar(canvas: &mut Canvas<Window>, selected_contact: &Contact, font: &Font, size_info: &SizeInfo) {
	let x_pos = size_info.contact_pane.0 as i32;
	let (width, height) = size_info.chat_title;
	canvas.set_draw_color(CHAT_TITLE_COLOR);
	canvas.fill_rect(Rect::new(x_pos, 0, width, height));

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
	selected_contact: &Contact, 
	font: &Font, 
	size_info: &SizeInfo, 
	current_input: &str,
	txt_input: &TextInputUtil
) {
	let (width, height) = size_info.chat_input;
	let x_pos = size_info.contact_pane.0 as i32;
	let y_pos = (size_info.contact_pane.1 - height) as i32;
	canvas.set_draw_color(CHAT_TITLE_COLOR);
	canvas.fill_rect(Rect::new(x_pos, y_pos, width, height));

	if current_input.is_empty() { return; }
	let partial = font.render(current_input);
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
	canvas.fill_rect(Rect::new(x_pos, 0, pane_width, pane_height));

	let partial = font.render("Welcome to `chitchat-rs`!");
	let solid = partial.solid(CHAT_PANE_TEXT_COLOR).unwrap();
	let tc = canvas.texture_creator();
	let texture = solid.as_texture(&tc).unwrap();
	let text_query = texture.query();
	let (text_x, text_y) = (x_pos + ((pane_width - text_query.width) / 2) as i32, ((pane_height - text_query.height) / 2) as i32);

	canvas.copy(&texture, None, Rect::new(text_x, text_y, text_query.width, text_query.height)).unwrap();
}
