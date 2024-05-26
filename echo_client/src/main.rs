
use std::io::prelude::*;
use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &'static str = "tcpbin.com:4242";

fn process_echo(mut stream: TcpStream) {
    println!(
        "connected to echo server: {}:{}",
        stream.local_addr().unwrap().ip(),
        stream.local_addr().unwrap().port()
    );
    let message = "hello world \r\n";
    let _  = stream.write(message.as_bytes()).unwrap();
    print!("sent: {}", message);
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).unwrap();
    println!("echo: {}", String::from_utf8_lossy(&buffer));
}

fn main() {
    println!("connecting to echo server at {}", ECHO_SERVER_ADDRESS);

    //let mut stream = TcpStream::connect(ECHO_SERVER_ADDRESS).unwrap();
    match TcpStream::connect(ECHO_SERVER_ADDRESS) {
        Ok(mut stream  ) => {
            process_echo(stream);
        },
        Err(e) => {
            println!("failed to connected to echo server: {}", e);
        }
        
    }

}
