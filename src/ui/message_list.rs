use gtk4::prelude::*;
use gtk4::{Box, Label, ScrolledWindow, Orientation};
use crate::email::account::Message;

pub fn build() -> (ScrolledWindow, Box) {
	let scrolled = ScrolledWindow::new();
	scrolled.set_width_request(400);

	let list_box = Box::new(Orientation::Vertical, 2);
	list_box.set_margin_start(8);
	list_box.set_margin_end(8);
	list_box.set_margin_top(8);

	scrolled.set_child(Some(&list_box));
	(scrolled, list_box.clone())
}

pub fn update_messages(list_box: &Box, messages: &[Message]) {
	// Clear existing messages
	while let Some(child) = list_box.first_child() {
		list_box.remove(&child);
	}

	// Add new messages
	for msg in messages {
		let msg_box = Box::new(Orientation::Vertical, 4);
		msg_box.set_margin_top(8);
		msg_box.set_margin_bottom(8);

		let from_label = Label::new(Some(&msg.from));
		from_label.set_xalign(0.0);
		if msg.unread {
			from_label.add_css_class("bold");
		}
		msg_box.append(&from_label);

		let subject_label = Label::new(Some(&msg.subject));
		subject_label.set_xalign(0.0);
		msg_box.append(&subject_label);

		let preview_label = Label::new(Some(&msg.preview));
		preview_label.set_xalign(0.0);
		preview_label.add_css_class("dim-label");
		msg_box.append(&preview_label);

		list_box.append(&msg_box);
	}
}