use std::sync::Arc;

use pg_schema_cache::SchemaCache;
use pg_workspace::Workspace;
use sqlx::{postgres::PgListener, PgPool};
use tokio::{sync::RwLock, task::JoinHandle};

pub(crate) struct DbConnection {
    pool: PgPool,
    connection_string: String,
    schema_update_handle: JoinHandle<()>,
    close_tx: tokio::sync::oneshot::Sender<()>,
}

impl DbConnection {
    #[tracing::instrument(name = "Setting up new Database Connection…", skip(ide))]
    pub(crate) async fn new(
        connection_string: String,
        ide: Arc<RwLock<Workspace>>,
    ) -> Result<Self, sqlx::Error> {
        tracing::info!("Trying to connect to pool…");
        let pool = PgPool::connect(&connection_string).await?;
        tracing::info!("Connected to Pool.");

        let mut listener = PgListener::connect_with(&pool).await?;
        tracing::info!("Connected to Listener.");

        listener.listen_all(["postgres_lsp", "pgrst"]).await?;
        tracing::info!("Listening!");

        let (close_tx, close_rx) = tokio::sync::oneshot::channel::<()>();

        let cloned_pool = pool.clone();

        let schema_update_handle: JoinHandle<()> = tokio::spawn(async move {
            let mut moved_rx = close_rx;

            loop {
                tokio::select! {
                    res = listener.recv() => {
                        match res {
                            Ok(not) => {
                                if not.payload().to_string() == "reload schema" {
                                    let schema_cache = SchemaCache::load(&cloned_pool).await.unwrap();
                                    ide.write().await.set_schema_cache(schema_cache);
                                };
                            }
                            Err(why) => {
                                eprintln!("Error receiving notification: {:?}", why);
                                break;
                            }
                        }
                    }

                    _ = &mut moved_rx => {
                        return;
                    }
                }
            }
        });
        tracing::info!("Set up schema update handle.");

        Ok(Self {
            pool,
            connection_string: connection_string,
            schema_update_handle,
            close_tx,
        })
    }

    pub(crate) fn connected_to(&self, connection_string: &str) -> bool {
        connection_string == self.connection_string
    }

    #[tracing::instrument(name = "Closing DB Pool", skip(self))]
    pub(crate) async fn close(self) {
        let _ = self.close_tx.send(());
        let _ = self.schema_update_handle.await;

        self.pool.close().await;
    }

    pub(crate) fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }
}
