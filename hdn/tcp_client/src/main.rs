use std::{net::TcpStream, io::Write};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    request_type: String,
    key: Option<String>,
    hash: Option<String>,
}


impl Request {
    pub fn serealization(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }
}

fn send_to_server(mut stream: TcpStream) {
    println!("Successfully connected to server");

    let message: Request = Request { request_type: ("store".to_string()), key: (Some("some_key".to_string())), hash: (Some("0b672dd94fd3da6a8d404b66ee3f0c83".to_string())) };

    let request = message.serealization().unwrap();

    stream.write(&request).unwrap();
}
 
fn main() {
    match TcpStream::connect("localhost:3333") {
        Err(e) => {
            println!("Failed to connect: {}", e);
        },
        Ok(stream) => {
            send_to_server(stream);
        }
    }
}
