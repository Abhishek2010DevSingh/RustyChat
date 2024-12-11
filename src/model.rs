use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Conversation {
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Message {
    pub user: bool,
    pub text: String,
}
