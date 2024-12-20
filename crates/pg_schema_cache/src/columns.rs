use crate::schema_cache::SchemaCacheItem;

enum ColumnClassKind {
    OrdinaryTable,
    View,
    MaterializedView,
    ForeignTable,
    PartitionedTable,
}

impl From<&str> for ColumnClassKind {
    fn from(value: &str) -> Self {
        match value {
            "r" => ColumnClassKind::OrdinaryTable,
            "v" => ColumnClassKind::View,
            "m" => ColumnClassKind::MaterializedView,
            "f" => ColumnClassKind::ForeignTable,
            "p" => ColumnClassKind::PartitionedTable,
            _ => panic!(
                "Columns belonging to a class with pg_class.relkind = '{}' should be filtered out in the query.",
                value
            ),
        }
    }
}

impl From<String> for ColumnClassKind {
    fn from(value: String) -> Self {
        ColumnClassKind::from(value.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub name: String,

    pub table_name: String,
    pub table_oid: i64,

    /// What type of class does this column belong to?
    pub class_kind: bool,

    pub schema_name: String,
    pub type_id: i64,

    pub is_nullable: bool,

    pub is_primary_key: bool,
    pub is_unique: bool,

    /// The Default "value" of the column. Might be a function call, hence "_expr".
    pub default_expr: Option<String>,

    pub varchar_length: Option<i32>,
    // /// None if the column is not a foreign key.
    // pub foreign_key: Option<ForeignKeyReference>,
    /// Comment inserted via `COMMENT ON COLUMN my_table.my_comment '...'`, if present.
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignKeyReference {
    pub schema: Option<String>,
    pub table: String,
    pub column: String,
}

impl SchemaCacheItem for Column {
    type Item = Column;

    async fn load(pool: &sqlx::PgPool) -> Result<Vec<Self::Item>, sqlx::Error> {
        sqlx::query_file_as!(Column, "src/queries/columns.sql")
            .fetch_all(pool)
            .await
    }
}
