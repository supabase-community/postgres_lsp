//! Generated file, do not edit by hand, see `xtask/codegen`

use pg_analyse::declare_lint_group;

pub mod prefer_text_field;

declare_lint_group! {
    pub Performance {
        name : "performance" ,
        rules : [
            self :: prefer_text_field :: PreferTextField ,
        ]
     }
}
