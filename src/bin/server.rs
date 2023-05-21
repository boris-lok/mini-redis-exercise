use std::io;

use tokio::net::{TcpListener, TcpStream};
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = 12345;
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;

    if let Ok(stream) = accept(&listener).await {
        println!("We have stream now, so we can send back some data here.");

        loop {
            stream.readable().await?;

            let mut buf = [0; 4096];

            match stream.try_read(&mut buf) {
                Ok(0) => {
                    println!("Read EOF from stream.");
                    break;
                }
                Ok(n) => {
                    println!("read {} bytes", n);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(_) => {
                    println!("Got an error");
                    break;
                }
            }
        }
    }

    Ok(())
}

async fn accept(listener: &TcpListener) -> anyhow::Result<TcpStream> {
    let mut backoff = 1;

    loop {
        match listener.accept().await {
            Ok((socket, _)) => return Ok(socket),
            Err(err) => {
                if backoff > 64 {
                    return Err(err.into());
                }
            }
        }

        time::sleep(Duration::from_secs(backoff)).await;

        backoff *= 2;
    }
}
