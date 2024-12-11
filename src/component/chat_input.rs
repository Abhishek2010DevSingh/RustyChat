use futures::StreamExt;
use leptos::logging;
use leptos::{ev::SubmitEvent, html::Input, prelude::*, task::spawn_local};

use crate::{
    api,
    model::{Conversation, Message},
};

#[component]
pub fn ChatInput() -> impl IntoView {
    let conversation =
        use_context::<RwSignal<Conversation>>().expect("`Conversation` as global state not found.");
    let input_ref: NodeRef<Input> = NodeRef::new();
    let is_submitting = RwSignal::new(false);

    let handle_response = move |prompt: String| {
        spawn_local(async move {
            match api::streaming_response(prompt.clone()).await {
                Ok(response_stream) => {
                    let mut response_stream = response_stream.into_inner();
                    while let Some(Ok(chunk)) = response_stream.next().await {
                        conversation.update(|conv| {
                            if let Some(last_message) = conv.messages.last_mut() {
                                last_message.text.push_str(&chunk);
                            }
                        });
                    }
                }
                Err(err) => {
                    logging::error!("Failed to fetch response: {err}");
                    conversation.update(|conv| {
                        if let Some(last_message) = conv.messages.last_mut() {
                            last_message.text =
                                "Error fetching response. Please try again.".to_string();
                        }
                    });
                }
            }
            is_submitting.set(false);
        });
    };

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if is_submitting.get() {
            return;
        }

        let input = match input_ref.get() {
            Some(input) => input,
            None => {
                logging::warn!("<input> element is not mounted");
                return;
            }
        };

        let prompt = input.value().trim().to_string();
        if prompt.is_empty() {
            logging::warn!("Prompt is empty");
            return;
        }

        conversation.update(|conv| {
            conv.messages.push(Message {
                user: true,
                text: prompt.clone(),
            });
            conv.messages.push(Message::default());
        });

        is_submitting.set(true);
        input.set_value("");
        handle_response(prompt);
    };

    view! {
        <div class="flex items-center justify-center p-4 bg-gray-100 dark:bg-gray-900">
            <form on:submit=on_submit class="flex w-full max-w-2xl space-x-4">
                <input
                    type="text"
                    node_ref=input_ref
                    placeholder="Type your message..."
                    class="flex-1 px-4 py-2 text-gray-900 border border-gray-300 rounded-lg shadow-sm focus:ring-2 focus:ring-blue-500 focus:outline-none dark:bg-gray-800 dark:text-gray-100 dark:border-gray-700"
                    aria-disabled=is_submitting.get().to_string()
                    disabled=is_submitting.get()
                />
                <button
                    type="submit"
                    disabled=is_submitting.get()
                    class="px-6 py-2 text-white bg-blue-600 rounded-lg shadow-sm hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 focus:outline-none disabled:opacity-50 disabled:cursor-not-allowed"
                    aria-disabled=is_submitting.get().to_string()
                >
                    "Send"
                </button>
            </form>
        </div>
    }
}
