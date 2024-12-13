use pg_diagnostics::Diagnostic;
use std::fmt::Debug;

#[derive(Debug, Diagnostic)]
pub enum SchemaCacheError {
    DatabaseConnectionError(DatabaseConnectionError),
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "database/connection",
    severity = Error,
    message(
        description = "Unable to Load Database Schema",
        message("Database Error Message:: "{self.message})
    )
)]
pub struct DatabaseConnectionError {
    pub message: String,
    pub code: Option<String>,
}

impl From<sqlx::Error> for SchemaCacheError {
    fn from(err: sqlx::Error) -> Self {
        let db_err = err.as_database_error();
        if let Some(db_err) = db_err {
            Self::DatabaseConnectionError(DatabaseConnectionError {
                message: db_err.message().to_string(),
                code: db_err.code().map(|c| c.to_string()),
            })
        } else {
            Self::DatabaseConnectionError(DatabaseConnectionError {
                message: err.to_string(),
                code: None,
            })
        }
    }
}
