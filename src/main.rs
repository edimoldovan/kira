// src/main.rs
mod ui;
mod email;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

fn main() {
	let app = Application::builder()
		.application_id("com.kira.email")
		.build();

	app.connect_activate(|app| {
		let window = ApplicationWindow::builder()
			.application(app)
			.title("Kira")
			.default_width(1200)
			.default_height(800)
			.build();

		let content = ui::window::build();
		window.set_child(Some(&content));
		window.present();
	});

	app.run();
}