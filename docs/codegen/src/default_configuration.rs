use std::{fs, path::Path};

use crate::utils::replace_section;

use pglt_configuration::PartialConfiguration;

pub fn generate_default_configuration(docs_dir: &Path) -> anyhow::Result<()> {
    let index_path = docs_dir.join("index.md");

    let printed_config = format!(
        "\n```toml\n{}```\n",
        toml::ser::to_string_pretty(&PartialConfiguration::init())?
    );

    let data = fs::read_to_string(&index_path)?;

    let new_data = replace_section(&data, "DEFAULT_CONFIGURATION", &printed_config);

    fs::write(&index_path, new_data)?;

    Ok(())
}
