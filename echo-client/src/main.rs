//! Echo Client
use std::io::{Read, Write};
use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:8000";

fn main() {
    //connection
    println!("Connecting to {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        println!(
            "Connected to {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        //set our message
        let message = "This is a message from echo client";
        let _ = stream.write(message.as_bytes());
        let _ = stream.flush();
        println!("sent : {}", message);

        //read the result
        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).unwrap(); // writing to the buffer from stream
        let response = String::from_utf8_lossy(&buffer[..len]);
        println!("received: {}", response);
    } else {
        println!(
            "Failed to connect to server at address : {}",
            ECHO_SERVER_ADDRESS
        );
    }
}
