use gtk4::prelude::*;
use gtk4::{Box, Button, Expander, Orientation};
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
			folders: vec!["Sent".to_string(), "Drafts".to_string(), "Trash".to_string()],
		},
		Account {
			name: "Personal".to_string(),
			email: "personal@example.com".to_string(),
			unread: 12,
			folders: vec!["Sent".to_string(), "Drafts".to_string(), "Archive".to_string(), "Spam".to_string()],
		},
	];

	// Section 1: Inboxes
	let inboxes_box = Box::new(Orientation::Vertical, 4);
	for account in &accounts {
		let inbox_btn = Button::with_label(&format!("{} Inbox ({})", account.name, account.unread));
		inboxes_box.append(&inbox_btn);
	}
	sidebar.append(&inboxes_box);

	// Spacing between sections
	let spacer = Box::new(Orientation::Vertical, 0);
	spacer.set_margin_top(16);
	sidebar.append(&spacer);

	// Section 2: Account details with folders
	let accounts_box = Box::new(Orientation::Vertical, 4);
	for account in &accounts {
		let expander = Expander::new(Some(&format!("{} ({})", account.name, account.email)));

		let folders_box = Box::new(Orientation::Vertical, 2);
		folders_box.set_margin_start(16);
		folders_box.set_margin_top(4);

		for folder in &account.folders {
			let folder_btn = Button::with_label(folder);
			folders_box.append(&folder_btn);
		}

		expander.set_child(Some(&folders_box));
		accounts_box.append(&expander);
	}
	sidebar.append(&accounts_box);

	sidebar
}