
use tokio::io::{AsyncWriteExt,AsyncReadExt};
use tokio::net::TcpStream;
use tokio::io;

const ECHO_SERVER_ADDRESS: &'static str = "127.0.0.1:8001"; //"tcpbin.com:4242";

async fn process_message(mut stream: TcpStream) -> Result<(), io::Error> {
    println!("connected to echo server at {}", ECHO_SERVER_ADDRESS);

    let message = "Hello, world!";
    println!("sending message: {}", message);

    // Use the ? operator to propagate errors
    stream.write_all(message.as_bytes()).await?;

    println!("message sent successfully");

    let mut response = String::new();
    // Again, use the ? operator to propagate errors
    stream.read_to_string(&mut response).await?;

    println!("received response: {}", response);

    // Return Ok(()) to satisfy the expected return type of Result<(), io::Error>
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("connecting to echo server at {}", ECHO_SERVER_ADDRESS);

    match TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        Ok(stream) => {
            match process_message(stream).await {
                Ok(_) => {
                    println!("message processed successfully");
                },
                Err(e) => {
                    eprintln!("failed to process message: {}", e);
                }                
            }            
        },
        Err(e) => {
            eprintln!("failed to connect to echo server at {}: {}", ECHO_SERVER_ADDRESS, e);
        }
    }
}
