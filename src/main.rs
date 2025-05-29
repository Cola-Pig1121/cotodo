// main.rs
use std::net::TcpListener;
use std::sync::Arc;

mod ui;
mod ai;
mod server;
mod models;

fn main() {
    // Initialize the user interface
    let ui = ui::initialize_ui();

    // Set up the server
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind to address");
    let server = Arc::new(server::Server::new(listener));

    // Start the server in a separate thread
    std::thread::spawn(move || {
        server.run();
    });

    // Run the UI event loop
    ui.run();
}