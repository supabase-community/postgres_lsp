use sqlx::{postgres::PgQueryResult, Executor, PgPool};

use crate::command::{Command, CommandType};

pub struct ExecuteStatementCommand {
    statement: String,
}

impl ExecuteStatementCommand {
    pub fn new(statement: String) -> Self {
        Self { statement }
    }

    pub async fn run(&self, conn: Option<PgPool>) -> anyhow::Result<PgQueryResult> {
        if let Some(conn) = conn {
            match conn.execute(self.statement.as_str()).await {
                Ok(res) => Ok(res),
                Err(e) => Err(anyhow::anyhow!(e.to_string())),
            }
        } else {
            Err(anyhow::anyhow!("No connection to database".to_string()))
        }
    }

    pub fn trim_statement(stmt: String, max_length: usize) -> String {
        let len = stmt.len();
        if len <= max_length {
            return stmt;
        }

        let half = max_length / 2;
        let start = &stmt[..half];
        let end = &stmt[len - half + (max_length % 2)..];

        format!("{}...{}", start, end)
    }
}

impl Command for ExecuteStatementCommand {
    type ExecuteStatement = ExecuteStatementCommand;

    fn command_type() -> CommandType {
        CommandType::ExecuteStatement
    }
}
