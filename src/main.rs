use anyhow::Ok;
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut socket = TcpStream::connect("127.0.0.1:12345").await?;

    // Write some data to socket
    socket.write_u8(1).await?;

    // flush all data
    socket.flush().await?;

    Ok(())
}
