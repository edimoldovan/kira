use gtk4::prelude::*;
use gtk4::{Box, Button, Expander, Orientation};
use crate::email::account::Account;
use std::rc::Rc;
use std::cell::RefCell;

pub fn build<F>(accounts: &[Account], on_inbox_click: F) -> Box
where
	F: Fn(usize) + 'static + Clone,
{
	let sidebar = Box::new(Orientation::Vertical, 8);
	sidebar.set_width_request(250);
	sidebar.set_margin_start(8);
	sidebar.set_margin_end(8);
	sidebar.set_margin_top(8);
	sidebar.set_margin_bottom(8);

	// Section 1: Inboxes
	let inboxes_box = Box::new(Orientation::Vertical, 4);
	let inbox_buttons: Rc<RefCell<Vec<Button>>> = Rc::new(RefCell::new(Vec::new()));

	for (index, account) in accounts.iter().enumerate() {
		let inbox_btn = Button::with_label(&format!("{} Inbox ({})", account.name, account.unread));

		// Select first inbox by default
		if index == 0 {
			inbox_btn.add_css_class("suggested-action");
		}

		let callback = on_inbox_click.clone();
		let buttons_clone = Rc::clone(&inbox_buttons);
		inbox_btn.connect_clicked(move |btn| {
			// Remove selected state from all buttons
			for b in buttons_clone.borrow().iter() {
				b.remove_css_class("suggested-action");
			}
			// Add selected state to clicked button
			btn.add_css_class("suggested-action");
			callback(index);
		});

		inbox_buttons.borrow_mut().push(inbox_btn.clone());
		inboxes_box.append(&inbox_btn);
	}
	sidebar.append(&inboxes_box);

	// Spacing between sections
	let spacer = Box::new(Orientation::Vertical, 0);
	spacer.set_margin_top(16);
	sidebar.append(&spacer);

	// Section 2: Account details with folders
	let accounts_box = Box::new(Orientation::Vertical, 4);
	for account in accounts {
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