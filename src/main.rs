#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    use std::sync::Arc;

    use anyhow::Context;
    use axum::{routing::get, Extension, Router};
    use kalosm::language::{Llama, LlamaSource};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use rusty_chat::app::*;
    use tower_http::trace::TraceLayer;
    use tracing::Level;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    tracing::debug!("Downloading model...");
    let model = Llama::builder()
        .with_source(LlamaSource::llama_3_2_1b_chat())
        .build()
        .await
        .context("Failed to load the model.")?;
    tracing::debug!("Model downloading completed");

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(TraceLayer::new_for_http())
        .layer(Extension(Arc::new(model)))
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    tracing::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
