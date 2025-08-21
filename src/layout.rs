use crate::{contacts::*, SizeInfo};
use sdl2::rect::Point;

const SCROLL_SENSITIVITY: i32 = 5;

enum Scrollables {
	Contacts,
	Chat,
}

pub struct Layout {
	pub contacts: Vec<Contact>,
	pub selected_contact: Option<Contact>,
	pub chat_scroll_state: i32,
	pub contact_scroll_state: i32,
}

impl Layout {
	pub fn sample() -> Self {
		let sel_contact = Contact::new(String::from("123.123.2"), String::from("Polsegut"), vec![]);
		let contacts = vec![Contact::sample(), Contact::sample(), sel_contact.clone()];
		let selected_contact = Some(sel_contact);
		Self {
			contacts,
			selected_contact,
			chat_scroll_state: 0,
			contact_scroll_state: 0,
		}
	}

	pub fn scroll(&mut self, mouse_x: &i32, mouse_y: &i32, size_info: &SizeInfo, y: &i32, hidpi_scaling: i32) {
		
		match where_is_mouse(mouse_x, mouse_y, size_info, hidpi_scaling) {
			Scrollables::Chat => {
				self.chat_scroll_state += y * SCROLL_SENSITIVITY;
			}
			Scrollables::Contacts => {
				self.contact_scroll_state += y * SCROLL_SENSITIVITY;
			}
		}
		self.chat_scroll_state = self.chat_scroll_state.max(0);
		self.contact_scroll_state = self.contact_scroll_state.max(0);
	}
}

fn where_is_mouse(mouse_x: &i32, mouse_y: &i32, size_info: &SizeInfo, hidpi_scaling: i32) -> Scrollables {
	match mouse_x.clone() {
		x if x * hidpi_scaling <= size_info.contact_pane.0 as i32 => Scrollables::Contacts,
		_ => Scrollables::Chat,
	}
}
