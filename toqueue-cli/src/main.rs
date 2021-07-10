use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;

const SERVER_HOST: &str = "127.0.0.1:7700";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(SERVER_HOST).await?;

    let result = stream.write(b"hello world\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());

    Ok(())
}
