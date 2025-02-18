use pglt_analyse::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use pglt_console::markup;

declare_lint_rule! {
    /// Dropping a table may break existing clients.
    ///
    /// Update your application code to no longer read or write the table.
    ///
    /// Once the table is no longer needed, you can delete it by running the command "DROP TABLE mytable;".
    ///
    /// This command will permanently remove the table from the database and all its contents.
    /// Be sure to back up the table before deleting it, just in case you need to restore it in the future.
    ///
    /// ## Examples
    /// ```sql,expect_diagnostic
    /// drop table some_table;
    /// ```
    pub BanDropTable {
        version: "next",
        name: "banDropTable",
        recommended: false,
        sources: &[RuleSource::Squawk("ban-drop-table")],
    }
}

impl Rule for BanDropTable {
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Vec<RuleDiagnostic> {
        let mut diagnostics = vec![];

        if let pglt_query_ext::NodeEnum::DropStmt(stmt) = &ctx.stmt() {
            if stmt.remove_type() == pglt_query_ext::protobuf::ObjectType::ObjectTable {
                diagnostics.push(
                    RuleDiagnostic::new(
                        rule_category!(),
                        None,
                        markup! {
                            "Dropping a table may break existing clients."
                        },
                    )
                    .detail(
                        None,
                        "Update your application code to no longer read or write the table, and only then delete the table. Be sure to create a backup.",
                    ),
                );
            }
        }

        diagnostics
    }
}
