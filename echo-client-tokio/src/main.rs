use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const ECHO_SERVER_ADDRESS: &str = "localhost:8001";

#[tokio::main]
async fn main() {
    // println!("Hello, world!");

    //connection
    println!("Connecting to : {}", ECHO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        //connected
        println!(
            "
            Connected to {}:{}
            ",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        //send
        let message = "Message from echo client tokio";
        let _ = stream.write_all(message.as_bytes()).await.unwrap();
        println!("sent : {}", message);

        //receive
        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).await.unwrap();
        let response = String::from_utf8_lossy(&buffer[..len]);
        println!("received : {}", response);
    }else {
        println!("couldn't connect to echo server : {}", ECHO_SERVER_ADDRESS);
    }
}
