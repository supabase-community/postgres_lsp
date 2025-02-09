//! Generated file, do not edit by hand, see `xtask/codegen`

use pg_analyse::declare_lint_group;
pub mod ban_drop_column;
pub mod ban_drop_not_null;
declare_lint_group! { pub Safety { name : "safety" , rules : [self :: ban_drop_column :: BanDropColumn , self :: ban_drop_not_null :: BanDropNotNull ,] } }
