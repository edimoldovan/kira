pub struct Account {
	pub name: String,
	#[allow(dead_code)]
	pub email: String,
	pub unread: u32,
	#[allow(dead_code)]
	pub folders: Vec<String>,
	pub messages: Vec<Message>,
}

pub struct Message {
	pub from: String,
	pub subject: String,
	pub preview: String,
	pub body: String,
	#[allow(dead_code)]
	pub unread: bool,
}