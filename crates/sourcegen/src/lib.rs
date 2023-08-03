//! Helpers to generate rust code
//!
//! This crate provides utilities to generate rust source code.

mod attribute;
mod builder;
mod comment;
mod enum_;
mod function;
mod implementation;
mod imports;
mod match_;
mod source_file;
mod struct_;

pub use attribute::Attribute;
pub use builder::Builder;
pub use comment::Comment;
pub use enum_::Enum;
pub use function::Function;
pub use implementation::Implementation;
pub use imports::Imports;
pub use match_::Match;
pub use source_file::SourceFile;
pub use struct_::Struct;

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::builder::Builder;
    use crate::enum_::Enum;
    use crate::function::Function;
    use crate::match_::Match;
    use crate::source_file::SourceFile;

    #[test]
    fn test_sourcegen() {
        let test_enum = Enum::new("TestEnum".into())
            .with_value("A".into(), None)
            .with_value("B".into(), Some("Test".into()))
            .finish();

        let test_match = Match::new("value".into())
            .with_arm("A".into(), "1".into())
            .with_arm("B".into(), "2".into())
            .finish();

        let test_fn = Function::new("TestEnum".into())
            .public()
            .with_body(test_match)
            .finish();

        assert_eq!(
            SourceFile::new()
                .add_block(test_enum)
                .add_block(test_fn)
                .finish(),
            "enum TestEnum {\n    A,\n    B = Test,\n}\n\npub fn TestEnum(){\nmatch value {\n    A => 1,\n    B => 2,\n}\n\n}\n\n"
        )
    }
}
