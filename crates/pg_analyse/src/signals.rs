use crate::diagnostics::AnalyzerDiagnostic;
use crate::rule::RuleGroup;
use crate::{AnalyzerOptions, RuleKey};
use crate::{categories::ActionCategory, context::RuleContext, rule::Rule};
use pg_console::MarkupBuf;
use pg_diagnostics::{advice::CodeSuggestionAdvice, Applicability, CodeSuggestion, Error};
use text_size::{TextRange, TextSize};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::vec::IntoIter;

/// Event raised by the analyzer when a [Rule](crate::Rule)
/// emits a diagnostic, a code action, or both
pub trait AnalyzerSignal {
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic>;
}

/// Simple implementation of [AnalyzerSignal] generating a [AnalyzerDiagnostic]
/// from a provided factory function. Optionally, this signal can be configured
/// to also emit a code action, by calling `.with_action` with a secondary
/// factory function for said action.
pub struct DiagnosticSignal<D, T> {
    diagnostic: D,
    _diag: PhantomData<T>,
}

impl<D, T> DiagnosticSignal<D, T>
where
    D: Fn() -> T,
    Error: From<T>,
{
    pub fn new(factory: D) -> Self {
        Self {
            diagnostic: factory,
            _diag: PhantomData,
        }
    }
}

impl<D, T> AnalyzerSignal for DiagnosticSignal<D, T>
where
    D: Fn() -> T,
    Error: From<T>,
{
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic> {
        let diag = (self.diagnostic)();
        let error = Error::from(diag);
        Some(AnalyzerDiagnostic::from_error(error))
    }
}

/// Entry for a pending signal in the `signal_queue`
pub struct SignalEntry<'analyzer> {
    /// Boxed analyzer signal to be emitted
    pub signal: Box<dyn AnalyzerSignal + 'analyzer>,
    /// Unique identifier for the rule that emitted this signal
    pub rule: RuleKey,
    /// Text range in the statement this signal covers. If `None`, the signal covers the entire
    /// statement
    pub text_range: Option<TextRange>,
}


// SignalEntry is ordered based on the starting point of its `text_range`
impl<'analyzer> Ord for SignalEntry<'analyzer> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.text_range.map(|x| x.start()).unwrap_or(TextSize::from(0)).cmp(&self.text_range.map(|x| x.start()).unwrap_or(TextSize::from(0)))
    }
}

impl<'a> PartialOrd for SignalEntry<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Eq for SignalEntry<'a> {}

impl<'a> PartialEq for SignalEntry<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.text_range.map(|x| x.start()) == other.text_range.map(|x| x.start())
    }
}


/// Analyzer-internal implementation of [AnalyzerSignal] for a specific [Rule](crate::registry::Rule)
pub(crate) struct RuleSignal<'analyzer, R: Rule> {
    root: &'analyzer pg_query_ext::NodeEnum,
    state: R::State,
    /// A list of strings that are considered "globals" inside the analyzer
    options: &'analyzer AnalyzerOptions,
}

impl<'analyzer, R> RuleSignal<'analyzer, R>
where
    R: Rule + 'static,
{
    pub(crate) fn new(
        root: &'analyzer pg_query_ext::NodeEnum,
        state: R::State,
        options: &'analyzer AnalyzerOptions,
    ) -> Self {
        Self {
            root,
            state,
            options,
        }
    }
}

impl<'bag, R> AnalyzerSignal for RuleSignal<'bag, R>
where
    R: Rule<Options: Default> + 'static,
{
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic> {
        let options = self.options.rule_options::<R>().unwrap_or_default();
        let ctx = RuleContext::new(self.root, &self.options.file_path, &options).ok()?;

        R::diagnostic(&ctx, &self.state).map(AnalyzerDiagnostic::from)
    }
}
