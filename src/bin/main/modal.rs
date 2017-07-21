use std::string::String;

pub struct Modal {
	pub title: String,
	pub message: String
}

impl Modal {
	pub fn new(title: String, message: String) -> Modal {
		Modal {
			title: title,
			message: message
		}
	}
}
