from pathlib import Path


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one match, found {count}")
    return text.replace(old, new, 1)


path = Path("crates/execsurface-observe/src/lib.rs")
text = path.read_text()

anchor = '''fn classify_observation_completeness(observation: &Observation) -> CollectionCompleteness {\n'''
insert = '''#[derive(Debug, Clone, Copy, PartialEq, Eq)]\nenum PtraceSharedFdGuardPolicy {\n    LegacyConservative,\n    #[cfg(test)]\n    CertificateAwareResearch,\n}\n\nfn finalize_ptrace_observation(\n    observation: Observation,\n    _clone_fd_semantics_certified: bool,\n    policy: PtraceSharedFdGuardPolicy,\n) -> Observation {\n    match policy {\n        PtraceSharedFdGuardPolicy::LegacyConservative => {\n            apply_shared_fd_ambiguity_guard(observation)\n        }\n        #[cfg(test)]\n        PtraceSharedFdGuardPolicy::CertificateAwareResearch => {\n            if _clone_fd_semantics_certified {\n                observation\n            } else {\n                apply_shared_fd_ambiguity_guard(observation)\n            }\n        }\n    }\n}\n\n'''
text = replace_once(text, anchor, insert + anchor, "insert isolated guard policy")

old = '''            let ptrace = linux_ptrace::observe(spec, options)?;\n            // C1 stage 1 records an internal clone/fd completeness certificate,\n            // but the legacy public alpha.4 guard remains authoritative until\n            // the preregistered falsification and real-workload gates close.\n            let _clone_fd_semantics_certified = ptrace.clone_fd_certification.fully_certified();\n            let observation = apply_shared_fd_ambiguity_guard(ptrace.observation);\n'''
new = '''            let ptrace = linux_ptrace::observe(spec, options)?;\n            // Public/default behavior remains the accepted alpha.4/v2 contract.\n            // The certificate-aware mode is compiled only for research tests and\n            // cannot be selected by observe_command or the default backend.\n            let clone_fd_semantics_certified = ptrace.clone_fd_certification.fully_certified();\n            let observation = finalize_ptrace_observation(\n                ptrace.observation,\n                clone_fd_semantics_certified,\n                PtraceSharedFdGuardPolicy::LegacyConservative,\n            );\n'''
text = replace_once(text, old, new, "keep public backend legacy")

anchor_test = '''    #[test]\n    fn shared_fd_ambiguity_is_never_pass_eligible() {\n'''
insert_test = r'''    fn c6r_clone_observation() -> Observation {
        let mut observation = Observation::empty(execsurface_model::BackendMetadata {
            name: "linux-ptrace-metadata-v2".to_owned(),
            platform: "linux".to_owned(),
            architecture: "x86_64".to_owned(),
            capabilities: Vec::new(),
            limitations: Vec::new(),
        });
        observation.events.push(execsurface_model::RawEvent {
            sequence: 1,
            tid: 7,
            kind: execsurface_model::RawEventKind::ProcessSpawn {
                child_tid: 8,
                mechanism: execsurface_model::SpawnMechanism::Clone,
            },
        });
        observation
    }

    #[test]
    fn c6r_default_policy_remains_legacy_even_with_positive_certificate() {
        let finalized = finalize_ptrace_observation(
            c6r_clone_observation(),
            true,
            PtraceSharedFdGuardPolicy::LegacyConservative,
        );
        assert!(!finalized.complete);
        assert!(finalized
            .warnings
            .iter()
            .any(|warning| warning.code == "shared_fd_table_ambiguity"));
    }

    #[test]
    fn c6r_uncertified_research_mode_remains_fail_closed() {
        let finalized = finalize_ptrace_observation(
            c6r_clone_observation(),
            false,
            PtraceSharedFdGuardPolicy::CertificateAwareResearch,
        );
        assert!(!finalized.complete);
        assert!(finalized
            .warnings
            .iter()
            .any(|warning| warning.code == "shared_fd_table_ambiguity"));
    }

    #[test]
    fn c6r_certified_research_mode_skips_only_synthetic_clone_guard() {
        let finalized = finalize_ptrace_observation(
            c6r_clone_observation(),
            true,
            PtraceSharedFdGuardPolicy::CertificateAwareResearch,
        );
        assert!(finalized.complete);
        assert!(finalized.warnings.is_empty());
    }

    #[test]
    fn c6r_certificate_never_clears_independent_incompleteness() {
        let mut observation = c6r_clone_observation();
        observation.complete = false;
        observation.warnings.push(ObserverWarning {
            code: "event_limit_exceeded".to_owned(),
            tid: None,
            message: "controlled independent blocker".to_owned(),
        });
        let finalized = finalize_ptrace_observation(
            observation,
            true,
            PtraceSharedFdGuardPolicy::CertificateAwareResearch,
        );
        assert!(!finalized.complete);
        assert_eq!(finalized.warnings.len(), 1);
        assert_eq!(finalized.warnings[0].code, "event_limit_exceeded");
    }

'''
text = replace_once(text, anchor_test, insert_test + anchor_test, "insert C6R policy tests")
path.write_text(text)
