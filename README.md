<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Rusty Chat

Welcome to **Rusty Chat**, a full-stack web application built using the **Leptos** web framework, **Axum** backend, and **Tokio** for asynchronous operations. The app integrates a local **AI language model** using **Ollama** to offer intelligent chat responses. This project is designed for developers interested in building performant, real-time chat applications with a modern Rust stack.

## Features

- **Real-time Chat**: 💬 Leverage WebSockets to enable seamless real-time chat functionality.
- **AI-Powered Chat**: 🤖 Integrate Ollama to run local AI models for enhanced chat responses.
- **Full-Stack Development**: 🌐 Leptos is used for both the frontend and backend, with Axum as the backend server.
- **Async Framework**: ⚡ Tokio is used for handling asynchronous tasks efficiently.
- **TailwindCSS**: 🎨 Beautiful, responsive UI powered by TailwindCSS.
- **WebAssembly**: 🕹️ Support for WebAssembly (WASM) for better performance in the browser.

## Demo

🎥 **Demo Video**  
You can watch the demo of Rusty Chat here:  
[Add Demo Video Link]

## Project Setup

### Prerequisites

To get started with **Rusty Chat**, you'll need the following tools installed:

- **Rust (Nightly version)**: Install Rust through [rustup](https://rustup.rs/).
- **Node.js & npm**: Required for managing frontend dependencies and building assets.
- **cargo-leptos**: Install this tool to manage Leptos-specific tasks.

Install **cargo-leptos** with the following command:

```bash
cargo install cargo-leptos --locked
```

### Clone the Repository

Clone the project to your local machine:

```bash
git clone https://github.com/Abhishek2010DevSingh/RustyChat
cd RustyChat
```

### Install Dependencies

Install the required dependencies using Cargo:

```bash
cargo build
```

For frontend assets, you can install the necessary npm packages by running:

```bash
npm install
```

### Development Environment

Start the development server with the following command:

```bash
cargo leptos watch
```

This will compile the Rust backend and the frontend assets, then serve the application locally.

By default, the app will be hosted at `127.0.0.1:3000`.

### Running the Application with TailwindCSS

Rusty Chat uses TailwindCSS for styling. Make sure the `tailwind-input-file` is set to `"style/main.css"` in your `Cargo.toml` file. You can customize the styles according to your needs.

To start the build process for the frontend assets, run the following:

```bash
npm run build
```

### Running the Application with AI Support

If you want to use the **AI chat** functionality with **Ollama**, make sure you have Ollama installed and set up locally.

```bash
cargo leptos watch --features "ssr"
```

This enables server-side rendering (SSR) along with Ollama's local AI model integration.

### Testing the Application

Rusty Chat includes end-to-end testing with Playwright. You can run tests using the following command:

```bash
cargo leptos end-to-end
```

To run the tests in release mode:

```bash
cargo leptos end-to-end --release
```

### Building for Production

For a production build, use the following command:

```bash
cargo leptos build --release
```

This will compile both the backend and the frontend assets and prepare them for deployment.

### Deploying

After building for release, you can deploy the compiled server and the static site to your production server.

Copy the following files from the `target` directory to your server:

1. **Server binary**: Located at `target/server/release`
2. **Site package**: Located at `target/site`

Set up the environment variables for your project:

```bash
LEPTOS_OUTPUT_NAME="rusty-chat"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary:

```bash
./target/server/release/rusty-chat
```

### Development with WebAssembly (WASM)

If you plan to run the app with WebAssembly, make sure the `wasm32-unknown-unknown` target is added:

```bash
rustup target add wasm32-unknown-unknown
```

You can then build the app for WebAssembly by running:

```bash
cargo leptos build --target wasm32-unknown-unknown
```

### Iconography

The app's design uses modern icons and clean UI elements that align with current web design trends. Custom icons are integrated into the app for a unique look and feel. Here's a small preview:

- **Chat bubble**: 💬
- **AI-powered**: 🤖
- **Real-time**: ⚡
- **TailwindCSS**: 🎨

### Support

If you encounter any issues or need help, feel free to open an issue on the [GitHub repository](https://github.com/Abhishek2010DevSingh/RustyChat).

## Conclusion

Rusty Chat is a full-stack, real-time web chat app built with cutting-edge technologies in Rust. With features like local AI integration, real-time messaging, and full-stack development using Leptos and Axum, it provides a modern and efficient platform for building chat applications.

We hope this template helps you get started with your next full-stack web project!

Happy coding! 🎉

Feel free to replace the demo video link with the actual one when you have it.
