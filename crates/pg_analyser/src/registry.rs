//! Generated file, do not edit by hand, see `xtask/codegen`

use pg_analyse::RegistryVisitor;
pub fn visit_registry<V: RegistryVisitor>(registry: &mut V) {
    registry.record_category::<crate::lint::Lint>();
}