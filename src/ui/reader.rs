use gtk4::prelude::*;
use gtk4::{Box, Button, Label, ScrolledWindow, Orientation};
use crate::email::account::Message;

pub fn build() -> (ScrolledWindow, Box) {
	let scrolled = ScrolledWindow::new();
	scrolled.set_hexpand(true);

	let reader_box = Box::new(Orientation::Vertical, 16);
	reader_box.set_margin_start(16);
	reader_box.set_margin_end(16);
	reader_box.set_margin_top(16);

	scrolled.set_child(Some(&reader_box));
	(scrolled, reader_box.clone())
}

pub fn update_message(reader_box: &Box, message: &Message) {
	// Clear existing content
	while let Some(child) = reader_box.first_child() {
		reader_box.remove(&child);
	}

	// Toolbar with action buttons
	let toolbar = Box::new(Orientation::Horizontal, 8);
	let reply_btn = Button::with_label("Reply");
	let reply_all_btn = Button::with_label("Reply all");
	let forward_btn = Button::with_label("Forward");
	let delete_btn = Button::with_label("Delete");
	let spam_btn = Button::with_label("Mark as spam");
	toolbar.append(&reply_btn);
	toolbar.append(&reply_all_btn);
	toolbar.append(&forward_btn);
	toolbar.append(&delete_btn);
	toolbar.append(&spam_btn);
	reader_box.append(&toolbar);

	// Add message content
	let from_label = Label::new(Some(&format!("From: {}", message.from)));
	from_label.set_xalign(0.0);
	reader_box.append(&from_label);

	let subject_label = Label::new(Some(&format!("Subject: {}", message.subject)));
	subject_label.set_xalign(0.0);
	reader_box.append(&subject_label);

	let body_label = Label::new(Some(&message.body));
	body_label.set_xalign(0.0);
	body_label.set_wrap(true);
	reader_box.append(&body_label);
}