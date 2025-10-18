use gtk4::prelude::*;
use gtk4::{Box, Orientation, Paned};

pub fn build() -> Box {
	let main_box = Box::new(Orientation::Horizontal, 0);
	
	let sidebar = super::sidebar::build();
	main_box.append(&sidebar);
	
	let paned = Paned::new(Orientation::Horizontal);
	
	let message_list = super::message_list::build();
	paned.set_start_child(Some(&message_list));
	
	let reader = super::reader::build();
	paned.set_end_child(Some(&reader));
	paned.set_position(400);
	
	main_box.append(&paned);
	
	main_box
}