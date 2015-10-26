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
        socket.read_to_string(&mut request).unwrap();
        println!("Received '{}'", request);

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

    endpoint.shutdown().unwrap();
}

fn reply(socket: &mut Socket, reply: &str) {
    socket.write_all(reply.as_bytes()).unwrap();
    println!("Replied with '{}'", reply)
}

