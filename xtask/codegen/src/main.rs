use xtask::{project_root, pushd, Result};

use xtask_codegen::{
    generate_crate, generate_new_analyser_rule, promote_rule, task_command, TaskCommand,
};

fn main() -> Result<()> {
    let _d = pushd(project_root());
    let result = task_command().fallback_to_usage().run();

    match result {
        TaskCommand::NewCrate { name } => {
            generate_crate(name)?;
        }
        TaskCommand::NewRule { name, category } => {
            generate_new_analyser_rule(category, &name);
        }
        TaskCommand::PromoteRule { name, group } => {
            promote_rule(&name, &group);
        }
    }

    Ok(())
}
