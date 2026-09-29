from pathlib import Path


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one match, found {count}")
    return text.replace(old, new, 1)


linux_path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
linux = linux_path.read_text()

linux = replace_once(
    linux,
    """struct Collector {\n    observation: Observation,\n    sequence: u64,\n    event_limit: usize,\n    event_limit_reported: bool,\n}\n\nimpl Collector {\n""",
    """#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]\npub(super) struct CloneFdCertification {\n    clone_events: u64,\n    correlated_clone_flags: u64,\n    shared_fd_transitions: u64,\n    cloned_fd_transitions: u64,\n    clone_thread_transitions: u64,\n    ambiguity: bool,\n}\n\nimpl CloneFdCertification {\n    fn record_clone(&mut self, flags: Option<u64>) {\n        self.clone_events += 1;\n        let Some(flags) = flags else {\n            self.ambiguity = true;\n            return;\n        };\n\n        self.correlated_clone_flags += 1;\n        if flags & libc::CLONE_FILES as u64 != 0 {\n            self.shared_fd_transitions += 1;\n        } else {\n            self.cloned_fd_transitions += 1;\n        }\n        if flags & libc::CLONE_THREAD as u64 != 0 {\n            self.clone_thread_transitions += 1;\n        }\n    }\n\n    fn invalidate(&mut self) {\n        self.ambiguity = true;\n    }\n\n    pub(super) fn fully_certified(&self) -> bool {\n        !self.ambiguity\n            && self.clone_events == self.correlated_clone_flags\n            && self.clone_events == self.shared_fd_transitions + self.cloned_fd_transitions\n    }\n}\n\n#[derive(Debug, Clone, PartialEq, Eq)]\npub(super) struct PtraceObservation {\n    pub(super) observation: Observation,\n    pub(super) clone_fd_certification: CloneFdCertification,\n}\n\nstruct Collector {\n    observation: Observation,\n    sequence: u64,\n    event_limit: usize,\n    event_limit_reported: bool,\n    clone_fd_certification: CloneFdCertification,\n}\n\nimpl Collector {\n""",
    "insert certification types",
)

linux = replace_once(
    linux,
    """        Self {\n            observation: Observation::empty(backend),\n            sequence: 0,\n            event_limit,\n            event_limit_reported: false,\n        }\n""",
    """        Self {\n            observation: Observation::empty(backend),\n            sequence: 0,\n            event_limit,\n            event_limit_reported: false,\n            clone_fd_certification: CloneFdCertification::default(),\n        }\n""",
    "initialize certification",
)

linux = replace_once(
    linux,
    """    fn warning(&mut self, tid: libc::pid_t, code: &str, message: impl Into<String>) {\n        self.observation.complete = false;\n        self.observation.warnings.push(ObserverWarning {\n""",
    """    fn warning(&mut self, tid: libc::pid_t, code: &str, message: impl Into<String>) {\n        self.observation.complete = false;\n        if matches!(\n            code,\n            \"clone_flags_unavailable\" | \"clone3_flags_unreadable\" | \"syscall_pairing_lost\"\n        ) {\n            self.clone_fd_certification.invalidate();\n        }\n        self.observation.warnings.push(ObserverWarning {\n""",
    "invalidate certification on lifecycle ambiguity",
)

linux = replace_once(
    linux,
    """pub(super) fn observe(\n    spec: &CommandSpec,\n    options: ObserveOptions,\n) -> Result<Observation, ObserveError> {\n""",
    """pub(super) fn observe(\n    spec: &CommandSpec,\n    options: ObserveOptions,\n) -> Result<PtraceObservation, ObserveError> {\n""",
    "observe return type",
)

linux = replace_once(
    linux,
    """fn trace_parent(root: libc::pid_t, options: ObserveOptions) -> Result<Observation, ObserveError> {\n""",
    """fn trace_parent(\n    root: libc::pid_t,\n    options: ObserveOptions,\n) -> Result<PtraceObservation, ObserveError> {\n""",
    "trace_parent return type",
)

linux = replace_once(
    linux,
    """    collector.observation.outcome = root_outcome;\n    Ok(collector.observation)\n}\n""",
    """    collector.observation.outcome = root_outcome;\n    Ok(PtraceObservation {\n        observation: collector.observation,\n        clone_fd_certification: collector.clone_fd_certification,\n    })\n}\n""",
    "return certification envelope",
)

linux = replace_once(
    linux,
    """            let share_files = clone_flags\n                .map(|flags| flags & libc::CLONE_FILES as u64 != 0)\n                .unwrap_or(false);\n""",
    """            if event == libc::PTRACE_EVENT_CLONE {\n                collector.clone_fd_certification.record_clone(clone_flags);\n            }\n\n            let share_files = clone_flags\n                .map(|flags| flags & libc::CLONE_FILES as u64 != 0)\n                .unwrap_or(false);\n""",
    "record clone certificate evidence",
)

linux = replace_once(
    linux,
    """    #[test]\n    fn normalizes_only_the_tracees_own_proc_identity() {\n""",
    """    #[test]\n    fn c1_a_clone_free_certificate_is_trivially_exact() {\n        let certificate = CloneFdCertification::default();\n        assert!(certificate.fully_certified());\n        assert_eq!(certificate.clone_events, 0);\n        assert_eq!(certificate.shared_fd_transitions, 0);\n        assert_eq!(certificate.cloned_fd_transitions, 0);\n    }\n\n    #[test]\n    fn c1_b_private_clone_is_classified_exactly_once() {\n        let mut certificate = CloneFdCertification::default();\n        certificate.record_clone(Some(0));\n        assert!(certificate.fully_certified());\n        assert_eq!(certificate.clone_events, 1);\n        assert_eq!(certificate.correlated_clone_flags, 1);\n        assert_eq!(certificate.shared_fd_transitions, 0);\n        assert_eq!(certificate.cloned_fd_transitions, 1);\n        assert_eq!(certificate.clone_thread_transitions, 0);\n    }\n\n    #[test]\n    fn c1_cde_shared_and_thread_bits_remain_independent() {\n        let mut shared = CloneFdCertification::default();\n        shared.record_clone(Some(libc::CLONE_FILES as u64));\n        assert!(shared.fully_certified());\n        assert_eq!(shared.shared_fd_transitions, 1);\n        assert_eq!(shared.clone_thread_transitions, 0);\n\n        let mut thread_only = CloneFdCertification::default();\n        thread_only.record_clone(Some(libc::CLONE_THREAD as u64));\n        assert!(thread_only.fully_certified());\n        assert_eq!(thread_only.shared_fd_transitions, 0);\n        assert_eq!(thread_only.cloned_fd_transitions, 1);\n        assert_eq!(thread_only.clone_thread_transitions, 1);\n\n        let mut both = CloneFdCertification::default();\n        both.record_clone(Some((libc::CLONE_THREAD | libc::CLONE_FILES) as u64));\n        assert!(both.fully_certified());\n        assert_eq!(both.shared_fd_transitions, 1);\n        assert_eq!(both.clone_thread_transitions, 1);\n    }\n\n    #[test]\n    fn c1_f_missing_clone_flags_fail_certification() {\n        let mut certificate = CloneFdCertification::default();\n        certificate.record_clone(None);\n        assert!(!certificate.fully_certified());\n        assert_eq!(certificate.clone_events, 1);\n        assert_eq!(certificate.correlated_clone_flags, 0);\n    }\n\n    #[test]\n    fn c1_g_clone3_unreadable_warning_invalidates_certification() {\n        let mut collector = Collector::new(128);\n        collector.warning(42, \"clone3_flags_unreadable\", \"controlled test\");\n        assert!(!collector.clone_fd_certification.fully_certified());\n        assert!(!collector.observation.complete);\n    }\n\n    #[test]\n    fn c1_i_exec_cloexec_unshares_from_other_shared_table_users() {\n        let mut tables = FdTables::new();\n        let shared_id = tables.root_id();\n        tables.insert_fd(\n            shared_id,\n            7,\n            FdEntry {\n                path: \"/tmp/c1-cloexec\".to_owned(),\n                cloexec: true,\n            },\n        );\n\n        let mut tracees = HashMap::new();\n        tracees.insert(100, TraceeState::root(shared_id, 100));\n        tracees.insert(101, TraceeState::child(shared_id, 100));\n        apply_exec_fd_semantics(101, &mut tracees, &mut tables);\n\n        let exec_table = tracees.get(&101).expect(\"execing task\").fd_table_id;\n        assert_ne!(exec_table, shared_id);\n        assert!(tables.fd(exec_table, 7).is_none());\n        assert!(tables.fd(shared_id, 7).is_some());\n    }\n\n    #[test]\n    fn c1_jk_shared_mutation_and_fd_reuse_never_leave_stale_identity() {\n        let mut tables = FdTables::new();\n        let shared_id = tables.root_id();\n        tables.insert_fd(\n            shared_id,\n            9,\n            FdEntry {\n                path: \"/tmp/c1-old\".to_owned(),\n                cloexec: false,\n            },\n        );\n        tables.duplicate(shared_id, 9, 10, false);\n        assert_eq!(tables.fd(shared_id, 10).expect(\"dup fd\").path, \"/tmp/c1-old\");\n\n        tables.remove_fd(shared_id, 9);\n        tables.insert_fd(\n            shared_id,\n            9,\n            FdEntry {\n                path: \"/tmp/c1-new\".to_owned(),\n                cloexec: false,\n            },\n        );\n        assert_eq!(tables.fd(shared_id, 9).expect(\"reused fd\").path, \"/tmp/c1-new\");\n        assert_eq!(tables.fd(shared_id, 10).expect(\"old dup remains\").path, \"/tmp/c1-old\");\n\n        tables.close_range(shared_id, 9, 10);\n        assert!(tables.fd(shared_id, 9).is_none());\n        assert!(tables.fd(shared_id, 10).is_none());\n    }\n\n    #[test]\n    fn normalizes_only_the_tracees_own_proc_identity() {\n""",
    "insert C1 unit/state-machine tests",
)

linux_path.write_text(linux)

lib_path = Path("crates/execsurface-observe/src/lib.rs")
lib = lib_path.read_text()
lib = replace_once(
    lib,
    """            let observation =\n                apply_shared_fd_ambiguity_guard(linux_ptrace::observe(spec, options)?);\n""",
    """            let ptrace = linux_ptrace::observe(spec, options)?;\n            // C1 stage 1 records an internal clone/fd completeness certificate,\n            // but the legacy public alpha.4 guard remains authoritative until\n            // the preregistered falsification and real-workload gates close.\n            let _clone_fd_semantics_certified =\n                ptrace.clone_fd_certification.fully_certified();\n            let observation = apply_shared_fd_ambiguity_guard(ptrace.observation);\n""",
    "consume internal ptrace envelope without public semantic change",
)
lib_path.write_text(lib)
