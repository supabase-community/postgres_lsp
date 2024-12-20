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
    context::RuleContext, {macro_name}, Rule, RuleDiagnostic, Ast
}};
use pg_console::markup;

{macro_name}! {{
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
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

pub fn generate_new_analyser_rule(category: Category, rule_name: &str) {
    let rule_name_camel = Case::Camel.convert(rule_name);
    let crate_folder = project_root().join(format!("crates/pg_lint"));
    let test_folder = crate_folder.join("tests/specs/nursery");
    let rule_folder = match &category {
        Category::Lint => crate_folder.join("src/lint/nursery"),
    };
    // Generate rule code
    let code = generate_rule_template(
        &category,
        Case::Pascal.convert(rule_name).as_str(),
        rule_name_camel.as_str(),
    );
    if !rule_folder.exists() {
        std::fs::create_dir(rule_folder.clone()).expect("To create the rule folder");
    }
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
                r#"    "lint/nursery/{rule_name_camel}": "https://biomejs.dev/linter/rules/{kebab_case_rule}","#
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

    // Generate test code
    let tests_path = format!("{}/{rule_name_camel}", test_folder.display());
    let _ = std::fs::create_dir_all(tests_path);

    let test_file = format!("{}/{rule_name_camel}/valid.sql", test_folder.display());
    if std::fs::File::open(&test_file).is_err() {
        let _ = std::fs::write(
            test_file,
            "/* should not generate diagnostics */\n// var a = 1;",
        );
    }

    let test_file = format!("{}/{rule_name_camel}/invalid.sql", test_folder.display());
    if std::fs::File::open(&test_file).is_err() {
        let _ = std::fs::write(test_file, "var a = 1;\na = 2;\na = 3;");
    }
}
