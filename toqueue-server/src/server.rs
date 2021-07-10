#![warn(rust_2018_idioms)]
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};

use std::error::Error;

const SERVER_HOST: &str = "127.0.0.1:7700";


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(&SERVER_HOST).await?;
    println!("Listening on: {}", SERVER_HOST);

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut framed = BytesCodec::new().framed(socket);
            while let Some(message) = framed.next().await {
                match message {
                    Ok(bytes) => println!("bytes: {:?}", bytes),
                    Err(err) => println!("Socket closed with error: {:?}", err),
                }
            }
            
        });
    }
}
