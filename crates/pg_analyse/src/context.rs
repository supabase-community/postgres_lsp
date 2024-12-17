use pg_diagnostics::{Error, Result};
use std::path::Path;

use crate::{categories::RuleCategory, rule::{GroupCategory, Rule, RuleGroup, RuleMetadata}};

pub struct RuleContext<'a, R: Rule> {
    stmt: &'a pg_query_ext::NodeEnum,
    file_path: &'a Path,
    options: &'a R::Options,
}

impl<'a, R> RuleContext<'a, R>
where
    R: Rule + Sized + 'static,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
    stmt: &'a pg_query_ext::NodeEnum,
        file_path: &'a Path,
        options: &'a R::Options,
    ) -> Result<Self, Error> {
        Ok(Self {
            stmt,
            file_path,
            options,
        })
    }

    /// Returns the group that belongs to the current rule
    pub fn group(&self) -> &'static str {
        <R::Group as RuleGroup>::NAME
    }

    /// Returns the category that belongs to the current rule
    pub fn category(&self) -> RuleCategory {
        <<R::Group as RuleGroup>::Category as GroupCategory>::CATEGORY
    }

    /// Returns a clone of the AST root
    pub fn stmt(&self) -> pg_query_ext::NodeEnum {
        self.stmt.clone()
    }

    /// Returns the metadata of the rule
    ///
    /// The metadata contains information about the rule, such as the name, version, language, and whether it is recommended.
    ///
    /// ## Examples
    /// ```rust,ignore
    /// declare_lint_rule! {
    ///     /// Some doc
    ///     pub(crate) Foo {
    ///         version: "0.0.0",
    ///         name: "foo",
    ///         language: "js",
    ///         recommended: true,
    ///     }
    /// }
    ///
    /// impl Rule for Foo {
    ///     const CATEGORY: RuleCategory = RuleCategory::Lint;
    ///     type State = ();
    ///     type Signals = ();
    ///     type Options = ();
    ///
    ///     fn run(ctx: &RuleContext<Self>) -> Self::Signals {
    ///         assert_eq!(ctx.metadata().name, "foo");
    ///     }
    /// }
    /// ```
    pub fn metadata(&self) -> &RuleMetadata {
        &R::METADATA
    }

    /// It retrieves the options that belong to a rule, if they exist.
    ///
    /// In order to retrieve a typed data structure, you have to create a deserializable
    /// data structure and define it inside the generic type `type Options` of the [Rule]
    ///
    pub fn options(&self) -> &R::Options {
        self.options
    }

    /// The file path of the current file
    pub fn file_path(&self) -> &Path {
        self.file_path
    }
}

