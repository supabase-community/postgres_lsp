use xtask::{project_root, pushd, Result};

use xtask_codegen::{
    generate_crate, task_command, TaskCommand,
};

fn main() -> Result<()> {
    let _d = pushd(project_root());
    let result = task_command().fallback_to_usage().run();

    match result {
        TaskCommand::NewCrate { name } => {
            generate_crate(name)?;
        }
    }

    Ok(())
}

