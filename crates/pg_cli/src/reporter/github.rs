use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use pg_console::{markup, Console, ConsoleExt};
use pg_diagnostics::PrintGitHubDiagnostic;
use std::io;

use super::UserHintsPayload;

pub(crate) struct GithubReporter {
    pub(crate) diagnostics_payload: DiagnosticsPayload,
    pub(crate) execution: Execution,
    pub(crate) user_hints: UserHintsPayload,
}

impl Reporter for GithubReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> io::Result<()> {
        visitor.report_diagnostics(&self.execution, self.diagnostics_payload)?;
        visitor.report_user_hints(&self.execution, self.user_hints)?;
        Ok(())
    }
}
pub(crate) struct GithubReporterVisitor<'a>(pub(crate) &'a mut dyn Console);

impl ReporterVisitor for GithubReporterVisitor<'_> {
    fn report_summary(
        &mut self,
        _execution: &Execution,
        _summary: TraversalSummary,
    ) -> io::Result<()> {
        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        _execution: &Execution,
        diagnostics_payload: DiagnosticsPayload,
    ) -> io::Result<()> {
        for diagnostic in &diagnostics_payload.diagnostics {
            if diagnostic.severity() >= diagnostics_payload.diagnostic_level {
                if diagnostic.tags().is_verbose() && diagnostics_payload.verbose {
                    self.0.log(markup! {{PrintGitHubDiagnostic(diagnostic)}});
                } else if !diagnostics_payload.verbose {
                    self.0.log(markup! {{PrintGitHubDiagnostic(diagnostic)}});
                }
            }
        }

        Ok(())
    }

    fn report_user_hints(
        &mut self,
        _execution: &Execution,
        payload: super::UserHintsPayload,
    ) -> io::Result<()> {
        for hint in payload.hints {
            self.0.log(markup! {{hint}});
        }
        Ok(())
    }
}
