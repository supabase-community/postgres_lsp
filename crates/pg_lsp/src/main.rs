use lsp_server::Connection;
use pg_lsp::server::Server;

fn main() -> anyhow::Result<()> {
    let (connection, threads) = Connection::stdio();
    Server::init(connection)?;
    threads.join()?;

    Ok(())
}
