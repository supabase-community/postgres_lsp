use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use sqlx::PgPool;

use crate::schema_cache::SchemaCacheItem;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Behavior {
    Immutable,
    Stable,
    #[default]
    Volatile,
}

impl From<Option<String>> for Behavior {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => match s.as_str() {
                "IMMUTABLE" => Behavior::Immutable,
                "STABLE" => Behavior::Stable,
                "VOLATILE" => Behavior::Volatile,
                _ => panic!("Invalid behavior"),
            },
            None => Behavior::Volatile,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FunctionArg {
    pub mode: String,
    pub name: String,
    pub type_id: i64,
    pub has_default: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FunctionArgs {
    pub args: Vec<FunctionArg>,
}

impl From<Option<JsonValue>> for FunctionArgs {
    fn from(s: Option<JsonValue>) -> Self {
        let args: Vec<FunctionArg> =
            serde_json::from_value(s.unwrap_or(JsonValue::Array(vec![]))).unwrap();
        FunctionArgs { args }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Function {
    pub id: Option<i64>,
    pub schema: Option<String>,
    pub name: Option<String>,
    pub language: Option<String>,
    pub definition: Option<String>,
    pub complete_statement: Option<String>,
    pub args: FunctionArgs,
    pub argument_types: Option<String>,
    pub identity_argument_types: Option<String>,
    pub return_type_id: Option<i64>,
    pub return_type: Option<String>,
    pub return_type_relation_id: Option<i64>,
    pub is_set_returning_function: bool,
    pub behavior: Behavior,
    pub security_definer: bool,
}

impl SchemaCacheItem for Function {
    type Item = Function;

    async fn load(pool: &PgPool) -> Result<Vec<Function>, sqlx::Error> {
        sqlx::query_file_as!(Function, "src/queries/functions.sql")
            .fetch_all(pool)
            .await
    }
}
