use lsp::server::Server;
use lsp_server::Connection;

fn main() -> anyhow::Result<()> {
    let (connection, threads) = Connection::stdio();
    Server::init(connection)?;
    threads.join()?;

    Ok(())
}
