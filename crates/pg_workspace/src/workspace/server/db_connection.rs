use sqlx::{pool::PoolOptions, postgres::PgConnectOptions, PgPool, Postgres};

use crate::settings::DatabaseSettings;

#[derive(Default)]
pub struct DbConnection {
    pool: Option<PgPool>,
}

impl DbConnection {
    /// Requires that you call `set_conn_settings` at least once before getting a pool.
    pub(crate) fn get_pool(&self) -> PgPool {
        self.pool
            .clone()
            .expect("The database has never been properly initialized.")
    }

    pub(crate) fn set_conn_settings(&mut self, settings: &DatabaseSettings) {
        let config = PgConnectOptions::new()
            .host(&settings.host)
            .port(settings.port)
            .username(&settings.username)
            .password(&settings.password)
            .database(&settings.database);

        let timeout = settings.conn_timeout_secs.clone();

        let pool = PoolOptions::<Postgres>::new()
            .acquire_timeout(timeout)
            .connect_lazy_with(config);

        self.pool = Some(pool);
    }
}
