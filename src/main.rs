use std::{io::Write, net::TcpListener, str};

struct Response {
    message_size: i32,
    header: Header,
}

struct Header {
    correlation_id: i32,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let buffer = [0, 0, 0, 0, 0, 0, 0, 7];
                match _stream.write(&buffer) {
                    Ok(value) => {
                        println!("sent {} bytes", value);
                    }
                    Err(e) => {
                        println!("failed to send response: {}", e);
                    }
                };
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
