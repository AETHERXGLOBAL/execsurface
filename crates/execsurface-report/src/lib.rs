//! Deterministic human-readable reporting for ExecSurface verdict evidence.
//!
//! M6 deliberately does not synthesize SARIF source locations. Runtime effects
//! are rendered to a job summary and a JSON evidence artifact instead.

use execsurface_policy::{ChangeKind, EffectKind, FindingAction, Verdict, VerdictReport};

pub const SARIF_STATUS: &str = "not-generated:no-source-provenance";
pub const SARIF_REASON: &str =
    "SARIF not generated: current runtime evidence has no causal source-code location.";

pub fn render_markdown(report: &VerdictReport) -> String {
    let mut output = String::new();
    output.push_str("# ExecSurface — ");
    output.push_str(verdict_name(report.verdict));
    output.push_str("\n\n");

    if let Some(digest) = &report.baseline_digest {
        output.push_str("**Baseline:** ");
        output.push_str(&escape_markdown(digest));
        output.push_str("\n\n");
    }

    if let Some(target) = &report.target {
        output.push_str(&format!(
            "**Target:** exit_code={:?}, signal={:?}\n\n",
            target.exit_code, target.signal
        ));
    }

    let allow = report
        .findings
        .iter()
        .filter(|finding| finding.action == FindingAction::Allow)
        .count();
    let review = report
        .findings
        .iter()
        .filter(|finding| finding.action == FindingAction::Review)
        .count();
    let block = report
        .findings
        .iter()
        .filter(|finding| finding.action == FindingAction::Block)
        .count();

    output.push_str(&format!(
        "**Findings:** {} total — {} allow / {} review / {} block\n\n",
        report.findings.len(),
        allow,
        review,
        block
    ));

    if let Some(policy) = &report.policy {
        output.push_str("**Policy:** ");
        output.push_str(&escape_markdown(&policy.source));
        output.push_str("\n\n");
    }

    if !report.findings.is_empty() {
        output.push_str("| Action | Change | Effect | Matched rules |\n");
        output.push_str("| --- | --- | --- | --- |\n");
        for finding in &report.findings {
            let rules = if finding.matched_rules.is_empty() {
                "<default>".to_owned()
            } else {
                finding.matched_rules.join(", ")
            };
            output.push_str("| ");
            output.push_str(action_name(finding.action));
            output.push_str(" | ");
            output.push_str(change_name(finding.change));
            output.push_str(" | ");
            output.push_str(effect_name(finding.effect_kind));
            output.push_str(" | ");
            output.push_str(&escape_markdown(&rules));
            output.push_str(" |\n");
        }
        output.push('\n');
    }

    if let Some(error) = &report.error {
        output.push_str("## Error\n\n");
        output.push_str(&escape_markdown(error));
        output.push_str("\n\n");
    }

    output.push_str("## SARIF\n\n");
    output.push_str(SARIF_REASON);
    output.push_str("\n\n");
    output.push_str(
        "ExecSurface will not invent a repository file or source line solely to create a code-scanning annotation.\n",
    );

    output
}

pub fn verdict_name(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::Pass => "PASS",
        Verdict::Review => "REVIEW",
        Verdict::Block => "BLOCK",
        Verdict::Error => "ERROR",
    }
}

fn action_name(action: FindingAction) -> &'static str {
    match action {
        FindingAction::Allow => "ALLOW",
        FindingAction::Review => "REVIEW",
        FindingAction::Block => "BLOCK",
    }
}

fn change_name(change: ChangeKind) -> &'static str {
    match change {
        ChangeKind::Added => "ADDED",
        ChangeKind::Removed => "REMOVED",
        ChangeKind::Changed => "CHANGED",
    }
}

fn effect_name(effect: EffectKind) -> &'static str {
    match effect {
        EffectKind::ProcessSpawn => "PROCESS_SPAWN",
        EffectKind::ProcessExec => "PROCESS_EXEC",
        EffectKind::FileOpen => "FILE_OPEN",
        EffectKind::FileCreate => "FILE_CREATE",
        EffectKind::FileDelete => "FILE_DELETE",
        EffectKind::FileRename => "FILE_RENAME",
        EffectKind::NetworkConnect => "NETWORK_CONNECT",
    }
}

fn escape_markdown(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\n' | '\r' => escaped.push(' '),
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '\\' | '|' | '*' | '_' | '[' | ']' | '`' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;
    use execsurface_policy::{error_report, PolicySummary, VerdictReport};

    #[test]
    fn error_summary_is_deterministic_and_declares_no_sarif() {
        let report = error_report("observer failed");
        let first = render_markdown(&report);
        let second = render_markdown(&report);
        assert_eq!(first, second);
        assert!(first.starts_with("# ExecSurface — ERROR"));
        assert!(first.contains(SARIF_REASON));
    }

    #[test]
    fn markdown_escaping_prevents_table_and_html_injection() {
        let mut report = error_report("safe");
        report.verdict = Verdict::Pass;
        report.error = None;
        report.policy = Some(PolicySummary {
            schema_version: 1,
            source: "evil|<tag>\n**bold**".to_owned(),
            default_action: FindingAction::Review,
        });

        let markdown = render_markdown(&report);
        assert!(!markdown.contains("<tag>"));
        assert!(markdown.contains("evil\\|&lt;tag&gt;"));
        assert!(!markdown.contains("\n**bold**"));
    }

    #[test]
    fn sarif_disposition_is_explicit() {
        assert_eq!(SARIF_STATUS, "not-generated:no-source-provenance");
    }

    #[test]
    fn empty_pass_report_has_no_findings_table() {
        let report = VerdictReport {
            schema_version: 1,
            baseline_digest: Some("sha256:test".to_owned()),
            target: None,
            verdict: Verdict::Pass,
            policy: None,
            findings: vec![],
            error: None,
        };
        let markdown = render_markdown(&report);
        assert!(markdown.contains("0 total"));
        assert!(!markdown.contains("| Action |"));
    }
}
