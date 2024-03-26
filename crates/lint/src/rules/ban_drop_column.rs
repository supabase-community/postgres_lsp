use base_db::{Diagnostic, DiagnosticSource, Severity};

pub fn ban_drop_column(node: &parser::pg_query::NodeEnum) -> Vec<Diagnostic> {
    let mut errs = vec![];

    match &node {
        parser::pg_query::NodeEnum::AlterTableStmt(stmt) => {
            if stmt
                .cmds
                .iter()
                .find(|cmd| {
                    if let Some(cmd) = &cmd.node {
                        if let parser::pg_query::NodeEnum::AlterTableCmd(cmd) = cmd {
                            if parser::pg_query::protobuf::AlterTableType::AtDropColumn
                                == cmd.subtype()
                            {
                                return true;
                            }
                        }
                    }
                    false
                })
                .is_some()
            {
                errs.push(Diagnostic {
                    source: DiagnosticSource::Lint,
                    range: None,
                    message: "DROP COLUMN is not allowed".to_string(),
                    severity: Severity::Error,
                });
            }
        }
        _ => (),
    };

    errs
}
