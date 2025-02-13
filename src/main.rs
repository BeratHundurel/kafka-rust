#![allow(unused_imports)]
use std::{net::TcpListener, str};

struct Response {
    message_size: i32,
    header: Header,

}

struct Header{
    correlation_id: i32,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                Response {
                    message_size: 0,
                    header: Header {
                        correlation_id: 7,
                    }
                };
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
