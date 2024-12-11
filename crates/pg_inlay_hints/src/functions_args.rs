use pg_query_ext::ChildrenIterator;
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
                pg_query_ext::NodeEnum::FuncCall(source_fn) => {
                    pg_type_resolver::resolve_func_call(source_fn.as_ref(), params.schema_cache)
                        .map(|schema_fn| {
                            resolve_func_arg_hint(
                                source_fn.as_ref(),
                                schema_fn,
                                params.schema_cache,
                            )
                        })
                }
                _ => None,
            })
            .flatten()
            .collect()
    }
}

fn resolve_func_arg_hint(
    source_fn: &pg_query_ext::protobuf::FuncCall,
    schema_fn: &pg_schema_cache::Function,
    schema_cache: &pg_schema_cache::SchemaCache,
) -> Vec<InlayHint> {
    let mut hints = vec![];

    // todo support named args
    for (func_arg, schema_arg) in source_fn.args.iter().zip(schema_fn.args.args.iter()) {
        hints.push(InlayHint {
            offset: TextSize::try_from(
                pg_query_ext::get_location(func_arg.node.as_ref().unwrap())
                    .expect("function arg to have a location"),
            )
            .unwrap(),
            content: InlayHintContent::FunctionArg(FunctionArgHint {
                name: if schema_arg.name.is_empty() {
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
    use pg_schema_cache::SchemaCache;
    use sqlx::PgPool;

    use crate::{
        functions_args::FunctionArgHint,
        inlay_hint::{InlayHint, InlayHintContent, InlayHintsParams, InlayHintsResolver},
    };

    #[test]
    fn test_function_args() {
        let input = "select lower('TEST')";

        let conn_string = std::env::var("DATABASE_URL").unwrap();

        let pool = block_on(PgPool::connect(conn_string.as_str())).unwrap();

        let root = pg_query_ext::parse(input).unwrap();

        let res = pg_syntax::parse_syntax(input, &root);

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
