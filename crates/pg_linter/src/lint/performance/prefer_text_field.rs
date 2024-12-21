use pg_analyse::{Rule, RuleContext};

pg_analyse::declare_lint_rule! {
    /// Changing the size of a varchar field requires an ACCESS EXCLUSIVE lock, that will prevent all reads and writes to the table.
    ///
    /// Use a text field with a CHECK CONSTRAINT makes it easier to change the max length.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```sql,expect_diagnostic
    /// CREATE TABLE "app_user" (
    ///     "id" serial NOT NULL PRIMARY KEY,
    ///     "email" varchar(100) NOT NULL
    /// );
    /// ```
    ///
    /// ### Valid
    ///
    /// ```sql
    /// CREATE TABLE "app_user" (
    ///     "id" serial NOT NULL PRIMARY KEY,
    ///     "email" TEXT NOT NULL
    /// );
    /// ALTER TABLE "app_user" ADD CONSTRAINT "text_size" CHECK (LENGTH("email") <= 100);
    /// ```
    ///
    pub PreferTextField {
        version: "0.0.1",
        name: "preferTextField",
        recommended: true,
    }
}

impl Rule for PreferTextField {
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Vec<pg_analyse::RuleDiagnostic> {
        todo!()
    }
}
