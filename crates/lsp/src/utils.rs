pub mod from_proto;
pub mod line_index_ext;
pub mod to_proto;

use std::path::PathBuf;

use base_db::PgLspPath;
use lsp_types;

/// Convert a `lsp_types::Url` to a `PgLspPath`.
pub(crate) fn file_path(url: &lsp_types::Url) -> PgLspPath {
    let path_to_file = match url.to_file_path() {
        Err(_) => {
            // If we can't create a path, it's probably because the file doesn't exist.
            // It can be a newly created file that it's not on disk
            PathBuf::from(url.path())
        }
        Ok(path) => path,
    };

    PgLspPath::new(path_to_file)
}

pub fn normalize_uri(uri: &mut lsp_types::Url) {
    if let Some(mut segments) = uri.path_segments() {
        if let Some(mut path) = segments.next().and_then(fix_drive_letter) {
            for segment in segments {
                path.push('/');
                path.push_str(segment);
            }

            uri.set_path(&path);
        }
    }

    uri.set_fragment(None);
}

fn fix_drive_letter(text: &str) -> Option<String> {
    if !text.is_ascii() {
        return None;
    }

    match &text[1..] {
        ":" => Some(text.to_ascii_uppercase()),
        "%3A" | "%3a" => Some(format!("{}:", text[0..1].to_ascii_uppercase())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use lsp_types::Url;

    use super::normalize_uri;

    #[test]
    fn test_lowercase_drive_letter() {
        let mut uri = Url::parse("file://c:/foo/bar.txt").unwrap();
        normalize_uri(&mut uri);
        assert_eq!(uri.as_str(), "file:///C:/foo/bar.txt");
    }

    #[test]
    fn test_uppercase_drive_letter() {
        let mut uri = Url::parse("file://C:/foo/bar.txt").unwrap();
        normalize_uri(&mut uri);
        assert_eq!(uri.as_str(), "file:///C:/foo/bar.txt");
    }

    #[test]
    fn test_fragment() {
        let mut uri = Url::parse("foo:///bar/baz.txt#qux").unwrap();
        normalize_uri(&mut uri);
        assert_eq!(uri.as_str(), "foo:///bar/baz.txt");
    }
}
