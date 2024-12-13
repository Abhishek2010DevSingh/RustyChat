use leptos::prelude::*;
use server_fn::codec::{StreamingText, TextStream};

#[server( output = StreamingText)]
pub async fn streaming_response(prompt: String) -> Result<TextStream, ServerFnError> {
    use crate::state::AppState;
    use axum::Extension;
    use futures::StreamExt;
    use leptos_axum::extract;
    use ollama_rs::generation::chat::{request::ChatMessageRequest, ChatMessage};

    let Extension(mut state) = extract::<Extension<AppState>>().await?;
    state.history.push(ChatMessage::user(prompt));

    let mut stream_response = state
        .ollama
        .send_chat_messages_stream(ChatMessageRequest::new(
            "llama3.2:1b".into(),
            state.history.clone(),
        ))
        .await
        .map_err(ServerFnError::new)?;

    let (tx, rx) = futures::channel::mpsc::unbounded();
    tokio::spawn(async move {
        let mut response = String::new();
        while let Some(Ok(result)) = stream_response.next().await {
            if let Some(assistant_message) = result.message {
                response.push_str(&assistant_message.content);
                if let Err(err) = tx.unbounded_send(Ok(assistant_message.content)) {
                    tracing::error!("Failed to send message: {}", err);
                    break;
                }
            }
        }
        state.history.push(ChatMessage::assistant(response));
    });

    Ok(TextStream::new(rx))
}
