use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use uuid::Uuid;

const KIREN_SERVER_ADDRESS: &str = "127.0.0.1:8001";
const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    //starting
    println!("Kiren starting {}", KIREN_SERVER_ADDRESS);

    //bind
    let listener = TcpListener::bind(KIREN_SERVER_ADDRESS).await.unwrap();

    //started
    println!("Kiren listening at {}", KIREN_SERVER_ADDRESS);

    //loop through incoming connections
    loop {
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    //create uuid
    let id = Uuid::new_v4();

    //create the buffer
    let mut buffer = [0; 1024];

    //read the stream
    let len = stream.read(&mut buffer).await.unwrap();

    //output the request
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("{} - received: {}", id, message);

    //cal sirocco
    let sirocco_message = call_sirocco(id, message.to_owned().to_string()).await;
    let output = format!("sirocco says: {}", sirocco_message);

    //send out message
    let _ = stream.write_all(output.as_bytes()).await;
    println!("{} - sent: {}", id, message);
}

async fn call_sirocco(id: Uuid, message: String) -> String {
    //connecting
    println!("connecting to sirocco : {}", SIROCCO_SERVER_ADDRESS);

    if let Ok(mut stream) = TcpStream::connect(SIROCCO_SERVER_ADDRESS).await {
        //connected message
        println!(
            "{} - connected to sirocco: {}:{}",
            id,
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port(),
        );

        //write to stream
        let _ = stream.write_all(message.as_bytes()).await;
        println!("{}- sent: {}", id, message);

        //read from stream
        let mut buffer = [0; 1024]; //1KB
        let len = stream.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer[..len]);
        println!("{}- received: {}", id, message);

        return message.to_owned().to_string();
    } else {
        println!("{}- failed to connect {}", id, SIROCCO_SERVER_ADDRESS);

        return String::from("sirocco is not available");
    }
}
