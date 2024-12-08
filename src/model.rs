use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatRequest {
    pub prompt: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Conversation {
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub user: bool,
    pub text: String,
}
