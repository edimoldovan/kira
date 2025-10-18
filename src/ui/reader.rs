use gtk4::prelude::*;
use gtk4::{Box, Label, ScrolledWindow, Orientation};

pub fn build() -> ScrolledWindow {
	let scrolled = ScrolledWindow::new();
	
	let reader_box = Box::new(Orientation::Vertical, 16);
	reader_box.set_margin_start(16);
	reader_box.set_margin_end(16);
	reader_box.set_margin_top(16);
	
	let from_label = Label::new(Some("From: alice@example.com"));
	from_label.set_xalign(0.0);
	reader_box.append(&from_label);
	
	let subject_label = Label::new(Some("Subject: Project Update"));
	subject_label.set_xalign(0.0);
	reader_box.append(&subject_label);
	
	let body_label = Label::new(Some("Here's the latest update on the project. Everything is progressing well and we're on schedule for the next milestone."));
	body_label.set_xalign(0.0);
	body_label.set_wrap(true);
	reader_box.append(&body_label);
	
	scrolled.set_child(Some(&reader_box));
	scrolled
}