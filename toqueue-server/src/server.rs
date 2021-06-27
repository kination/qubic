mod encoder;

use futures::SinkExt;
use http::{Request, Response};

use std::{error::Error, io};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed};

use crate::encoder::Http;

const SERVER_HOST: &str = "127.0.0.1:7700";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = TcpListener::bind(SERVER_HOST).await?;
    println!("Listening on: {}", SERVER_HOST);

    loop {
        let (stream, _) = server.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                println!("failed to process connection; error = {}", e);
            }
        });
    }
}

async fn process(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut transport = Framed::new(stream, Http);

    while let Some(request) = transport.next().await {
        match request {
            Ok(request) => {
                println!("request: {:?}", request);
                let response = respond(request).await?;
                transport.send(response).await?;
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

async fn respond(req: Request<()>) -> Result<Response<String>, Box<dyn Error>> {
    // req.uri.path?
    let body = String::from("test body");
    let response = Response::builder()
        .body(body)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    Ok(response)
}
