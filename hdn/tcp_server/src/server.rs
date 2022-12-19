use std::collections::HashMap;
use std::net::{
    TcpStream,
};
use std::io::{
    Read,
    BufReader,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use chrono::Utc;

enum LogStatus {
    Connection,
    Load,
    Store,
}


fn log_output(status: LogStatus, storage: HashMap<String, String>, data: Request) {
    // print!("{} ", server_ip);
    let time_date = Utc::now();
    print!("{}", time_date.to_rfc3339());
    match status {
        LogStatus::Connection => {
            println!("New client {}", data.request_type);
            println!("Connection established.");
            size_printer(storage);
        },
        LogStatus::Load => {
            print!("Received request to write new value {}", data.hash.unwrap());
            print!("by key {}.", data.key.unwrap());
            size_printer(storage);
        },
        LogStatus::Store => {
            print!("Received request to get value by key {}.", data.key.unwrap());
            size_printer(storage);
        }
    }
}

#[derive(Debug, Deserialize)]
struct Request {
    request_type: String,
    key: Option<String>,
    hash: Option<String>,
}

#[derive(Debug, Serialize)]
struct Feedback {
    response_status: String,
    requested_key: Option<String>,
    requested_hash: Option<String>,
}

fn size_printer(storage: HashMap<String, String>) {
    println!("Storage size: {}.", storage.len());
}

fn stream_function(mut stream: TcpStream, ans: Feedback) {    
    let ans_json = serde_json::to_vec(&ans);
    stream.write(&ans_json.unwrap()).unwrap();
}

fn store_function(mut storage: HashMap<String, String>, deserialized_data: Request, stream: TcpStream) {
    let ans;
    let store_data = Request { request_type: (deserialized_data.request_type.clone()), key: (deserialized_data.key.clone()), hash: (deserialized_data.hash.clone()) };
    storage.insert(deserialized_data.key.unwrap(), deserialized_data.hash.unwrap());
    ans = Feedback { response_status: ("success".to_string()), requested_key: (None), requested_hash: (None) };

    log_output(LogStatus::Store, storage, store_data);
    stream_function(stream, ans);
}

fn load_function(storage: HashMap<String, String>, deserialized_data: Request, stream: TcpStream) {
    let ans;
    let load_data = Request { request_type: (deserialized_data.request_type.clone()), key: (deserialized_data.key.clone()), hash: (deserialized_data.hash.clone()) };
    let output_data = Request { request_type: (deserialized_data.request_type.clone()), key: (deserialized_data.key.clone()), hash: (deserialized_data.hash.clone()) };
    if storage.contains_key(&deserialized_data.key.unwrap()) {
        ans = Feedback { response_status: ("success".to_string()), requested_key: (load_data.key), requested_hash: (Some(deserialized_data.hash.unwrap())) };
    } else {
        ans = Feedback { response_status: ("key not found".to_string()), requested_key: (None), requested_hash: (None) };
    }

    log_output(LogStatus::Load, storage, output_data);
    stream_function(stream, ans);
}

pub fn handle_connection(mut stream: TcpStream, storage: HashMap<String, String>) {

    let mut buffer = vec![];
    let request = BufReader::new(&mut stream).read(&mut buffer).unwrap();
    let deserialized_data: Request = serde_json::from_str(&request.to_string()).unwrap();
    let store_data = Request { request_type: (deserialized_data.request_type.clone()), key: (deserialized_data.key.clone()), hash: (deserialized_data.hash.clone()) };
    let load_data = Request { request_type: (deserialized_data.request_type.clone()), key: (deserialized_data.key.clone()), hash: (deserialized_data.hash.clone()) };

    match deserialized_data.request_type.as_str() {
        "store" => store_function(storage, store_data, stream),
        "load" => load_function(storage, load_data, stream),
        _ => log_output(LogStatus::Connection, storage, deserialized_data),
    }
}
