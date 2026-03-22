use auth_server::server;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    server::run("127.0.0.1:8080").await?;

    Ok(())
}
