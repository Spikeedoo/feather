use std::{net::TcpListener, collections::HashMap};

mod connection;
mod command;
mod store;

use connection::handle_connection;
use store::Store;

fn main() {
    // Initialize data store
    let mut data: Store = HashMap::new();

    // Parse HOST & PORT as environment variables
    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("3333".to_string());

    // Listen for connections
    let server = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
    println!("[FEATHER] Listening on port {}...", port);

    // Capture incoming data
    for stream in server.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut data);
    }
}
