use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt,AsyncReadExt};
use std::str::FromStr;
use uuid::Uuid;

// constants
const ECHO_PROXY_SERVER_ADDRESS: &str = "127.0.0.1:8001";
const ECHO_BACKEND_SERVER_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    println!("karin starting {}", ECHO_PROXY_SERVER_ADDRESS);

    match TcpListener::bind(ECHO_PROXY_SERVER_ADDRESS).await {
        Ok(listener) => {
            println!("karin listening {}", ECHO_PROXY_SERVER_ADDRESS);

            loop {
                match listener.accept().await {
                    Ok((socket, _)) => {
                        tokio::spawn(async move {
                            handle_connection(socket).await;
                        });
                    },
                    Err(e) => eprintln!("Error accepting connection: {}", e),
                }
            }
        },
        Err(e) => eprintln!("Failed to bind to {}: {}", ECHO_PROXY_SERVER_ADDRESS, e),
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let id = Uuid::new_v4();
    let mut buffer: [u8; 1024] = [0; 1024];

    match stream.read(&mut buffer).await {
        Ok(len) => {
            let message = String::from_utf8_lossy(&buffer[..len]);
            println!("{} - received: {}", id, message);

            match call_echo_backend(id, message.to_string()).await {
                Ok(echo_backend_message) => {
                    let output = format!("echo_backend says: {}", echo_backend_message);
                    if let Err(e) = stream.write_all(output.as_bytes()).await {
                        eprintln!("{} - error sending response: {}", id, e);
                    }
                    println!("{} - sent: {}", id, message);
                },
                Err(e) => eprintln!("{} - error calling echo_backend: {}", id, e),
            }
        },
        Err(e) => eprintln!("{} - error reading from stream: {}", id, e),
    }
}

async fn call_echo_backend(id:Uuid, message: String) -> Result<String, std::io::Error> {
    println!("{} - connecting to echo_backend: {}", id, ECHO_BACKEND_SERVER_ADDRESS);

    match TcpStream::connect(ECHO_BACKEND_SERVER_ADDRESS).await {
        Ok(mut stream) => {
            println!("{} - connected to echo_backend", id);
            stream.write_all(message.as_bytes()).await?;
            let mut buffer = [0; 1024];
            let len = stream.read(&mut buffer).await?;
            let message = String::from_utf8_lossy(&buffer[..len]);
            println!("{} - received from echo_backend: {}", id, message);
            Ok(message.to_string())
        },
        Err(e) => {
            println!("{} - couldn't connect to echo_backend: {}", id, e);
            Err(e)
        },
    }
}