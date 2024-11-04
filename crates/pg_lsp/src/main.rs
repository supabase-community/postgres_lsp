use pg_lsp::server::Server;
use tower_lsp::LspService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (server, client_socket) = LspService::build(|client| Server::new(client)).finish();

    Ok(())
}
