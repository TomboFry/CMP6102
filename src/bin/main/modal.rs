use std::string::String;

pub struct Modal {
	pub title: String,
	pub message: String,
	pub button_a_label: String,
	pub button_b_label: String
}

impl Modal {
	pub fn new(title: String, message: String, btn_a_label: Option<String>, btn_b_label: Option<String>) -> Modal {
		let mut button_a_label = "Okay".to_string();
		let mut button_b_label = "Close".to_string();

		if let Some(lbl_a) = btn_a_label { button_a_label = lbl_a; }
		if let Some(lbl_b) = btn_b_label { button_b_label = lbl_b; }

		Modal {
			title: title,
			message: message,
			button_a_label: button_a_label,
			button_b_label: button_b_label
		}
	}
}
