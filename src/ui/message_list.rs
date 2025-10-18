use gtk4::prelude::*;
use gtk4::{Box, Button, Label, ScrolledWindow, Orientation};
use crate::email::account::Message;
use std::rc::Rc;
use std::cell::RefCell;

pub fn build() -> (ScrolledWindow, Box) {
	let scrolled = ScrolledWindow::new();
	scrolled.set_width_request(400);

	let list_box = Box::new(Orientation::Vertical, 8);
	list_box.set_margin_start(8);
	list_box.set_margin_end(8);
	list_box.set_margin_top(8);

	scrolled.set_child(Some(&list_box));
	(scrolled, list_box.clone())
}

pub fn update_messages<F>(list_box: &Box, messages: &[Message], on_message_click: F)
where
	F: Fn(usize) + 'static + Clone,
{
	// Clear existing messages
	while let Some(child) = list_box.first_child() {
		list_box.remove(&child);
	}

	// Toolbar with plus button
	let toolbar = Box::new(Orientation::Horizontal, 0);
	let plus_btn = Button::with_label("+");
	toolbar.append(&plus_btn);
	list_box.append(&toolbar);

	let message_buttons: Rc<RefCell<Vec<Button>>> = Rc::new(RefCell::new(Vec::new()));

	// Add new messages
	for (index, msg) in messages.iter().enumerate() {
		let msg_btn = Button::new();

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

		msg_btn.set_child(Some(&msg_box));

		let callback = on_message_click.clone();
		let buttons_clone = Rc::clone(&message_buttons);
		msg_btn.connect_clicked(move |btn| {
			// Remove selected state from all message buttons
			for b in buttons_clone.borrow().iter() {
				b.remove_css_class("suggested-action");
			}
			// Add selected state to clicked button
			btn.add_css_class("suggested-action");
			callback(index);
		});

		// Select first message by default
		if index == 0 {
			msg_btn.add_css_class("suggested-action");
		}

		message_buttons.borrow_mut().push(msg_btn.clone());
		list_box.append(&msg_btn);
	}
}