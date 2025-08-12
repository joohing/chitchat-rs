// In: some layout and a canvas, out: SizeInfo, result: layout drawn on canvas.
mod render;

// Relates the contacts and the chat to each other: contains info about their current contents.
// A layout is a contacts pane and a chat pane.
mod layout;

// Contacts pane and contacts, as well as a plus button.
mod contacts;

// Chat pane and messages, as well as input bar and title of chat.
mod chat;

fn main() {
	println!("Hello, world!");
}
