use std::time::Duration;

use sqlx::{pool::PoolOptions, postgres::PgConnectOptions, PgPool, Postgres};

use crate::settings::DatabaseSettings;

#[derive(Default)]
pub struct DbConnection {
    pool: Option<PgPool>,
}

impl DbConnection {
    /// There might be no pool available if the user decides to skip db checks.
    pub(crate) fn get_pool(&self) -> Option<PgPool> {
        self.pool.clone()
    }

    pub(crate) fn set_conn_settings(&mut self, settings: &DatabaseSettings) {
        let config = PgConnectOptions::new()
            .host(&settings.host)
            .port(settings.port)
            .username(&settings.username)
            .password(&settings.password)
            .database(&settings.database);

        let timeout = settings.conn_timeout_secs;

        let pool = PoolOptions::<Postgres>::new()
            .acquire_timeout(timeout)
            .acquire_slow_threshold(Duration::from_secs(2))
            .connect_lazy_with(config);

        self.pool = Some(pool);
    }
}
