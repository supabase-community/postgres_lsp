//! Codegen tools. Derived from Biome's codegen

mod generate_crate;
mod generate_new_analyser_rule;

pub use self::generate_crate::generate_crate;
pub use self::generate_new_analyser_rule::generate_new_analyser_rule;
use bpaf::Bpaf;
use generate_new_analyser_rule::Category;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub enum TaskCommand {
    /// Creates a new crate
    #[bpaf(command, long("new-crate"))]
    NewCrate {
        /// The name of the crate
        #[bpaf(long("name"), argument("STRING"))]
        name: String,
    },
    /// Creates a new lint rule
    #[bpaf(command, long("new-lintrule"))]
    NewRule {
        /// Name of the rule
        #[bpaf(long("name"))]
        name: String,

        /// Name of the rule
        #[bpaf(long("category"))]
        category: Category,
    },
}
