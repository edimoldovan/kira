use gtk4::prelude::*;
use gtk4::{Box, Label, Button, Orientation};
use crate::email::account::Account;

pub fn build() -> Box {
	let sidebar = Box::new(Orientation::Vertical, 8);
	sidebar.set_width_request(250);
	sidebar.set_margin_start(8);
	sidebar.set_margin_end(8);
	sidebar.set_margin_top(8);
	sidebar.set_margin_bottom(8);
	
	let accounts = vec![
		Account {
			name: "Work".to_string(),
			email: "work@example.com".to_string(),
			unread: 5,
		},
		Account {
			name: "Personal".to_string(),
			email: "personal@example.com".to_string(),
			unread: 12,
		},
	];
	
	for account in accounts {
		let account_box = Box::new(Orientation::Vertical, 4);
		
		let inbox_btn = Button::with_label(&format!("{} ({})", account.name, account.unread));
		account_box.append(&inbox_btn);
		
		let folders_box = Box::new(Orientation::Vertical, 2);
		folders_box.set_margin_start(16);
		
		for folder in &["Sent", "Drafts", "Trash"] {
			let folder_btn = Button::with_label(folder);
			folders_box.append(&folder_btn);
		}
		
		account_box.append(&folders_box);
		sidebar.append(&account_box);
	}
	
	sidebar
}