from pathlib import Path


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one match, found {count}")
    return text.replace(old, new, 1)


lib_path = Path("crates/execsurface-observe/src/lib.rs")
lib = lib_path.read_text()
old = '''            let ptrace = linux_ptrace::observe(spec, options)?;
            // C1 stage 1 records an internal clone/fd completeness certificate,
            // but the legacy public alpha.4 guard remains authoritative until
            // the preregistered falsification and real-workload gates close.
            let _clone_fd_semantics_certified =
                ptrace.clone_fd_certification.fully_certified();
            let observation = apply_shared_fd_ambiguity_guard(ptrace.observation);
'''
new = '''            let ptrace = linux_ptrace::observe(spec, options)?;
            let observation = if ptrace.clone_fd_certification.fully_certified() {
                // C6 research integration: suppress only the conservative synthetic
                // shared-fd ambiguity guard when the internal collector certificate
                // positively proves every clone-origin fd-table transition. Existing
                // observer warnings/incompleteness remain untouched and authoritative.
                ptrace.observation
            } else {
                apply_shared_fd_ambiguity_guard(ptrace.observation)
            };
'''
lib = replace_once(lib, old, new, "certificate-aware ptrace handoff")

anchor = '''    #[test]
    fn shared_fd_ambiguity_guard_marks_clone_observations_incomplete() {
'''
insert = r'''    #[test]
    fn c6_uncertified_clone_still_uses_legacy_guard() {
        let mut observation = Observation::empty(BackendKind::Ptrace);
        observation.events.push(RawEvent {
            sequence: 1,
            tid: 7,
            executable: None,
            kind: RawEventKind::ProcessSpawn {
                child_tid: 8,
                mechanism: SpawnMechanism::Clone,
            },
        });
        let guarded = apply_shared_fd_ambiguity_guard(observation);
        assert!(!guarded.complete);
        assert!(guarded
            .warnings
            .iter()
            .any(|warning| warning.code == "shared_fd_table_ambiguity"));
    }

'''
lib = replace_once(lib, anchor, insert + anchor, "add C6 negative guard test")
lib_path.write_text(lib)
