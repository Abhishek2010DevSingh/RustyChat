use crate::{component::prelude::*, model::Conversation};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(RwSignal::new(Conversation::default()));
    view! {
        <Stylesheet id="leptos" href="/pkg/rusty-chat.css" />
        <Title text="RustyChat" />

        <div class="flex flex-col h-screen">
            <ChatsView />
            <ChatInput />
        </div>
    }
}
