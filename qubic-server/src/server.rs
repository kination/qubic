#![warn(rust_2018_idioms)]
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};

use quinn::{Endpoint, ServerConfig, NewConnection,};

use ring::rand::*;

use std::error::Error;
use std::net::SocketAddr;

const SERVER_HOST: &str = "127.0.0.1:7700";

fn client_addr() -> SocketAddr {
    "127.0.0.1:5000".parse::<SocketAddr>().unwrap()
}

fn server_addr() -> SocketAddr {
    SERVER_HOST.parse::<SocketAddr>().unwrap()
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let mut endpoint_builder = Endpoint::builder();
    endpoint_builder.listen(ServerConfig::default());

    // Bind this endpoint to a UDP socket on the given server address. 
    let (endpoint, mut incoming) = endpoint_builder.bind(&server_addr())?;

    // Start iterating over incoming connections.
    while let Some(conn) = incoming.next().await {
        let mut connection: NewConnection = conn.await?;

        // Save connection somewhere, start transferring, receiving data, see DataTransfer tutorial.
    }
    


    Ok(())
}
