//! Sirocco | Single treaded web server

use std::env::args;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

fn main() {
    //set a delay
    let delay = args()
        .nth(1)
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or_default();

    //starting
    println!("Sirocco starting at {}", SIROCCO_SERVER_ADDRESS);

    //adding a delay
    println!("Sirocco has a delay of {} milliseconds", delay);

    //bind
    let listener = TcpListener::bind(SIROCCO_SERVER_ADDRESS).unwrap();

    //listening
    println!("Sirocco listening at {}", SIROCCO_SERVER_ADDRESS);

    //loop through incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //handle connection
        handle_connection(stream, delay)
    }
}

fn handle_connection(mut stream: TcpStream, delay: u64) {
    //create the buffer
    let mut buffer = [0; 1024]; //1KB

    //read the stream
    let len = stream.read(&mut buffer).unwrap();

    //output
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("received : {}", message);

    //put a delay
    thread::sleep(Duration::from_millis(delay));

    //send out message
    let _ = stream.write(message.as_bytes());
    println!("sent: {}", message);
}
