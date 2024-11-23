use std::{fs::File, path::PathBuf, str::FromStr};

use pg_lsp::server::LspServer;
use tower_lsp::{LspService, Server};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = PathBuf::from_str("pglsp.log").expect("Opened the log file.");
    let file = File::create(path).expect("Could not open the file.");

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_span_events(FmtSpan::ENTER)
        .with_span_events(FmtSpan::CLOSE)
        .with_ansi(false)
        .with_writer(file)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    tracing::info!("Starting server.");

    let (service, socket) = LspService::new(|client| LspServer::new(client));

    Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}
