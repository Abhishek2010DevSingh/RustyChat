use leptos::prelude::*;
use server_fn::codec::{StreamingText, TextStream as LeptosTextStream};

#[server(output = StreamingText)]
pub async fn streaming_response(prompt: String) -> Result<LeptosTextStream, ServerFnError> {
    use axum::extract::Extension;
    use kalosm::language::*;
    use leptos_axum::extract;
    use std::sync::Arc;

    let Extension(model) = extract::<Extension<Arc<Llama>>>().await?;

    let markers = model.chat_markers().unwrap();
    let message = markers.system_prompt_marker.to_string()
        + "You are a helpful assistant who responds to user input with concise, helpful answers."
        + markers.end_system_prompt_marker
        + markers.user_marker
        + &prompt
        + markers.end_user_marker
        + markers.assistant_marker;

    let stream = model
        .stream_text(&message)
        .with_stop_on(markers.end_system_prompt_marker.to_string())
        .await
        .unwrap();

    Ok(LeptosTextStream::new(stream.map(Ok)))
}
