use bytes::{BufMut, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

const UNSUPPORTED_VERSION_CODE: u16 = 35;

struct Header {
    api_key: u16,
    api_version: u16,
    corelation_id: u32,
}

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

async fn parse_header(stream: &mut TcpStream) -> Result<Header, std::io::Error> {
    let mut reader = BufReader::new(stream);
    let _header_len = reader.read_u32().await? as usize;
    let api_key = reader.read_u16().await?;
    let api_version = reader.read_u16().await?;
    let corelation_id = reader.read_u32().await?;
    Ok(Header {
        api_key,
        api_version,
        corelation_id,
    })
}

async fn handle_client(socket: &mut tokio::net::TcpStream) -> tokio::io::Result<()> {
    let header = parse_header(socket).await?;

    let mut response = BytesMut::with_capacity(64); // Adjusted for safe capacity

    response.put_u32(0); // Placeholder for message length
    response.put_u32(header.corelation_id);

    let error_code = if header.api_version as i16 >= 0 {
        0
    } else {
        UNSUPPORTED_VERSION_CODE
    };

    response.put_u16(error_code);

    response.put_u8(2); // Number of APIs (was incorrectly 2 before)
    response.put_i16(18); // API Key
    response.put_i16(0); // Min Version
    response.put_i16(4); // Max Version

    response.put_i32(0); // Throttle time ms
    response.put_u8(0); // TAG_BUFFER (UVarInt encoding)
    response.put_u8(0); // TAG_BUFFER (should be UVarInt, not i8)

    // Now fix the length at the beginning
    let message_length = (response.len() - 4) as u32; // Excluding the length field itself
    response[..4].copy_from_slice(&message_length.to_be_bytes());

    println!("Sending response: {:?}", response);

    socket.write_all(&response).await?;

    Ok(())
}
