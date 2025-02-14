use regex::Regex;

pub(crate) fn replace_section(
    content: &str,
    section_identifier: &str,
    replacement: &str,
) -> String {
    let pattern = format!(
        r"(\[//\]: # \(BEGIN {}\)\n)(?s).*?(\n\[//\]: # \(END {}\))",
        section_identifier, section_identifier
    );
    let re = Regex::new(&pattern).unwrap();
    re.replace_all(content, format!("${{1}}{}${{2}}", replacement))
        .to_string()
}
