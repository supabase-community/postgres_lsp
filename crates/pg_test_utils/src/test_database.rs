use sqlx::{postgres::PgConnectOptions, Executor, PgPool};
use uuid::Uuid;

// TODO: Work with proper config objects instead of a connection_string.
// With the current implementation, we can't parse the password from the connection string.
pub async fn get_new_test_db(connection_string: String, database_password: String) -> PgPool {
    let options_from_conn_str: PgConnectOptions = connection_string
        .parse()
        .expect("Invalid Connection String");

    let host = options_from_conn_str.get_host();
    assert!(
        host == "localhost" || host == "127.0.0.1",
        "Running tests against non-local database!"
    );

    let options_without_db_name = PgConnectOptions::new()
        .host(host)
        .port(options_from_conn_str.get_port())
        .username(options_from_conn_str.get_username())
        .password(&database_password);

    let postgres = sqlx::PgPool::connect_with(options_without_db_name.clone())
        .await
        .expect("Unable to connect to test postgres instance");

    let database_name = Uuid::new_v4().to_string();

    postgres
        .execute(format!(r#"create database "{}";"#, database_name).as_str())
        .await
        .expect("Failed to create test database.");

    sqlx::PgPool::connect_with(options_without_db_name.database(&database_name))
        .await
        .expect("Could not connect to test database")
}
