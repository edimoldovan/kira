use gtk4::prelude::*;
use gtk4::{Box, Orientation, Paned};
use std::rc::Rc;
use std::cell::RefCell;
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
					body: "Please review the attached budget proposal for Q4. We need to finalize the numbers by end of week. The proposal includes increased spending on infrastructure and a new initiative for team development.".to_string(),
					unread: true,
				},
				Message {
					from: "bob@work.com".to_string(),
					subject: "Team Meeting Notes".to_string(),
					preview: "Here are the notes from today's standup...".to_string(),
					body: "Here are the notes from today's standup meeting:\n\n- Sprint is on track\n- Two tickets need review\n- Planning session scheduled for Thursday\n- Remember to update your time logs".to_string(),
					unread: true,
				},
				Message {
					from: "carol@work.com".to_string(),
					subject: "Project Deadline Extension".to_string(),
					preview: "Good news - we've been granted an extension...".to_string(),
					body: "Good news - we've been granted an extension for the project deadline. The new due date is next month. This gives us more time to ensure quality and proper testing.".to_string(),
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
					body: "Hi dear, would you like to come over for dinner this Sunday? I'm making your favorite lasagna! Let me know if you can make it. Love, Mom".to_string(),
					unread: true,
				},
				Message {
					from: "netflix@streaming.com".to_string(),
					subject: "New shows you might like".to_string(),
					preview: "Check out these new releases...".to_string(),
					body: "Check out these new releases based on your viewing history:\n\n- Mystery at Midnight (New Series)\n- The Documentary Series Everyone's Talking About\n- Comedy Special: Stand Up Night\n\nHappy watching!".to_string(),
					unread: true,
				},
				Message {
					from: "friend@example.com".to_string(),
					subject: "Game night this Friday".to_string(),
					preview: "Hey! Want to join us for board games?".to_string(),
					body: "Hey! Want to join us for board games this Friday at 7pm? We're planning to play Catan and maybe some Cards Against Humanity. Bring snacks if you can!".to_string(),
					unread: false,
				},
			],
		},
	]);

	let paned = Paned::new(Orientation::Horizontal);

	let (message_list, list_box) = super::message_list::build();
	let (reader, reader_box) = super::reader::build();

	// Track current account index
	let current_account_index = Rc::new(RefCell::new(0));

	// Initialize reader with first message
	if !accounts.is_empty() && !accounts[0].messages.is_empty() {
		super::reader::update_message(&reader_box, &accounts[0].messages[0]);
	}

	// Create the callback for when a message is clicked
	let accounts_for_msg_click = Rc::clone(&accounts);
	let reader_box_for_msg_click = reader_box.clone();
	let current_account_for_msg_click = Rc::clone(&current_account_index);
	let message_click_callback = move |message_index: usize| {
		let account_idx = *current_account_for_msg_click.borrow();
		if let Some(account) = accounts_for_msg_click.get(account_idx) {
			if let Some(message) = account.messages.get(message_index) {
				super::reader::update_message(&reader_box_for_msg_click, message);
			}
		}
	};

	// Initialize with first account's messages (Work)
	if !accounts.is_empty() {
		super::message_list::update_messages(&list_box, &accounts[0].messages, message_click_callback.clone());
	}

	// Create the callback that will update messages when an inbox is clicked
	let accounts_for_inbox = Rc::clone(&accounts);
	let list_box_for_inbox = list_box.clone();
	let reader_box_for_inbox = reader_box.clone();
	let current_account_for_inbox = Rc::clone(&current_account_index);
	let inbox_callback = move |account_index: usize| {
		*current_account_for_inbox.borrow_mut() = account_index;
		if let Some(account) = accounts_for_inbox.get(account_index) {
			let msg_callback = message_click_callback.clone();
			super::message_list::update_messages(&list_box_for_inbox, &account.messages, msg_callback);

			// Update reader with first message of the selected account
			if let Some(first_msg) = account.messages.first() {
				super::reader::update_message(&reader_box_for_inbox, first_msg);
			}
		}
	};

	let sidebar = super::sidebar::build(&accounts, inbox_callback);
	main_box.append(&sidebar);

	paned.set_start_child(Some(&message_list));
	paned.set_end_child(Some(&reader));
	paned.set_position(400);

	main_box.append(&paned);

	main_box
}