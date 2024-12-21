use pg_analyse::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use pg_console::markup;

declare_lint_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```sql,expect_diagnostic
    /// select 1;
    /// ```
    ///
    /// ### Valid
    ///
    /// ``sql`
    /// select 2;
    /// ```
    ///
    pub BanDropColumn {
        version: "next",
        name: "banDropColumn",
        recommended: false,
    }
}

impl Rule for BanDropColumn {
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Vec<RuleDiagnostic> {
        Vec::new()
    }
}
