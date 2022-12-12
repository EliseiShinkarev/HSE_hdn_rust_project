use std::{net::TcpStream, io::Write};

fn send_to_server(mut stream: TcpStream) {
    println!("Successfully connected to server");
    let message = "Hello, world!";

    let request = format!("{message}");
    stream.write(request.as_bytes()).unwrap();
    println!("");
}
 
fn main() {
    match TcpStream::connect("localhost:3333") {
        Err(e) => {
            println!("Failed to connect: {}", e);
        },
        Ok(mut stream) => {
            send_to_server(stream);
        }
    }
}