use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

const SERVER_IP: &str = "0.0.0.0";
const SERVER_PORT: u32 = 8000;

const BUFFER_SIZE: usize = 2048;


fn echo(mut client_stream: TcpStream) {
    let client_address = client_stream.peer_addr().unwrap();
    println!("Connection from {addr}", addr=client_address);

    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    loop  {
        let byte_count = match client_stream.read(&mut buffer) {
            Ok(count) => count,
            Err(e) => { println!("Failed receiving from client: {}", e); return; }
        };

        if byte_count == 0 { println!("Disconnected {}", client_address); return; }

        match client_stream.write(&mut buffer[..byte_count]) {
            Ok(_) => (),
            Err(e) => { println!("Filed sending to client: {}", e); return; }
        };
    }
}

fn main() -> std::io::Result<()> {
    let listener_address = format!("{ip}:{port}", ip=SERVER_IP, port=SERVER_PORT);
    let listener = TcpListener::bind(&listener_address)?;
    println!("Listening for connections at {}", listener_address);

    for client_stream in listener.incoming() {
        match client_stream {
            Ok(client_stream) => { echo(client_stream); },
            Err(e) => { print!("Error connecting to client: {}", e); },
        }
    }
    
    Ok(())
}
