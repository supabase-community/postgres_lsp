use biome_string_case::Case;
use bpaf::Bpaf;
use std::str::FromStr;
use xtask::project_root;

#[derive(Debug, Clone, Bpaf)]
pub enum Category {
    /// Lint rules
    Lint,
}

impl FromStr for Category {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "lint" => Ok(Self::Lint),
            _ => Err("Not supported"),
        }
    }
}

fn generate_rule_template(
    category: &Category,
    rule_name_upper_camel: &str,
    rule_name_lower_camel: &str,
) -> String {
    let macro_name = match category {
        Category::Lint => "declare_lint_rule",
    };
    format!(
        r#"use pg_analyse::{{
    context::RuleContext, {macro_name}, Rule, RuleDiagnostic
}};
use pg_console::markup;

{macro_name}! {{
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
    pub {rule_name_upper_camel} {{
        version: "next",
        name: "{rule_name_lower_camel}",
        recommended: false,
    }}
}}

impl Rule for {rule_name_upper_camel} {{
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Vec<RuleDiagnostic> {{
        Vec::new()
    }}
}}
"#
    )
}

static EXAMPLE_MUTED_SQL: &'static str = r#"
    /** expect-no-diagnostics */
    ## select 1;
"#;

static EXAMPLE_SQL: &'static str = r#"
    ## select 1;
"#;

pub fn generate_new_analyser_rule(category: Category, rule_name: &str, group: &str) {
    let rule_name_camel = Case::Camel.convert(rule_name);

    let crate_folder = project_root().join("crates/pg_analyser");

    let rule_folder = match &category {
        Category::Lint => crate_folder.join(format!("src/lint/{group}")),
    };
    if !rule_folder.exists() {
        std::fs::create_dir(rule_folder.clone()).expect("To create the rule folder");
    }

    // Generate rule code
    let code = generate_rule_template(
        &category,
        Case::Pascal.convert(rule_name).as_str(),
        rule_name_camel.as_str(),
    );
    let file_name = format!(
        "{}/{}.rs",
        rule_folder.display(),
        Case::Snake.convert(rule_name)
    );
    std::fs::write(file_name.clone(), code).unwrap_or_else(|_| panic!("To write {}", &file_name));

    let categories_path = "crates/pg_diagnostics_categories/src/categories.rs";
    let mut categories = std::fs::read_to_string(categories_path).unwrap();

    if !categories.contains(&rule_name_camel) {
        let kebab_case_rule = Case::Kebab.convert(&rule_name_camel);
        // We sort rules to reduce conflicts between contributions made in parallel.
        let rule_line = match category {
            Category::Lint => format!(
                r#"    "lint/{group}/{rule_name_camel}": "https://pglsp.dev/linter/rules/{kebab_case_rule}","#
            ),
        };
        let lint_start = match category {
            Category::Lint => "define_categories! {\n",
        };
        let lint_end = match category {
            Category::Lint => "\n    // end lint rules\n",
        };
        debug_assert!(categories.contains(lint_start), "{}", lint_start);
        debug_assert!(categories.contains(lint_end), "{}", lint_end);
        let lint_start_index = categories.find(lint_start).unwrap() + lint_start.len();
        let lint_end_index = categories.find(lint_end).unwrap();
        let lint_rule_text = &categories[lint_start_index..lint_end_index];
        let mut lint_rules: Vec<_> = lint_rule_text.lines().chain(Some(&rule_line[..])).collect();
        lint_rules.sort_unstable();
        let new_lint_rule_text = lint_rules.join("\n");
        categories.replace_range(lint_start_index..lint_end_index, &new_lint_rule_text);
        std::fs::write(categories_path, categories).unwrap();
    }

    let test_folder = match &category {
        Category::Lint => crate_folder.join(format!("tests/specs/lint/{group}")),
    };
    if !test_folder.exists() {
        std::fs::create_dir(test_folder.clone()).expect("To create the test folder");
    }

    let test_file_name = format!("{}/query.sql", test_folder.display(),);
    std::fs::write(test_file_name.clone(), EXAMPLE_SQL)
        .unwrap_or_else(|_| panic!("To write {}", &test_file_name));

    let muted_test_file_name = format!("{}/query_muted.sql", test_folder.display(),);
    std::fs::write(muted_test_file_name.clone(), EXAMPLE_MUTED_SQL)
        .unwrap_or_else(|_| panic!("To write {}", &muted_test_file_name));
}
