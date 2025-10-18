pub struct Account {
	pub name: String,
	pub email: String,
	pub unread: u32,
	pub folders: Vec<String>,
	pub messages: Vec<Message>,
}

pub struct Message {
	pub from: String,
	pub subject: String,
	pub preview: String,
	pub body: String,
	pub unread: bool,
}