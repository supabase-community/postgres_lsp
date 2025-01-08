use clap::*;

#[derive(Parser)]
#[command(name = "query-debugger", about = "Debugs a query")]
struct Args {
    #[arg(long = "file", short = 'f')]
    file: String,
}

fn main() {
    let args = Args::parse();

    let stmt = std::fs::read_to_string(&args.file).expect("Failed to read file.");

    let mut parser = tree_sitter::Parser::new();
    let lang = tree_sitter_sql::language();
    parser
        .set_language(lang.clone())
        .expect("Setting Language failed.");

    let tree = parser
        .parse(stmt.clone(), None)
        .expect("Failed to parse Statement");

    let results = relation_matches(tree.root_node(), &stmt);

    for r in results {
        println!("{}", r.to_full_name(&stmt))
    }
}

struct RelationMatch<'a> {
    schema: Option<tree_sitter::Node<'a>>,
    table: tree_sitter::Node<'a>,
}

impl<'a> RelationMatch<'a> {
    fn to_full_name(&self, stmt: &str) -> String {
        match self.schema {
            Some(s) => format!(
                "{}.{}",
                s.utf8_text(stmt.as_bytes()).unwrap(),
                self.table.utf8_text(stmt.as_bytes()).unwrap()
            ),
            None => format!("{}", self.table.utf8_text(stmt.as_bytes()).unwrap()),
        }
    }
}

fn relation_matches<'a>(root_node: tree_sitter::Node<'a>, stmt: &str) -> Vec<RelationMatch<'a>> {
    static QUERY: &str = r#"
    (relation
        (object_reference 
            (identifier)+ @schema_or_table
            "." 
            (identifier) @table
        )+
    )
    "#;

    let query =
        tree_sitter::Query::new(tree_sitter_sql::language(), QUERY).expect("Invalid Query!");

    let mut cursor = tree_sitter::QueryCursor::new();

    let matches = cursor.matches(&query, root_node, stmt.as_bytes());

    let mut to_return = vec![];

    for m in matches {
        if m.captures.len() == 1 {
            let capture = m.captures[0].node;
            to_return.push(RelationMatch {
                schema: None,
                table: capture,
            });
        }

        if m.captures.len() == 2 {
            let schema = m.captures[0].node;
            let table = m.captures[1].node;

            to_return.push(RelationMatch {
                schema: Some(schema),
                table,
            });
        }
    }

    to_return
}
