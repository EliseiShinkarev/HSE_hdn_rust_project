use std::collections::HashMap;
use std::net::{
    TcpListener,
    TcpStream,
};
use std::io::{
    Read,
    BufReader,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::fs::File;

use std::time::SystemTime;

#[derive(Debug, Deserialize)]
struct Request {
    request_type: String,
    key: Option<String>,
    hash: Option<String>,
}

struct Feedback {
    response_status: String,
    requested_key: Option<String>,
    requested_hash: Option<String>,
}

fn size_printer(mut storage: HashMap<String, String>) {
    print!("Storage size: {}", storage.len());
    println!(".");
}

fn handle_connection(mut stream: TcpStream, mut storage: HashMap<String, String>) {
    // let cur_time = SystemTime::now();
    // let ip = 

    // parse json file, use map
    //let reader = BufReader::new(&mut stream);

    let mut buffer = vec![];
    let mut request = BufReader::new(&mut stream).read(&mut buffer).unwrap();
    // let request = BufReader::new(&mut stream).read_until(b'}', &mut buffer).unwrap();
    // let mut request = String::from_utf8(buffer).unwrap();

    // let mut file = File::open("request").unwrap();
    // let mut data = String::new();
    // file.read_to_string(&mut data).unwrap();

    // let deserialized_data: Request = serde_json::from_str(&data).unwrap();
    let deserialized_data: Request = serde_json::from_str(&request.to_string()).unwrap();

    if deserialized_data.request_type == "store" {
        storage.insert(deserialized_data.key.unwrap(), deserialized_data.hash.unwrap());
        let mut ans: Feedback = Feedback { response_status: ("success".to_string()), requested_key: (None), requested_hash: (None) };

        print!("Received request to write new value {}", deserialized_data.hash.unwrap());
        print!("by key {}", deserialized_data.key.unwrap());
        print!(".");
        size_printer(storage);
        // { "response_status": "success"} [p rfr]
    } else if deserialized_data.request_type == "load" {
        if storage.contains_key(&deserialized_data.key.unwrap()) {
            let mut ans: Feedback = Feedback { response_status: ("success".to_string()), requested_key: (deserialized_data.key.clone()), requested_hash: (Some(deserialized_data.hash.unwrap())) };
            // { "response_status": "success",   "requested_key": "some_key", "requested_hash": "0b672dd94fd3da6a8d404b66ee3f0c83",}
        } else {
            let mut ans: Feedback = Feedback { response_status: ("key not found".to_string()), requested_key: (None), requested_hash: (None) };
            // {"response_status": "key not found",}
        }

        print!("Received request to get value by key {}", deserialized_data.key.unwrap());
        print!(".");
        size_printer(storage);
    } else {
        println!("New client {}", deserialized_data.request_type);
        print!("Connection established.");
        print!(".");
        size_printer(storage);
    }

    // println!("Received: {request}");
    // let response = format!("Somebody sent {request}");
    // stream.write_all(response.as_bytes()).unwrap();
}

fn main() {

    let mut storage: HashMap<String, String> = HashMap::new();

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server is listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                print!("Error: {}", e);
            },
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                std::thread::spawn(move || { handle_connection(stream, storage); });
            }
        }
    }
}