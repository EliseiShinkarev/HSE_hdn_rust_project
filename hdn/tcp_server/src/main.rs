use std::collections::HashMap;
use std::net::{
    TcpListener,
};
pub mod server;

fn main() {
    let storage: HashMap<String, String> = HashMap::new();
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server is listening on port 3333");
    for stream in listener.incoming() {

        let moved_storage = storage.clone();

        match stream {
            Err(e) => {
                print!("Error: {}", e);
            },
            Ok(stream) => {
                println!("New connection: {} ", stream.peer_addr().unwrap());
                std::thread::spawn(move || { server::handle_connection(stream, moved_storage); });
            }
        }
    }
}
