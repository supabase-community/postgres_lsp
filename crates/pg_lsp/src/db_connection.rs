use pg_schema_cache::SchemaCache;
use sqlx::{postgres::PgListener, PgPool};

#[derive(Debug)]
pub(crate) struct DbConnection {
    pub pool: PgPool,
    connection_string: String,
}

impl DbConnection {
    pub(crate) async fn new(connection_string: String) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(&connection_string).await?;
        Ok(Self {
            pool,
            connection_string: connection_string,
        })
    }

    pub(crate) async fn refresh_db_connection(
        self,
        connection_string: Option<String>,
    ) -> anyhow::Result<Self> {
        if connection_string.is_none()
            || connection_string.as_ref() == Some(&self.connection_string)
        {
            return Ok(self);
        }

        self.pool.close().await;

        let conn = DbConnection::new(connection_string.unwrap()).await?;

        Ok(conn)
    }

    pub(crate) async fn start_listening<F>(&self, on_schema_update: F) -> anyhow::Result<()>
    where
        F: Fn() -> () + Send + 'static,
    {
        let mut listener = PgListener::connect_with(&self.pool).await?;
        listener.listen_all(["postgres_lsp", "pgrst"]).await?;

        loop {
            match listener.recv().await {
                Ok(notification) => {
                    if notification.payload().to_string() == "reload schema" {
                        on_schema_update();
                    }
                }
                Err(e) => {
                    eprintln!("Listener error: {}", e);
                    return Err(e.into());
                }
            }
        }
    }

    pub(crate) async fn get_schema_cache(&self) -> SchemaCache {
        SchemaCache::load(&self.pool).await
    }
}
