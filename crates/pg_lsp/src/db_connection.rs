use pg_schema_cache::SchemaCache;
use sqlx::{postgres::PgListener, PgPool};
use tokio::task::JoinHandle;

pub(crate) struct DbConnection {
    pool: PgPool,
    connection_string: String,
    schema_update_handle: JoinHandle<()>,
    close_tx: tokio::sync::oneshot::Sender<()>,
}

impl DbConnection {
    pub(crate) async fn new<F>(
        connection_string: String,
        on_schema_update: F,
    ) -> Result<Self, sqlx::Error>
    where
        F: Fn(SchemaCache) -> () + Send + 'static,
    {
        let pool = PgPool::connect(&connection_string).await?;

        let mut listener = PgListener::connect_with(&pool).await?;
        listener.listen_all(["postgres_lsp", "pgrst"]).await?;

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
                                    let schema_cache = SchemaCache::load(&cloned_pool).await;
                                    on_schema_update(schema_cache);
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

    pub(crate) async fn close(self) {
        let _ = self.close_tx.send(());
        let _ = self.schema_update_handle.await;

        self.pool.close().await;
    }

    pub(crate) fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }
}
