use std::{borrow, collections::{BTreeSet, BinaryHeap}};

use pg_diagnostics::Error;

use crate::{context::RuleContext, filter::AnalysisFilter, matcher::{GroupKey, RuleKey}, rule::{GroupCategory, Rule, RuleGroup}, signals::{RuleSignal, SignalEntry}, AnalyzerOptions};

pub trait RegistryVisitor {
    /// Record the category `C` to this visitor
    fn record_category<C: GroupCategory>(&mut self) {
        C::record_groups(self);
    }

    /// Record the group `G` to this visitor
    fn record_group<G: RuleGroup>(&mut self) {
        G::record_rules(self);
    }

    /// Record the rule `R` to this visitor
    fn record_rule<R>(&mut self)
    where
        R: Rule + 'static;
}


/// Key struct for a rule in the metadata map, sorted alphabetically
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MetadataKey {
    inner: (&'static str, &'static str),
}

impl MetadataKey {
    fn into_group_key(self) -> GroupKey {
        let (group, _) = self.inner;
        GroupKey::new(group)
    }

    fn into_rule_key(self) -> RuleKey {
        let (group, rule) = self.inner;
        RuleKey::new(group, rule)
    }
}

impl<'a> borrow::Borrow<(&'a str, &'a str)> for MetadataKey {
    fn borrow(&self) -> &(&'a str, &'a str) {
        &self.inner
    }
}

impl borrow::Borrow<str> for MetadataKey {
    fn borrow(&self) -> &str {
        self.inner.0
    }
}


/// Stores metadata information for all the rules in the registry, sorted
/// alphabetically
#[derive(Debug, Default)]
pub struct MetadataRegistry {
    inner: BTreeSet<MetadataKey>,
}

impl MetadataRegistry {
    /// Return a unique identifier for a rule group if it's known by this registry
    pub fn find_group(&self, group: &str) -> Option<GroupKey> {
        let key = self.inner.get(group)?;
        Some(key.into_group_key())
    }

    /// Return a unique identifier for a rule if it's known by this registry
    pub fn find_rule(&self, group: &str, rule: &str) -> Option<RuleKey> {
        let key = self.inner.get(&(group, rule))?;
        Some(key.into_rule_key())
    }

    pub(crate) fn insert_rule(&mut self, group: &'static str, rule: &'static str) {
        self.inner.insert(MetadataKey {
            inner: (group, rule),
        });
    }
}

impl RegistryVisitor for MetadataRegistry {
    fn record_rule<R>(&mut self)
    where
        R: Rule + 'static,
    {
        self.insert_rule(<R::Group as RuleGroup>::NAME, R::METADATA.name);
    }
}

pub struct RuleRegistryBuilder<'a> {
    filter: &'a AnalysisFilter<'a>,
    root: &'a pg_query_ext::NodeEnum,
    // Rule Registry
    registry: RuleRegistry,
    diagnostics: Vec<Error>,
}

// TOOD: add build, then run through rules in registry and emit signals
// i hope we can simplify this.
// - suppresions are on statement level anyways, so we can just run through the statements and check if they are suppressing certain rules
// - range is always the statement for now. we will add the weak ref thingy later
// - do we really need the RegistryRule stuff? the registry rule doesnt execute, it just provides
// the run function

impl RegistryVisitor for RuleRegistryBuilder<'_> {
    fn record_category<C: GroupCategory>(&mut self) {
        if self.filter.match_category::<C>() {
            C::record_groups(self);
        }
    }

    fn record_group<G: RuleGroup>(&mut self) {
        if self.filter.match_group::<G>() {
            G::record_rules(self);
        }
    }

    /// Add the rule `R` to the list of rules stored in this registry instance
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default> + 'static,
    {
        if !self.filter.match_rule::<R>() {
            return;
        }

        let rule = RegistryRule::new::<R>();

        self.registry.rules.push(rule);
    }
}

/// The rule registry holds type-erased instances of all active analysis rules
pub struct RuleRegistry {
    rules: Vec<RegistryRule>,
}

/// Internal representation of a single rule in the registry
#[derive(Copy, Clone)]
pub struct RegistryRule {
    run: RuleExecutor,
}


pub struct RegistryRuleParams<'analyzer, 'query> {
    pub root: &'analyzer pg_query_ext::NodeEnum,
    pub signal_queue: &'query mut BinaryHeap<SignalEntry<'analyzer>>,
    pub options: &'analyzer AnalyzerOptions,
}


/// Executor for rule as a generic function pointer
type RuleExecutor = fn(&mut RegistryRuleParams) -> Result<(), Error>;

impl RegistryRule {
    fn new<R>() -> Self
    where
        R: Rule<Options: Default> + 'static,
    {
        /// Generic implementation of RuleExecutor for any rule type R
        fn run<R>(
            params: &mut RegistryRuleParams
        ) -> Result<(), Error>
        where
            R: Rule<Options: Default> + 'static,
        {
            let options = params.options.rule_options::<R>().unwrap_or_default();
            let ctx = match RuleContext::new(
                params.root,
                &params.options.file_path,
                &options
            ) {
                Ok(ctx) => ctx,
                Err(error) => return Err(error),
            };

            for result in R::run(&ctx) {
                // `None` means the entire range of the statement
                let text_range = R::text_range(&ctx, &result);

                let signal = Box::new(RuleSignal::<R>::new(
                    params.root,
                    result,
                    params.options,
                ));

                params.signal_queue.push(SignalEntry {
                    signal,
                    rule: RuleKey::rule::<R>(),
                    text_range,
                });
            }

            Ok(())
        }

        Self {
            run: run::<R>,
        }
    }
}





