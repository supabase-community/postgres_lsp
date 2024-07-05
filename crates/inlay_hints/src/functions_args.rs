use sql_parser::ChildrenIterator;
use text_size::TextSize;

use crate::{
    inlay_hint::{InlayHint, InlayHintContent, InlayHintsResolver},
    InlayHintsParams,
};

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionArgHint {
    pub name: Option<String>,
    pub type_name: String,
}

impl InlayHintsResolver for FunctionArgHint {
    fn find_all(params: InlayHintsParams) -> Vec<InlayHint> {
        if params.ast.is_none() {
            return vec![];
        }

        // args of a function have a correct location in the AST
        // so we can make it even easier based off the plain root node
        let root = params.ast.unwrap();

        ChildrenIterator::new(root.to_owned())
            .filter_map(|n| match n {
                sql_parser::AstNode::FuncCall(source_fn) => {
                    if let Some(schema_fn) =
                        type_resolver::resolve_func_call(source_fn.as_ref(), &params.schema_cache)
                    {
                        Some(resolve_func_arg_hint(
                            source_fn.as_ref(),
                            schema_fn,
                            &params.schema_cache,
                        ))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .flatten()
            .collect()
    }
}

fn resolve_func_arg_hint(
    source_fn: &sql_parser::pg_query_protobuf::FuncCall,
    schema_fn: &schema_cache::Function,
    schema_cache: &schema_cache::SchemaCache,
) -> Vec<InlayHint> {
    let mut hints = vec![];

    // todo support named args
    for (func_arg, schema_arg) in source_fn.args.iter().zip(schema_fn.args.args.iter()) {
        hints.push(InlayHint {
            offset: TextSize::try_from(
                sql_parser::get_location(func_arg.node.as_ref().unwrap())
                    .expect("function arg to have a location"),
            )
            .unwrap(),
            content: InlayHintContent::FunctionArg(FunctionArgHint {
                name: if schema_arg.name == "" {
                    None
                } else {
                    Some(schema_arg.name.clone())
                },
                type_name: schema_cache
                    .types
                    .iter()
                    .find(|t| t.id == schema_arg.type_id)
                    .unwrap()
                    .name
                    .clone(),
            }),
        });
    }

    hints
}

#[cfg(test)]
mod tests {
    use async_std::task::block_on;
    use schema_cache::SchemaCache;
    use sql_parser::parse_ast;
    use sqlx::PgPool;

    use crate::{
        functions_args::FunctionArgHint,
        inlay_hint::{InlayHint, InlayHintContent, InlayHintsParams, InlayHintsResolver},
    };

    #[test]
    fn test_function_args() {
        let input = "select lower('TEST')";

        let conn_string = std::env::var("DB_CONNECTION_STRING").unwrap();

        let pool = block_on(PgPool::connect(conn_string.as_str())).unwrap();

        let root = sql_parser::parse_sql_statement(input).unwrap();

        let res = parse_ast(input, &root);

        let schema_cache = block_on(SchemaCache::load(&pool));

        let hints = FunctionArgHint::find_all(InlayHintsParams {
            ast: Some(&root),
            tree: None,
            schema_cache: &schema_cache,
            enriched_ast: Some(&res.ast),
            cst: Some(&res.cst),
        });

        assert_eq!(hints.len(), 1);
        assert_eq!(
            hints[0],
            InlayHint {
                offset: 13.into(),
                content: InlayHintContent::FunctionArg(FunctionArgHint {
                    name: None,
                    type_name: "text".to_string(),
                }),
            }
        );
    }
}
