use ollama_rs::{generation::chat::ChatMessage, Ollama};

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub ollama: Ollama,
    pub history: Vec<ChatMessage>,
}
