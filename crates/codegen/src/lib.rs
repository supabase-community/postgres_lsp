mod get_location;
mod get_node_properties;
mod get_nodes;
mod parser;
mod syntax_kind;

use parser::parser_mod;

#[proc_macro]
pub fn parser_codegen(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parser_mod(item.into()).into()
}
