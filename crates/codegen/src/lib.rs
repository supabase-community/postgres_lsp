mod get_location;
mod get_nodes;
mod syntax_kind;

use get_location::get_location_mod;
use get_nodes::get_nodes_mod;
use syntax_kind::syntax_kind_mod;

#[proc_macro]
pub fn get_nodes(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    get_nodes_mod(item.into()).into()
}

#[proc_macro]
pub fn syntax_kind(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syntax_kind_mod(item.into()).into()
}

#[proc_macro]
pub fn get_location(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    get_location_mod(item.into()).into()
}
