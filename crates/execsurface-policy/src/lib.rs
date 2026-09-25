//! Deterministic policy evaluation over M4 execution-surface evidence.
//!
//! Policy never mutates the baseline or the underlying diff.

use std::collections::BTreeSet;
use std::fmt;

use execsurface_diff::{ChangedEffect, DiffReport, TargetOutcome};
use execsurface_model::canonical::{CanonicalEffect, CanonicalNetworkEndpoint, PathClass};
use execsurface_model::FileOperation;
use serde::{Deserialize, Serialize};

pub const POLICY_SCHEMA_VERSION: u32 = 1;
pub const VERDICT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingAction {
    Allow,
    Review,
    Block,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Pass,
    Review,
    Block,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    Added,
    Removed,
    Changed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectKind {
    ProcessSpawn,
    ProcessExec,
    FileOpen,
    FileCreate,
    FileDelete,
    FileRename,
    NetworkConnect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub schema_version: u32,
    pub default_action: FindingAction,
    pub rules: Vec<PolicyRule>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyRule {
    pub id: String,
    pub action: FindingAction,
    #[serde(rename = "match")]
    pub matcher: RuleMatcher,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuleMatcher {
    #[serde(default)]
    pub change: Option<ChangeKind>,
    #[serde(default)]
    pub effect: Option<EffectKind>,
    #[serde(default)]
    pub path_class: Option<PathClass>,
    #[serde(default)]
    pub path_prefix: Option<String>,
    #[serde(default)]
    pub executable_family: Option<String>,
    #[serde(default)]
    pub network_ip: Option<String>,
    #[serde(default)]
    pub network_port: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerdictReport {
    pub schema_version: u32,
    pub baseline_digest: Option<String>,
    pub target: Option<TargetOutcome>,
    pub verdict: Verdict,
    pub policy: Option<PolicySummary>,
    pub findings: Vec<FindingDecision>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySummary {
    pub schema_version: u32,
    pub source: String,
    pub default_action: FindingAction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FindingDecision {
    pub change: ChangeKind,
    pub effect_kind: EffectKind,
    pub action: FindingAction,
    pub matched_rules: Vec<String>,
    pub evidence: FindingEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "change", rename_all = "snake_case")]
pub enum FindingEvidence {
    Added { effect: CanonicalEffect },
    Removed { effect: CanonicalEffect },
    Changed { finding: Box<ChangedEffect> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyError {
    UnsupportedSchema(u32),
    EmptyRuleId,
    DuplicateRuleId(String),
    EmptyMatcher(String),
    InvalidMatcher { rule_id: String, reason: String },
}

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchema(version) => {
                write!(f, "unsupported policy schema version: {version}")
            }
            Self::EmptyRuleId => write!(f, "policy rule id must not be empty"),
            Self::DuplicateRuleId(id) => write!(f, "duplicate policy rule id: {id}"),
            Self::EmptyMatcher(id) => {
                write!(f, "policy rule {id} must specify at least one match field")
            }
            Self::InvalidMatcher { rule_id, reason } => {
                write!(f, "invalid matcher for policy rule {rule_id}: {reason}")
            }
        }
    }
}

impl std::error::Error for PolicyError {}

pub fn builtin_review_policy() -> Policy {
    Policy {
        schema_version: POLICY_SCHEMA_VERSION,
        default_action: FindingAction::Review,
        rules: Vec::new(),
    }
}

pub fn validate_policy(policy: &Policy) -> Result<(), PolicyError> {
    if policy.schema_version != POLICY_SCHEMA_VERSION {
        return Err(PolicyError::UnsupportedSchema(policy.schema_version));
    }

    let mut ids = BTreeSet::new();
    for rule in &policy.rules {
        if rule.id.trim().is_empty() {
            return Err(PolicyError::EmptyRuleId);
        }
        if !ids.insert(rule.id.clone()) {
            return Err(PolicyError::DuplicateRuleId(rule.id.clone()));
        }
        if rule.matcher.is_empty() {
            return Err(PolicyError::EmptyMatcher(rule.id.clone()));
        }
        if rule
            .matcher
            .path_prefix
            .as_deref()
            .is_some_and(str::is_empty)
        {
            return Err(PolicyError::InvalidMatcher {
                rule_id: rule.id.clone(),
                reason: "path_prefix must not be empty".to_owned(),
            });
        }
        if let Some(effect) = rule.matcher.effect {
            if (rule.matcher.network_ip.is_some() || rule.matcher.network_port.is_some())
                && effect != EffectKind::NetworkConnect
            {
                return Err(PolicyError::InvalidMatcher {
                    rule_id: rule.id.clone(),
                    reason: "network_ip/network_port require effect=network_connect when effect is specified"
                        .to_owned(),
                });
            }
        }
    }
    Ok(())
}

pub fn evaluate(
    diff: &DiffReport,
    policy: &Policy,
    source: impl Into<String>,
) -> Result<VerdictReport, PolicyError> {
    validate_policy(policy)?;

    let mut findings = Vec::new();

    for effect in &diff.added {
        findings.push(decide_finding(
            ChangeKind::Added,
            effect,
            FindingEvidence::Added {
                effect: effect.clone(),
            },
            policy,
        ));
    }
    for effect in &diff.removed {
        findings.push(decide_finding(
            ChangeKind::Removed,
            effect,
            FindingEvidence::Removed {
                effect: effect.clone(),
            },
            policy,
        ));
    }
    for changed in &diff.changed {
        findings.push(decide_finding(
            ChangeKind::Changed,
            &changed.after,
            FindingEvidence::Changed {
                finding: Box::new(changed.clone()),
            },
            policy,
        ));
    }

    let verdict = if findings.is_empty() {
        Verdict::Pass
    } else {
        match findings
            .iter()
            .map(|finding| finding.action)
            .max()
            .unwrap_or(FindingAction::Allow)
        {
            FindingAction::Allow => Verdict::Pass,
            FindingAction::Review => Verdict::Review,
            FindingAction::Block => Verdict::Block,
        }
    };

    Ok(VerdictReport {
        schema_version: VERDICT_SCHEMA_VERSION,
        baseline_digest: Some(diff.baseline_digest.clone()),
        target: Some(diff.target.clone()),
        verdict,
        policy: Some(PolicySummary {
            schema_version: policy.schema_version,
            source: source.into(),
            default_action: policy.default_action,
        }),
        findings,
        error: None,
    })
}

pub fn error_report(message: impl Into<String>) -> VerdictReport {
    VerdictReport {
        schema_version: VERDICT_SCHEMA_VERSION,
        baseline_digest: None,
        target: None,
        verdict: Verdict::Error,
        policy: None,
        findings: Vec::new(),
        error: Some(message.into()),
    }
}

fn decide_finding(
    change: ChangeKind,
    effect: &CanonicalEffect,
    evidence: FindingEvidence,
    policy: &Policy,
) -> FindingDecision {
    let effect_kind = effect_kind(effect);
    let mut matches = policy
        .rules
        .iter()
        .filter(|rule| rule.matcher.matches(change, effect, effect_kind))
        .map(|rule| (rule.action, rule.id.clone()))
        .collect::<Vec<_>>();

    matches.sort_by(|left, right| left.1.cmp(&right.1));
    let action = matches
        .iter()
        .map(|(action, _)| *action)
        .max()
        .unwrap_or(policy.default_action);
    let matched_rules = matches.into_iter().map(|(_, id)| id).collect();

    FindingDecision {
        change,
        effect_kind,
        action,
        matched_rules,
        evidence,
    }
}

impl RuleMatcher {
    fn is_empty(&self) -> bool {
        self.change.is_none()
            && self.effect.is_none()
            && self.path_class.is_none()
            && self.path_prefix.is_none()
            && self.executable_family.is_none()
            && self.network_ip.is_none()
            && self.network_port.is_none()
    }

    fn matches(&self, change: ChangeKind, effect: &CanonicalEffect, kind: EffectKind) -> bool {
        if self.change.is_some_and(|expected| expected != change) {
            return false;
        }
        if self.effect.is_some_and(|expected| expected != kind) {
            return false;
        }

        let metadata = EffectMetadata::from_effect(effect);

        if self
            .path_class
            .is_some_and(|expected| metadata.path_class != Some(expected))
        {
            return false;
        }
        if let Some(prefix) = &self.path_prefix {
            let Some(path) = metadata.path else {
                return false;
            };
            if !path_prefix_matches(path, prefix) {
                return false;
            }
        }
        if let Some(expected) = &self.executable_family {
            if metadata.executable_family != Some(expected.as_str()) {
                return false;
            }
        }
        if let Some(expected) = &self.network_ip {
            if metadata.network_ip != Some(expected.as_str()) {
                return false;
            }
        }
        if self
            .network_port
            .is_some_and(|expected| metadata.network_port != Some(expected))
        {
            return false;
        }

        true
    }
}

struct EffectMetadata<'a> {
    path: Option<&'a str>,
    path_class: Option<PathClass>,
    executable_family: Option<&'a str>,
    network_ip: Option<&'a str>,
    network_port: Option<u16>,
}

impl<'a> EffectMetadata<'a> {
    fn from_effect(effect: &'a CanonicalEffect) -> Self {
        match effect {
            CanonicalEffect::ProcessSpawn { actor, .. } => Self {
                path: actor.as_ref().map(|actor| actor.path.value.as_str()),
                path_class: actor.as_ref().map(|actor| actor.path.class),
                executable_family: actor.as_ref().map(|actor| actor.family.as_str()),
                network_ip: None,
                network_port: None,
            },
            CanonicalEffect::ProcessExec { executable, .. } => Self {
                path: Some(executable.path.value.as_str()),
                path_class: Some(executable.path.class),
                executable_family: Some(executable.family.as_str()),
                network_ip: None,
                network_port: None,
            },
            CanonicalEffect::FilePathAccess { actor, target, .. } => Self {
                path: Some(target.value.as_str()),
                path_class: Some(target.class),
                executable_family: actor.as_ref().map(|actor| actor.family.as_str()),
                network_ip: None,
                network_port: None,
            },
            CanonicalEffect::FileRename { actor, to, .. } => Self {
                path: Some(to.value.as_str()),
                path_class: Some(to.class),
                executable_family: actor.as_ref().map(|actor| actor.family.as_str()),
                network_ip: None,
                network_port: None,
            },
            CanonicalEffect::NetworkConnectAttempt { actor, endpoint } => match endpoint {
                CanonicalNetworkEndpoint::Inet { ip, port }
                | CanonicalNetworkEndpoint::Inet6 { ip, port } => Self {
                    path: None,
                    path_class: None,
                    executable_family: actor.as_ref().map(|actor| actor.family.as_str()),
                    network_ip: Some(ip.as_str()),
                    network_port: Some(*port),
                },
                CanonicalNetworkEndpoint::Unix { path } => Self {
                    path: path.as_ref().map(|path| path.value.as_str()),
                    path_class: path.as_ref().map(|path| path.class),
                    executable_family: actor.as_ref().map(|actor| actor.family.as_str()),
                    network_ip: None,
                    network_port: None,
                },
                CanonicalNetworkEndpoint::Other { .. } => Self {
                    path: None,
                    path_class: None,
                    executable_family: actor.as_ref().map(|actor| actor.family.as_str()),
                    network_ip: None,
                    network_port: None,
                },
            },
        }
    }
}

fn effect_kind(effect: &CanonicalEffect) -> EffectKind {
    match effect {
        CanonicalEffect::ProcessSpawn { .. } => EffectKind::ProcessSpawn,
        CanonicalEffect::ProcessExec { .. } => EffectKind::ProcessExec,
        CanonicalEffect::FilePathAccess { operation, .. } => match operation {
            FileOperation::Open => EffectKind::FileOpen,
            FileOperation::Create => EffectKind::FileCreate,
            FileOperation::Delete => EffectKind::FileDelete,
        },
        CanonicalEffect::FileRename { .. } => EffectKind::FileRename,
        CanonicalEffect::NetworkConnectAttempt { .. } => EffectKind::NetworkConnect,
    }
}

fn path_prefix_matches(path: &str, prefix: &str) -> bool {
    if path == prefix {
        return true;
    }
    if prefix.ends_with('/') {
        return path.starts_with(prefix);
    }
    path.strip_prefix(prefix)
        .is_some_and(|suffix| suffix.starts_with('/'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use execsurface_diff::{ChangedEffect, DiffReport, EffectSubject};
    use execsurface_model::canonical::{
        CanonicalExecutable, CanonicalPath, OpenIntent, PathResolution,
    };

    fn executable(name: &str) -> CanonicalExecutable {
        CanonicalExecutable {
            path: CanonicalPath {
                value: format!("/usr/bin/{name}"),
                class: PathClass::System,
                resolution: PathResolution::Lexical,
            },
            family: name.to_owned(),
        }
    }

    fn file_effect(path: &str) -> CanonicalEffect {
        CanonicalEffect::FilePathAccess {
            actor: Some(executable("demo")),
            operation: FileOperation::Open,
            target: CanonicalPath {
                value: path.to_owned(),
                class: PathClass::Workspace,
                resolution: PathResolution::Lexical,
            },
            open_intent: Some(OpenIntent {
                read: true,
                write: false,
                create: false,
                truncate: false,
                append: false,
                path_only: false,
                other_flags: 0,
            }),
        }
    }

    fn diff_with_added(effect: CanonicalEffect) -> DiffReport {
        DiffReport {
            schema_version: 1,
            baseline_digest: "sha256:test".to_owned(),
            target: TargetOutcome {
                exit_code: Some(0),
                signal: None,
            },
            added: vec![effect],
            removed: vec![],
            changed: vec![],
        }
    }

    #[test]
    fn no_drift_is_pass_even_with_block_default() {
        let diff = DiffReport {
            schema_version: 1,
            baseline_digest: "sha256:test".to_owned(),
            target: TargetOutcome {
                exit_code: Some(1),
                signal: None,
            },
            added: vec![],
            removed: vec![],
            changed: vec![],
        };
        let policy = Policy {
            schema_version: 1,
            default_action: FindingAction::Block,
            rules: vec![],
        };
        let report = evaluate(&diff, &policy, "test").unwrap();
        assert_eq!(report.verdict, Verdict::Pass);
    }

    #[test]
    fn builtin_policy_reviews_unmatched_drift() {
        let report = evaluate(
            &diff_with_added(file_effect("$WORKSPACE/file")),
            &builtin_review_policy(),
            "builtin",
        )
        .unwrap();
        assert_eq!(report.verdict, Verdict::Review);
    }

    #[test]
    fn allow_rule_can_accept_matching_drift() {
        let policy = Policy {
            schema_version: 1,
            default_action: FindingAction::Review,
            rules: vec![PolicyRule {
                id: "allow-workspace".to_owned(),
                action: FindingAction::Allow,
                matcher: RuleMatcher {
                    change: Some(ChangeKind::Added),
                    effect: Some(EffectKind::FileOpen),
                    path_prefix: Some("$WORKSPACE/fixtures".to_owned()),
                    ..RuleMatcher::default()
                },
            }],
        };
        let report = evaluate(
            &diff_with_added(file_effect("$WORKSPACE/fixtures/a.txt")),
            &policy,
            "test",
        )
        .unwrap();
        assert_eq!(report.verdict, Verdict::Pass);
        assert_eq!(report.findings[0].action, FindingAction::Allow);
    }

    #[test]
    fn most_restrictive_matching_rule_wins_independent_of_order() {
        let allow = PolicyRule {
            id: "a-allow".to_owned(),
            action: FindingAction::Allow,
            matcher: RuleMatcher {
                effect: Some(EffectKind::FileOpen),
                ..RuleMatcher::default()
            },
        };
        let block = PolicyRule {
            id: "z-block".to_owned(),
            action: FindingAction::Block,
            matcher: RuleMatcher {
                effect: Some(EffectKind::FileOpen),
                ..RuleMatcher::default()
            },
        };
        for rules in [
            vec![allow.clone(), block.clone()],
            vec![block.clone(), allow.clone()],
        ] {
            let policy = Policy {
                schema_version: 1,
                default_action: FindingAction::Review,
                rules,
            };
            let report = evaluate(
                &diff_with_added(file_effect("$WORKSPACE/a")),
                &policy,
                "test",
            )
            .unwrap();
            assert_eq!(report.verdict, Verdict::Block);
            assert_eq!(
                report.findings[0].matched_rules,
                vec!["a-allow".to_owned(), "z-block".to_owned()]
            );
        }
    }

    #[test]
    fn path_prefix_uses_component_boundary() {
        let policy = Policy {
            schema_version: 1,
            default_action: FindingAction::Review,
            rules: vec![PolicyRule {
                id: "allow-foo".to_owned(),
                action: FindingAction::Allow,
                matcher: RuleMatcher {
                    path_prefix: Some("$WORKSPACE/foo".to_owned()),
                    ..RuleMatcher::default()
                },
            }],
        };
        let allowed = evaluate(
            &diff_with_added(file_effect("$WORKSPACE/foo/a")),
            &policy,
            "test",
        )
        .unwrap();
        assert_eq!(allowed.verdict, Verdict::Pass);

        let boundary = evaluate(
            &diff_with_added(file_effect("$WORKSPACE/foobar/a")),
            &policy,
            "test",
        )
        .unwrap();
        assert_eq!(boundary.verdict, Verdict::Review);
    }

    #[test]
    fn duplicate_rule_ids_are_rejected() {
        let rule = PolicyRule {
            id: "same".to_owned(),
            action: FindingAction::Allow,
            matcher: RuleMatcher {
                effect: Some(EffectKind::FileOpen),
                ..RuleMatcher::default()
            },
        };
        let policy = Policy {
            schema_version: 1,
            default_action: FindingAction::Review,
            rules: vec![rule.clone(), rule],
        };
        assert_eq!(
            validate_policy(&policy),
            Err(PolicyError::DuplicateRuleId("same".to_owned()))
        );
    }

    #[test]
    fn empty_matcher_is_rejected() {
        let policy = Policy {
            schema_version: 1,
            default_action: FindingAction::Review,
            rules: vec![PolicyRule {
                id: "bad".to_owned(),
                action: FindingAction::Block,
                matcher: RuleMatcher::default(),
            }],
        };
        assert_eq!(
            validate_policy(&policy),
            Err(PolicyError::EmptyMatcher("bad".to_owned()))
        );
    }

    #[test]
    fn changed_rule_matches_after_state() {
        let before = CanonicalEffect::NetworkConnectAttempt {
            actor: Some(executable("demo")),
            endpoint: CanonicalNetworkEndpoint::Inet {
                ip: "192.0.2.1".to_owned(),
                port: 443,
            },
        };
        let after = CanonicalEffect::NetworkConnectAttempt {
            actor: Some(executable("demo")),
            endpoint: CanonicalNetworkEndpoint::Inet {
                ip: "192.0.2.1".to_owned(),
                port: 8443,
            },
        };
        let diff = DiffReport {
            schema_version: 1,
            baseline_digest: "sha256:test".to_owned(),
            target: TargetOutcome {
                exit_code: Some(0),
                signal: None,
            },
            added: vec![],
            removed: vec![],
            changed: vec![ChangedEffect {
                subject: EffectSubject::NetworkInet {
                    actor: Some(executable("demo")),
                    address_family: "inet".to_owned(),
                    ip: "192.0.2.1".to_owned(),
                },
                before,
                after,
            }],
        };
        let policy = Policy {
            schema_version: 1,
            default_action: FindingAction::Review,
            rules: vec![PolicyRule {
                id: "block-alt-port".to_owned(),
                action: FindingAction::Block,
                matcher: RuleMatcher {
                    change: Some(ChangeKind::Changed),
                    effect: Some(EffectKind::NetworkConnect),
                    network_port: Some(8443),
                    ..RuleMatcher::default()
                },
            }],
        };
        assert_eq!(
            evaluate(&diff, &policy, "test").unwrap().verdict,
            Verdict::Block
        );
    }

    #[test]
    fn error_report_is_explicit_error_state() {
        let report = error_report("observer failed");
        assert_eq!(report.verdict, Verdict::Error);
        assert_eq!(report.error.as_deref(), Some("observer failed"));
    }
}
