use std::{fs::File, path::PathBuf, str::FromStr};

use pg_lsp_new::ServerFactory;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // This is currently only used for local debugging.
    // The `pglsp.toml` and `pglsp.log` filepaths should be dynamic.

    let log_path = PathBuf::from_str("pglsp.log").expect("Opened the log file.");
    let log_file = File::create(log_path).expect("Could not open the file.");

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_span_events(FmtSpan::ENTER)
        .with_span_events(FmtSpan::CLOSE)
        .with_ansi(false)
        .with_writer(log_file)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let config_path = PathBuf::from_str("pglsp.toml").expect("Could not find the pglsp.toml file.");

    tracing::info!("Starting server.");

    ServerFactory::new(true)
        .create(Some(config_path))
        .accept(stdin, stdout)
        .await;

    Ok(())
}
