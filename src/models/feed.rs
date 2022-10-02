pub struct Feed {
    messages: Vec<Message>,
}

impl Feed {
    pub fn add_message(msg: Message) -> Result<Message> {
        self.messages.push(msg);
        self.messages.sort_by_key(|msg| msg.created_at);
        Ok(msg)
    }

    pub fn messages_iter(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
    }
}
