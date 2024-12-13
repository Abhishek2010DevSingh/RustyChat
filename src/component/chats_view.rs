use crate::model::Conversation;
use leptos::html::Div;
use leptos::prelude::*;
use pulldown_cmark::{html, Options, Parser};

#[component]
pub fn ChatsView() -> impl IntoView {
    let conversation =
        use_context::<RwSignal<Conversation>>().expect("`Conversation` as global state not found.");

    let div_node_ref: NodeRef<Div> = NodeRef::new();
    Effect::new(move |_| {
        conversation.get();
        if let Some(div) = div_node_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    let conversation_view = move || {
        conversation
            .get()
            .messages
            .iter()
            .map(move |message| {
                let markdown_input = &message.text;
                let mut options = Options::empty();
                options.insert(Options::ENABLE_STRIKETHROUGH);
                let parser = Parser::new_ext(markdown_input, options);
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);

                if message.user {
                    // User message styling with dynamic width based on text
                    view! {
                        <div class="flex justify-end mb-2">
                            <div
                                class="bg-blue-500 text-white rounded-lg px-4 py-2 max-w-full shadow-md"
                                style="max-width: calc(100% - 2rem);"
                                inner_html=html_output
                            ></div>
                        </div>
                    }
                } else {
                    // AI response styling with dynamic width based on text
                    view! {
                        <div class="flex justify-start mb-2">
                            <div
                                class="bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-gray-100 rounded-lg px-4 py-2 max-w-full shadow-md"
                                style="max-width: calc(100% - 2rem);"
                                inner_html=html_output
                            ></div>
                        </div>
                    }
                }
            })
            .collect_view()
    };

    view! {
        <div
            class="flex-1 overflow-auto p-4 bg-gray-50 dark:bg-gray-800 max-h-full resize-y"
            node_ref=div_node_ref
        >
            {conversation_view}
        </div>
    }
}
