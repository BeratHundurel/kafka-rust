use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bytes::{Buf, BufMut, BytesMut};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9092").await?;
    println!("Server listening on 127.0.0.1:9092");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(&mut socket).await {
                eprintln!("Client error: {}", e);
            }
        });
    }
}

async fn handle_client(socket: &mut tokio::net::TcpStream) -> tokio::io::Result<()> {
    let mut buf = BytesMut::with_capacity(8);

    buf.resize(4, 0);
    socket.read_exact(&mut buf).await?;
    let len = i32::from_be_bytes(buf[..4].try_into().unwrap()) as usize;

    buf.resize(len, 0);
    socket.read_exact(&mut buf).await?;

    let mut request = &buf[..];
    let _request_api_key = request.get_i16();
    let _request_api_version = request.get_i16();
    let correlation_id = request.get_i32();

    let mut response = BytesMut::with_capacity(8);
    response.put_i32(0); // Error code = 0 (success)
    response.put_i32(correlation_id); // Echo correlation ID

    socket.write_all(&response).await?;

    Ok(())
}
