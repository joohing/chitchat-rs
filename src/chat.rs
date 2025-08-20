#[derive(Clone)]
pub struct Message {
	content: String,
	is_own: bool,
}

impl Message {
	pub fn sample() -> Self {
		let content = "I am a sample string LOL! Check out this sample stuff LOL!".to_string();
		let is_own = false;
		Self { content, is_own }
	}
}
