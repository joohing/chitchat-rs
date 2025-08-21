use crate::{contacts::*, render::CONTACT_PADDING, SizeInfo};
use sdl2::rect::Point;

const SCROLL_SENSITIVITY: i32 = 5;

enum UIElements {
	Contacts(i32),
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
		let _0 = Contact::new(String::from("123.2.2"), String::from("Mette Frederiksen"), vec![]);
		let _1 = Contact::sample();
		let _2 = Contact::new(String::from("123.123.2"), String::from("Polsegut"), vec![]);
		let contacts = vec![_0, _1, _2.clone()];
		let selected_contact = Some(_2);
		Self {
			contacts,
			selected_contact,
			chat_scroll_state: 0,
			contact_scroll_state: 0,
		}
	}

	pub fn click(&mut self, x: &i32, y: &i32, size_info: &SizeInfo, hidpi_scaling: i32) {
		match where_is_mouse(x, y, size_info, hidpi_scaling) {
			UIElements::Chat => {}
			UIElements::Contacts(i) => {
				if let Some(ref ct) = self.selected_contact {
					let addr = &ct.addr;
					let old = self.contacts.iter_mut().find(|c| &c.addr == addr).expect("currently selected does not exist");
					*old = ct.clone();
				}
				if i < self.contacts.len() as i32 {
					self.selected_contact = self.contacts.get(i as usize).cloned()
				}
			}
		}
	}

	pub fn scroll(&mut self, mouse_x: &i32, mouse_y: &i32, size_info: &SizeInfo, y: &i32, hidpi_scaling: i32) {
		match where_is_mouse(mouse_x, mouse_y, size_info, hidpi_scaling) {
			UIElements::Chat => {
				self.chat_scroll_state += y * SCROLL_SENSITIVITY;
			}
			UIElements::Contacts(_) => {
				self.contact_scroll_state += y * SCROLL_SENSITIVITY;
			}
		}
		self.chat_scroll_state = self.chat_scroll_state.max(0);
		self.contact_scroll_state = self.contact_scroll_state.max(0);
	}
}

fn where_is_mouse(mouse_x: &i32, mouse_y: &i32, size_info: &SizeInfo, hidpi_scaling: i32) -> UIElements {
	match mouse_x.clone() {
		x if x * hidpi_scaling <= size_info.contact_pane.0 as i32 => {
			let contact_height = (size_info.contact.1 + CONTACT_PADDING) as i32;
			let i = mouse_y * hidpi_scaling / contact_height;
			println!("contact selected: {}", i);
			UIElements::Contacts(i)
		}
		_ => UIElements::Chat,
	}
}
