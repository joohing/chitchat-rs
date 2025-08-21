use crate::chat::*;

#[derive(Clone)]
pub struct Contact {
	pub addr: String,
	pub name: String,
	pub history: Vec<Message>,
}

impl Contact {
	pub fn sample() -> Self {
		let addr = "123.123.123.12".to_string();
		let name = "Sample Guy - OR GIRL!".to_string();
		let history = vec![Message::sample(), Message::sample(), Message::sample()];
		Self { addr, name, history }
	}

	pub fn new(addr: String, name: String, history: Vec<Message>) -> Self {
		Self { addr, name, history }
	}
}
