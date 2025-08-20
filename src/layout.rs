use crate::contacts::*;

pub struct Layout {
	pub contacts: Vec<Contact>,
	pub selected_contact: Option<Contact>,
}

impl Layout {
	pub fn sample() -> Self {
		let sel_contact = Contact::new(String::from("123.123.2"), String::from("Polsegut"), vec![]);
		let contacts = vec![Contact::sample(), Contact::sample(), sel_contact.clone()];
		let selected_contact = Some(sel_contact);
		Self {
			contacts,
			selected_contact,
		}
	}
}
