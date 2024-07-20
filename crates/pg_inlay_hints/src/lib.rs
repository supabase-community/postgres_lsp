mod functions_args;
mod inlay_hint;

use inlay_hint::InlayHintsResolver;

use crate::functions_args::FunctionArgHint;
pub use crate::inlay_hint::{InlayHint, InlayHintContent, InlayHintsParams};

pub fn inlay_hints(params: InlayHintsParams) -> Vec<InlayHint> {
    let mut hints = vec![];

    hints.extend(FunctionArgHint::find_all(params));

    hints
}
