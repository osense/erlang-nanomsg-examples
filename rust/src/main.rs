extern crate nanomsg;

use nanomsg::{Socket, Protocol};
use std::io::{Read, Write};

const ADDRESS: &'static str = "ipc:///tmp/a.ipc";


fn main() {
    let mut socket = Socket::new(Protocol::Rep).unwrap();
    println!("Connecting to address '{}'", ADDRESS);
    let mut endpoint = socket.bind(ADDRESS).unwrap();
    
    let mut request = String::new();
    loop {
        println!("Waiting for a message");
        assert!(socket.read_to_string(&mut request).is_ok());
        println!("Received {}", request);

        match request.as_ref() {
            "PING" => {
                reply(&mut socket, "PONG");
            }
            "STOP" => {
                reply(&mut socket, "OK");
                println!("Shutting down");
                break;
            },
            _ => reply(&mut socket, "UNKNOWN REQUEST")
        } 

        request.clear(); 
    }

    assert!(endpoint.shutdown().is_ok());
}

fn reply(socket: &mut Socket, reply: &str) {
    assert!(socket.write_all(reply.as_bytes()).is_ok());
    println!("Replied with '{}'", reply)
}

