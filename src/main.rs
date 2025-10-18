mod email;
mod ui;

use email::account::Account;
use iced::widget::{container, row};
use iced::{Element, Length, Task, Theme};

fn main() -> iced::Result {
    iced::application("Kira", Kira::update, Kira::view)
        .theme(Kira::theme)
        .window_size((1200.0, 800.0))
        .run_with(Kira::new)
}

struct Kira {
    accounts: Vec<Account>,
    current_account: usize,
    current_message: usize,
    expanded_accounts: Vec<bool>,
}

#[derive(Debug, Clone)]
enum Message {
    Sidebar(ui::sidebar::Message),
    MessageList(ui::message_list::Message),
    Reader(ui::reader::Message),
}

impl Kira {
    fn new() -> (Self, Task<Message>) {
        let accounts = vec![
            Account {
                name: "Work".to_string(),
                email: "work@example.com".to_string(),
                unread: 5,
                folders: vec!["Sent".to_string(), "Drafts".to_string(), "Trash".to_string()],
                messages: vec![
                    email::account::Message {
                        from: "alice@work.com".to_string(),
                        subject: "Q4 Budget Review".to_string(),
                        preview: "Please review the attached budget proposal...".to_string(),
                        body: "Please review the attached budget proposal for Q4. We need to finalize the numbers by end of week. The proposal includes increased spending on infrastructure and a new initiative for team development.".to_string(),
                        unread: true,
                    },
                    email::account::Message {
                        from: "bob@work.com".to_string(),
                        subject: "Team Meeting Notes".to_string(),
                        preview: "Here are the notes from today's standup...".to_string(),
                        body: "Here are the notes from today's standup meeting:\n\n- Sprint is on track\n- Two tickets need review\n- Planning session scheduled for Thursday\n- Remember to update your time logs".to_string(),
                        unread: true,
                    },
                    email::account::Message {
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
                    email::account::Message {
                        from: "mom@family.com".to_string(),
                        subject: "Dinner this Sunday?".to_string(),
                        preview: "Hi dear, would you like to come over for dinner...".to_string(),
                        body: "Hi dear, would you like to come over for dinner this Sunday? I'm making your favorite lasagna! Let me know if you can make it. Love, Mom".to_string(),
                        unread: true,
                    },
                    email::account::Message {
                        from: "netflix@streaming.com".to_string(),
                        subject: "New shows you might like".to_string(),
                        preview: "Check out these new releases...".to_string(),
                        body: "Check out these new releases based on your viewing history:\n\n- Mystery at Midnight (New Series)\n- The Documentary Series Everyone's Talking About\n- Comedy Special: Stand Up Night\n\nHappy watching!".to_string(),
                        unread: true,
                    },
                    email::account::Message {
                        from: "friend@example.com".to_string(),
                        subject: "Game night this Friday".to_string(),
                        preview: "Hey! Want to join us for board games?".to_string(),
                        body: "Hey! Want to join us for board games this Friday at 7pm? We're planning to play Catan and maybe some Cards Against Humanity. Bring snacks if you can!".to_string(),
                        unread: false,
                    },
                ],
            },
        ];

        let expanded_accounts = vec![false; accounts.len()];

        (
            Self {
                accounts,
                current_account: 0,
                current_message: 0,
                expanded_accounts,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Sidebar(sidebar_msg) => match sidebar_msg {
                ui::sidebar::Message::InboxClicked(index) => {
                    self.current_account = index;
                    self.current_message = 0;
                }
                ui::sidebar::Message::AddAccount => {}
                ui::sidebar::Message::ToggleAccountExpansion(index) => {
                    if let Some(expanded) = self.expanded_accounts.get_mut(index) {
                        *expanded = !*expanded;
                    }
                }
            },
            Message::MessageList(msg_list_msg) => match msg_list_msg {
                ui::message_list::Message::MessageClicked(index) => {
                    self.current_message = index;
                }
                ui::message_list::Message::AddMessage => {}
            },
            Message::Reader(_reader_msg) => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let sidebar = ui::sidebar::view(
            &self.accounts,
            self.current_account,
            &self.expanded_accounts,
        )
        .map(Message::Sidebar);

        let messages = self
            .accounts
            .get(self.current_account)
            .map(|acc| acc.messages.as_slice())
            .unwrap_or(&[]);

        let message_list =
            ui::message_list::view(messages, self.current_message).map(Message::MessageList);

        let current_msg = self
            .accounts
            .get(self.current_account)
            .and_then(|acc| acc.messages.get(self.current_message));

        let reader = ui::reader::view(current_msg).map(Message::Reader);

        let content = row![sidebar, message_list, reader]
            .spacing(0)
            .width(Length::Fill)
            .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
