mod email;

use email::account::Account;
use iced::widget::{button, column, container, row, scrollable, text};
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
    InboxClicked(usize),
    MessageClicked(usize),
    AddAccount,
    AddMessage,
    ToggleAccountExpansion(usize),
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
            Message::InboxClicked(index) => {
                self.current_account = index;
                self.current_message = 0;
            }
            Message::MessageClicked(index) => {
                self.current_message = index;
            }
            Message::AddAccount => {}
            Message::AddMessage => {}
            Message::ToggleAccountExpansion(index) => {
                if let Some(expanded) = self.expanded_accounts.get_mut(index) {
                    *expanded = !*expanded;
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let sidebar = self.view_sidebar();
        let message_list = self.view_message_list();
        let reader = self.view_reader();

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
    fn view_sidebar(&self) -> Element<'_, Message> {
        let toolbar = row![button(text("+")).on_press(Message::AddAccount)]
            .padding(0)
            .spacing(0);

        let mut sidebar_content = column![toolbar].spacing(8).padding(12);

        // Section 1: Inbox buttons
        for (index, account) in self.accounts.iter().enumerate() {
            let is_selected = index == self.current_account;
            let mut btn = button(text(format!("{} ({})", account.name, account.unread)))
                .width(Length::Fill);

            if is_selected {
                btn = btn.style(button::primary);
            }

            btn = btn.on_press(Message::InboxClicked(index));

            sidebar_content = sidebar_content.push(btn);
        }

        // Section 2: Account details with folders
        for (index, account) in self.accounts.iter().enumerate() {
            let email_btn = button(text(&account.email))
                .width(Length::Fill)
                .on_press(Message::ToggleAccountExpansion(index));
            sidebar_content = sidebar_content.push(email_btn);

            if *self.expanded_accounts.get(index).unwrap_or(&false) {
                for folder in &account.folders {
                    let folder_btn = button(text(folder))
                        .width(Length::Fill);
                    sidebar_content = sidebar_content.push(folder_btn);
                }
            }
        }

        container(scrollable(sidebar_content))
            .width(250)
            .height(Length::Fill)
            .into()
    }

    fn view_message_list(&self) -> Element<'_, Message> {
        let toolbar = row![button(text("+")).on_press(Message::AddMessage)]
            .padding(0)
            .spacing(0);

        let mut list_content = column![toolbar].spacing(8).padding(12);

        if let Some(account) = self.accounts.get(self.current_account) {
            for (index, msg) in account.messages.iter().enumerate() {
                let is_selected = index == self.current_message;

                let msg_view = column![
                    text(&msg.from).size(14),
                    text(&msg.subject).size(14),
                    text(&msg.preview).size(12),
                ]
                .spacing(4)
                .padding(8);

                let mut btn = button(msg_view).width(Length::Fill);

                if is_selected {
                    btn = btn.style(button::primary);
                }

                btn = btn.on_press(Message::MessageClicked(index));

                list_content = list_content.push(btn);
            }
        }

        container(scrollable(list_content))
            .width(400)
            .height(Length::Fill)
            .into()
    }

    fn view_reader(&self) -> Element<'_, Message> {
        let toolbar = row![
            button(text("Reply")),
            button(text("Reply all")),
            button(text("Forward")),
            button(text("Delete")),
            button(text("Mark as spam")),
        ]
        .spacing(8)
        .padding(0);

        let mut reader_content = column![toolbar].spacing(16).padding(12);

        if let Some(account) = self.accounts.get(self.current_account) {
            if let Some(msg) = account.messages.get(self.current_message) {
                reader_content = reader_content
                    .push(text(format!("From: {}", msg.from)))
                    .push(text(format!("Subject: {}", msg.subject)))
                    .push(text(&msg.body));
            }
        }

        container(scrollable(reader_content))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
