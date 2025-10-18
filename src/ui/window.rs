use gtk4::prelude::*;
use gtk4::{Box, Orientation, Paned};
use std::rc::Rc;
use crate::email::account::{Account, Message};

pub fn build() -> Box {
	let main_box = Box::new(Orientation::Horizontal, 0);

	// Create accounts data
	let accounts = Rc::new(vec![
		Account {
			name: "Work".to_string(),
			email: "work@example.com".to_string(),
			unread: 5,
			folders: vec!["Sent".to_string(), "Drafts".to_string(), "Trash".to_string()],
			messages: vec![
				Message {
					from: "alice@work.com".to_string(),
					subject: "Q4 Budget Review".to_string(),
					preview: "Please review the attached budget proposal...".to_string(),
					unread: true,
				},
				Message {
					from: "bob@work.com".to_string(),
					subject: "Team Meeting Notes".to_string(),
					preview: "Here are the notes from today's standup...".to_string(),
					unread: true,
				},
				Message {
					from: "carol@work.com".to_string(),
					subject: "Project Deadline Extension".to_string(),
					preview: "Good news - we've been granted an extension...".to_string(),
					unread: false,
				},
			],
		},
		Account {
			name: "Personal".to_string(),
			email: "personal@example.com".to_string(),
			unread: 12,
			folders: vec!["Sent".to_string(), "Drafts".to_string(), "Archive".to_string(), "Spam".to_string()],
			messages: vec![
				Message {
					from: "mom@family.com".to_string(),
					subject: "Dinner this Sunday?".to_string(),
					preview: "Hi dear, would you like to come over for dinner...".to_string(),
					unread: true,
				},
				Message {
					from: "netflix@streaming.com".to_string(),
					subject: "New shows you might like".to_string(),
					preview: "Check out these new releases...".to_string(),
					unread: true,
				},
				Message {
					from: "friend@example.com".to_string(),
					subject: "Game night this Friday".to_string(),
					preview: "Hey! Want to join us for board games?".to_string(),
					unread: false,
				},
			],
		},
	]);

	let paned = Paned::new(Orientation::Horizontal);

	let (message_list, list_box) = super::message_list::build();

	// Initialize with first account's messages (Work)
	if !accounts.is_empty() {
		super::message_list::update_messages(&list_box, &accounts[0].messages);
	}

	// Create the callback that will update messages when an inbox is clicked
	let accounts_clone = Rc::clone(&accounts);
	let list_box_clone = list_box.clone();
	let callback = move |account_index: usize| {
		if let Some(account) = accounts_clone.get(account_index) {
			super::message_list::update_messages(&list_box_clone, &account.messages);
		}
	};

	let sidebar = super::sidebar::build(&accounts, callback);
	main_box.append(&sidebar);

	paned.set_start_child(Some(&message_list));

	let reader = super::reader::build();
	paned.set_end_child(Some(&reader));
	paned.set_position(400);

	main_box.append(&paned);

	main_box
}