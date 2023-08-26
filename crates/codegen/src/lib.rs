mod syntax_kind;

use syntax_kind::syntax_kind_mod;

#[proc_macro]
pub fn syntax_kind(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syntax_kind_mod(item.into()).into()
}
