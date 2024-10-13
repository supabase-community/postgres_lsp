use lsp_server::Connection;
use pg_lsp::server::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (connection, threads) = Connection::stdio();

    let server = Server::init(connection)?;
    server.run().await?;
    threads.join()?;

    Ok(())
}
