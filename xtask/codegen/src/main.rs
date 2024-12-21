use xtask::{project_root, pushd, Result};

use xtask_codegen::{
    generate_analyser, generate_crate, generate_new_analyser_rule, task_command, TaskCommand,
};

fn main() -> Result<()> {
    let _d = pushd(project_root());
    let result = task_command().fallback_to_usage().run();

    match result {
        TaskCommand::Analyser => {
            generate_analyser()?;
        }
        TaskCommand::NewCrate { name } => {
            generate_crate(name)?;
        }
        TaskCommand::NewRule {
            name,
            category,
            group,
        } => {
            generate_new_analyser_rule(category, &name, &group);
        }
    }

    Ok(())
}
