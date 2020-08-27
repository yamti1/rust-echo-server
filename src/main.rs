use std::net::{TcpListener, TcpStream};

const SERVER_IP: &str = "0.0.0.0";
const SERVER_PORT: u32 = 8000;

fn echo_to_client(_client_stream: TcpStream) {
    println!("Connected!");
}

fn main() -> std::io::Result<()> {
    let listener_address = format!("{ip}:{port}", ip=SERVER_IP, port=SERVER_PORT);
    let listener = TcpListener::bind(listener_address)?;
    for client_stream in listener.incoming() {
        echo_to_client(client_stream?);
    }
    
    Ok(())
}
