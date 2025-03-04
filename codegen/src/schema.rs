use pglt_configuration::{PartialConfiguration, VERSION};
use schemars::{
    schema::{RootSchema, Schema, SchemaObject},
    schema_for,
};
use serde_json::to_string_pretty;
use std::{fs, path::Path};

/// Generates the lint rules index.
///
/// * `docs_dir`: Path to the docs directory.
pub fn generate_schema(docs_dir: &Path) -> anyhow::Result<()> {
    let schemas_dir = docs_dir.join("schemas");
    let latest_schema_dir = schemas_dir.join("latest");
    let latest_schema_path = latest_schema_dir.join("schema.json");

    let version_schema_dir = schemas_dir.join(VERSION);
    let version_schema_path = version_schema_dir.join("schema.json");

    if !latest_schema_dir.exists() {
        fs::create_dir_all(&latest_schema_dir)?;
    }

    if !version_schema_dir.exists() {
        fs::create_dir_all(&version_schema_dir)?;
    }

    let schema_content = get_configuration_schema_content()?;

    fs::write(latest_schema_path, &schema_content)?;
    fs::write(version_schema_path, &schema_content)?;

    Ok(())
}

/// Get the content of the configuration schema
pub(crate) fn get_configuration_schema_content() -> anyhow::Result<String> {
    let schema = rename_partial_references_in_schema(schema_for!(PartialConfiguration));

    Ok(to_string_pretty(&schema)?)
}

/// Strips all "Partial" prefixes from type names in the schema.
///
/// We do this to avoid leaking our `Partial` derive macro to the outside world,
/// since it should be just an implementation detail.
fn rename_partial_references_in_schema(mut schema: RootSchema) -> RootSchema {
    if let Some(meta) = schema.schema.metadata.as_mut() {
        if let Some(title) = meta.title.as_ref() {
            if let Some(stripped) = title.strip_prefix("Partial") {
                meta.title = Some(stripped.to_string());
            } else if title == "RuleWithOptions_for_Null" {
                meta.title = Some("RuleWithNoOptions".to_string());
            } else if title == "RuleWithFixOptions_for_Null" {
                meta.title = Some("RuleWithFixNoOptions".to_string());
            } else if title == "RuleConfiguration_for_Null" {
                meta.title = Some("RuleConfiguration".to_string());
            } else if title == "RuleFixConfiguration_for_Null" {
                meta.title = Some("RuleFixConfiguration".to_string());
            } else if let Some(stripped) = title.strip_prefix("RuleWithOptions_for_") {
                meta.title = Some(format!("RuleWith{stripped}"));
            } else if let Some(stripped) = title.strip_prefix("RuleWithFixOptions_for_") {
                meta.title = Some(format!("RuleWith{stripped}"));
            } else if let Some(stripped) = title
                .strip_prefix("RuleConfiguration_for_")
                .map(|x| x.strip_suffix("Options").unwrap_or(x))
            {
                meta.title = Some(format!("{stripped}Configuration"));
            } else if let Some(stripped) = title
                .strip_prefix("RuleFixConfiguration_for_")
                .map(|x| x.strip_suffix("Options").unwrap_or(x))
            {
                meta.title = Some(format!("{stripped}Configuration"));
            }
        }
    }

    rename_partial_references_in_schema_object(&mut schema.schema);

    schema.definitions = schema
        .definitions
        .into_iter()
        .map(|(mut key, mut schema)| {
            if let Some(stripped) = key.strip_prefix("Partial") {
                key = stripped.to_string();
            } else if key == "RuleWithOptions_for_Null" || key == "RuleWithFixOptions_for_Null" {
                key = if key == "RuleWithOptions_for_Null" {
                    "RuleWithNoOptions".to_string()
                } else {
                    "RuleWithFixNoOptions".to_string()
                };
                if let Schema::Object(schema_object) = &mut schema {
                    if let Some(object) = &mut schema_object.object {
                        object.required.remove("options");
                        object.properties.remove("options");
                    }
                }
            } else if key == "RuleConfiguration_for_Null" {
                key = "RuleConfiguration".to_string();
            } else if key == "RuleFixConfiguration_for_Null" {
                key = "RuleFixConfiguration".to_string();
            } else if let Some(stripped) = key.strip_prefix("RuleWithOptions_for_") {
                key = format!("RuleWith{stripped}");
            } else if let Some(stripped) = key.strip_prefix("RuleWithFixOptions_for_") {
                key = format!("RuleWith{stripped}");
            } else if let Some(stripped) = key
                .strip_prefix("RuleConfiguration_for_")
                .map(|x| x.strip_suffix("Options").unwrap_or(x))
            {
                key = format!("{stripped}Configuration");
            } else if let Some(stripped) = key
                .strip_prefix("RuleFixConfiguration_for_")
                .map(|x| x.strip_suffix("Options").unwrap_or(x))
            {
                key = format!("{stripped}Configuration");
            }

            if let Schema::Object(object) = &mut schema {
                rename_partial_references_in_schema_object(object);
            }

            (key, schema)
        })
        .collect();

    schema
}

fn rename_partial_references_in_schema_object(object: &mut SchemaObject) {
    if let Some(object) = &mut object.object {
        for prop_schema in object.properties.values_mut() {
            if let Schema::Object(object) = prop_schema {
                rename_partial_references_in_schema_object(object);
            }
        }
    }

    if let Some(reference) = &mut object.reference {
        if let Some(stripped) = reference.strip_prefix("#/definitions/Partial") {
            *reference = format!("#/definitions/{stripped}");
        } else if reference == "#/definitions/RuleWithOptions_for_Null" {
            *reference = "#/definitions/RuleWithNoOptions".to_string();
        } else if reference == "#/definitions/RuleWithFixOptions_for_Null" {
            *reference = "#/definitions/RuleWithFixNoOptions".to_string();
        } else if reference == "#/definitions/RuleConfiguration_for_Null" {
            *reference = "#/definitions/RuleConfiguration".to_string();
        } else if reference == "#/definitions/RuleFixConfiguration_for_Null" {
            *reference = "#/definitions/RuleFixConfiguration".to_string();
        } else if let Some(stripped) = reference.strip_prefix("#/definitions/RuleWithOptions_for_")
        {
            *reference = format!("#/definitions/RuleWith{stripped}");
        } else if let Some(stripped) =
            reference.strip_prefix("#/definitions/RuleWithFixOptions_for_")
        {
            *reference = format!("#/definitions/RuleWith{stripped}");
        } else if let Some(stripped) = reference
            .strip_prefix("#/definitions/RuleConfiguration_for_")
            .map(|x| x.strip_suffix("Options").unwrap_or(x))
        {
            *reference = format!("#/definitions/{stripped}Configuration");
        } else if let Some(stripped) = reference
            .strip_prefix("#/definitions/RuleFixConfiguration_for_")
            .map(|x| x.strip_suffix("Options").unwrap_or(x))
        {
            *reference = format!("#/definitions/{stripped}Configuration");
        }
    }

    if let Some(subschemas) = &mut object.subschemas {
        rename_partial_references_in_optional_schema_vec(&mut subschemas.all_of);
        rename_partial_references_in_optional_schema_vec(&mut subschemas.any_of);
        rename_partial_references_in_optional_schema_vec(&mut subschemas.one_of);

        rename_partial_references_in_optional_schema_box(&mut subschemas.not);
        rename_partial_references_in_optional_schema_box(&mut subschemas.if_schema);
        rename_partial_references_in_optional_schema_box(&mut subschemas.then_schema);
        rename_partial_references_in_optional_schema_box(&mut subschemas.else_schema);
    }
}

fn rename_partial_references_in_optional_schema_box(schema: &mut Option<Box<Schema>>) {
    if let Some(schema) = schema {
        if let Schema::Object(object) = schema.as_mut() {
            rename_partial_references_in_schema_object(object);
        }
    }
}

fn rename_partial_references_in_optional_schema_vec(schemas: &mut Option<Vec<Schema>>) {
    if let Some(schemas) = schemas {
        rename_partial_references_in_schema_slice(schemas);
    }
}

fn rename_partial_references_in_schema_slice(schemas: &mut [Schema]) {
    for schema in schemas {
        if let Schema::Object(object) = schema {
            rename_partial_references_in_schema_object(object);
        }
    }
}
