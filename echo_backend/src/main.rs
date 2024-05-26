use std::env::args;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{thread, time::Duration};

const ECHO_BACKEND_SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn handle_connection(mut stream: TcpStream, delay:u64) {
    // create the buffer
    let mut buffer = [0; 1024];

    // read the stream
    let len = stream.read(&mut buffer).unwrap();

    // output the request
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("received: {}", message);

    // put a delay in
    thread::sleep(Duration::from_millis(delay));

    // send out message
    let _ = stream.write_all(message.as_bytes());
    println!("sent: {}", message);
}


fn main() {
    let delay = args()
        .nth(1)
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or_default();

    // starting
    println!("ECHO_BACKEND starting {}", ECHO_BACKEND_SERVER_ADDRESS);

    // adding a delay
    println!("ECHO_BACKEND has a delay of {} milliseconds", delay);
    
    // bind
    let listener = TcpListener::bind(ECHO_BACKEND_SERVER_ADDRESS).unwrap();

    // starting
    println!("ECHO_BACKEND listening {}", ECHO_BACKEND_SERVER_ADDRESS);

    // loop through incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // handle connection
        handle_connection(stream, delay);
    }

}
