use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{prelude::*, BufReader, Read};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, MutexGuard};

enum LogStatus {
    Connection,
    Load,
    Store,
}

#[derive(Debug, Deserialize, Clone)]
struct Request {
    request_type: String,
    key: Option<String>,
    hash: Option<String>,
}

#[derive(Debug, Serialize)]
struct Response {
    response_status: String,
    requested_key: Option<String>,
    requested_hash: Option<String>,
}

fn size_printer(storage: MutexGuard<HashMap<String, String>>) {
    print!("Storage size: {}.", storage.len());
}

fn log_output(status: LogStatus, storage: MutexGuard<HashMap<String, String>>, data: Request) {
    // print!("{} ", (*stream).peer_addr().unwrap());
    let time_date = Utc::now();
    print!("{} ", time_date.to_rfc3339());
    match status {
        LogStatus::Connection => {
            println!("New client {}", data.request_type);
            println!("Connection established.");
        }
        LogStatus::Load => {
            print!("Received request to write new value {}", data.hash.unwrap());
            println!("by key {}.", data.key.unwrap());
        }
        LogStatus::Store => {
            println!(
                "Received request to get value by key {}.",
                data.key.unwrap()
            );
        }
    }
    size_printer(storage);
}

fn store_response(
    storage: MutexGuard<HashMap<String, String>>,
    deserialized_data: Request,
    key: &mut Option<String>,
    hash: &mut Option<String>,
    resp: &mut Option<Response>,
) {
    let output_data = deserialized_data.clone();
    let ans = Response {
        response_status: ("success".to_string()),
        requested_key: (None),
        requested_hash: (None),
    };

    log_output(LogStatus::Store, storage, output_data);

    *key = deserialized_data.key;
    *hash = deserialized_data.hash;
    *resp = Some(ans);
}

fn load_response(
    storage: MutexGuard<HashMap<String, String>>,
    deserialized_data: Request,
    resp: &mut Option<Response>,
) {
    let mut ans = Response {
        response_status: ("none").to_string(),
        requested_key: None,
        requested_hash: None,
    };
    let load_data = deserialized_data.clone();
    let output_data = deserialized_data.clone();
    if storage.contains_key(&(deserialized_data.key.unwrap())) {
        ans = Response {
            response_status: ("success").to_string(),
            requested_key: load_data.key,
            requested_hash: Some(load_data.hash.unwrap()),
        };
    } else {
        ans.response_status = ("key not found").to_string();
    };
    *resp = Some(ans);
    log_output(LogStatus::Load, storage, output_data);
}

pub fn handle_connection(mut stream: TcpStream, arc_storage: Arc<Mutex<HashMap<String, String>>>) {
    let mut buffer = vec![];
    let request = BufReader::new(&mut stream).read(&mut buffer).unwrap();
    loop {
        let storage = arc_storage.lock().unwrap();
        let deserialized_data: Request = serde_json::from_str(&request.to_string()).unwrap();
        let store_data = deserialized_data.clone();
        let load_data = deserialized_data.clone();
        let mut key: Option<String> = None;
        let mut hash: Option<String> = None;
        print!("{} ", (stream).peer_addr().unwrap());
        let mut response: Option<Response> = None;
        match deserialized_data.request_type.as_str() {
            "store" => store_response(storage, store_data, &mut key, &mut hash, &mut response),
            "load" => load_response(storage, load_data, &mut response),
            _ => log_output(LogStatus::Connection, storage, deserialized_data),
        }

        let ans_json = serde_json::to_vec(&response);
        stream.write_all(&ans_json.unwrap()).unwrap();

        if key.is_some() && hash.is_some() {
            (*arc_storage.lock().unwrap()).insert(key.unwrap(), hash.unwrap());
        }
    }
}
