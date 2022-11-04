use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888");

    match listener {
        Ok(listener_output) => {
            for stream in listener_output.incoming() {
                match stream {
                    Ok(tpc_stream) => {
                        println!("Connection Established!");
                        handle_connection(tpc_stream)
                    }
                    Err(err) => {
                        println!("Error: {}", err)
                    }
                }
            }
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]))
}
