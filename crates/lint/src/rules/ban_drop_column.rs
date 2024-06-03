use crate::{
    violations::{RuleViolation, RuleViolationKind},
    LinterParams,
};

pub fn ban_drop_column(params: &LinterParams) -> Vec<RuleViolation> {
    let mut errs: Vec<RuleViolation> = vec![];

    if let Some(enriched_ast) = params.enriched_ast {
        if let sql_parser::AstNode::AlterTableStmt(_) = &enriched_ast.root_node().node {
            for node in enriched_ast.iter_nodes() {
                if let sql_parser::AstNode::AlterTableCmd(cmd) = &node.node {
                    if cmd.subtype() == sql_parser::AlterTableType::AtDropColumn {
                        errs.push(RuleViolation::new(
                            RuleViolationKind::BanDropColumn,
                            Some(node.range()),
                            None,
                        ));
                    }
                }
            }
        }
    } else {
        match &params.ast {
            sql_parser::AstNode::AlterTableStmt(stmt) => {
                for cmd in &stmt.cmds {
                    if let Some(sql_parser::AstNode::AlterTableCmd(cmd)) = &cmd.node {
                        if cmd.subtype() == sql_parser::AlterTableType::AtDropColumn {
                            errs.push(RuleViolation::new(
                                RuleViolationKind::BanDropColumn,
                                None,
                                None,
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    errs
}
