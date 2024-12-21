//! Generated file, do not edit by hand, see `xtask/codegen`

use pg_analyse::declare_lint_group;
pub mod ban_drop_column;
declare_lint_group! { pub Nursery { name : "nursery" , rules : [self :: ban_drop_column :: BanDropColumn ,] } }
