#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use execsurface_p4_backend_authority::b0_success_evidence::{
    ActorIdentity, AttemptEvidence, EvidenceLedger, ExitEvidence, ObservationHealth, OperationKind,
    TargetProposition,
};
use execsurface_p4_backend_authority::b2_rename_delete::{
    RenameDeleteAuthority, RenameDeleteContext, RenameDeleteRecord,
};
use sha2::{Digest, Sha256};

static NEXT_TMP: AtomicU64 = AtomicU64::new(1);
const SYSCALL_STOP: i32 = libc::SIGTRAP | 0x80;
const RENAME_NOREPLACE: u32 = 1;
const RENAME_EXCHANGE: u32 = 2;
const RENAME_WHITEOUT: u32 = 4;

struct TempDir(PathBuf);

impl TempDir {
    fn new(label: &str) -> Self {
        let id = NEXT_TMP.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "execsurface-p4-b2-{}-{}-{id}",
            std::process::id(),
            label
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("create temp dir");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Debug, Clone)]
struct PendingEffect {
    context: RenameDeleteContext,
    entry_sequence: u64,
}

#[derive(Debug, Clone)]
struct LiveEffect {
    pid: libc::pid_t,
    context: RenameDeleteContext,
    entry_sequence: u64,
    exit_sequence: u64,
    raw_return: i64,
}

impl LiveEffect {
    fn actor(&self) -> ActorIdentity {
        let process_identity = format!("pid:{}", self.pid);
        ActorIdentity {
            tid: self.pid,
            causal_chain_digest: format!(
                "sha256:{:x}",
                Sha256::digest(process_identity.as_bytes())
            ),
            process_identity,
        }
    }

    fn evidence(
        &self,
    ) -> execsurface_p4_backend_authority::b0_success_evidence::SuccessEvidenceRecord {
        let actor = self.actor();
        let attempt = AttemptEvidence {
            proposition: TargetProposition::FileRenameDelete,
            operation: self.context.operation(),
            actor: actor.clone(),
            entry_sequence: self.entry_sequence,
            argument_digest: self.context.digest().expect("live context digest"),
            target_identity: self.context.target_identity(),
        };
        EvidenceLedger::default()
            .classify_pair(
                attempt,
                ExitEvidence {
                    actor,
                    originating_entry_sequence: self.entry_sequence,
                    exit_sequence: self.exit_sequence,
                    raw_return: self.raw_return,
                },
                ObservationHealth::healthy(),
            )
            .expect("live B0 result evidence")
    }

    fn record(&self) -> RenameDeleteRecord {
        RenameDeleteRecord::build(self.evidence(), self.context.clone())
            .expect("live B2 authority record")
    }
}

struct TraceOutcome {
    events: Vec<LiveEffect>,
    exit_code: i32,
}

fn fixture() -> &'static str {
    env!("CARGO_BIN_EXE_b2-rename-delete-fixture")
}

fn ptrace_resume(pid: libc::pid_t) {
    let rc = unsafe {
        libc::ptrace(
            libc::PTRACE_SYSCALL,
            pid,
            std::ptr::null_mut::<libc::c_void>(),
            std::ptr::null_mut::<libc::c_void>(),
        )
    };
    assert_eq!(rc, 0, "PTRACE_SYSCALL: {}", std::io::Error::last_os_error());
}

fn ptrace_regs(pid: libc::pid_t) -> libc::user_regs_struct {
    let mut regs = unsafe { std::mem::zeroed::<libc::user_regs_struct>() };
    let rc = unsafe {
        libc::ptrace(
            libc::PTRACE_GETREGS,
            pid,
            std::ptr::null_mut::<libc::c_void>(),
            &mut regs as *mut libc::user_regs_struct as *mut libc::c_void,
        )
    };
    assert_eq!(rc, 0, "PTRACE_GETREGS: {}", std::io::Error::last_os_error());
    regs
}

fn read_tracee_c_string(pid: libc::pid_t, address: u64) -> Result<String, String> {
    if address == 0 {
        return Err("null_path_pointer".to_owned());
    }
    let word_size = std::mem::size_of::<libc::c_long>();
    let mut bytes = Vec::new();
    for offset in (0..4096usize).step_by(word_size) {
        let word = unsafe {
            libc::ptrace(
                libc::PTRACE_PEEKDATA,
                pid,
                (address as usize + offset) as *mut libc::c_void,
                std::ptr::null_mut::<libc::c_void>(),
            )
        };
        if word == -1 && bytes.is_empty() {
            return Err(format!(
                "peekdata_failed: {}",
                std::io::Error::last_os_error()
            ));
        }
        for byte in word.to_ne_bytes() {
            if byte == 0 {
                return String::from_utf8(bytes).map_err(|_| "path_not_utf8".to_owned());
            }
            bytes.push(byte);
            if bytes.len() == 4096 {
                return Err("path_exceeds_4096".to_owned());
            }
        }
    }
    Err("unterminated_path".to_owned())
}

fn signed_i32_syscall_arg(value: u64) -> i64 {
    i64::from(value as u32 as i32)
}

fn known_rename_flags(flags: u32) -> bool {
    flags & !(RENAME_NOREPLACE | RENAME_EXCHANGE | RENAME_WHITEOUT) == 0
}

fn known_unlinkat_flags(flags: u32) -> bool {
    flags == 0 || flags == libc::AT_REMOVEDIR as u32
}

fn decode_entry(
    pid: libc::pid_t,
    regs: &libc::user_regs_struct,
) -> Result<Option<RenameDeleteContext>, String> {
    let nr = regs.orig_rax as libc::c_long;
    let context = if nr == libc::SYS_rename {
        Some(RenameDeleteContext::Rename {
            operation: OperationKind::Rename,
            source: read_tracee_c_string(pid, regs.rdi)?,
            target: read_tracee_c_string(pid, regs.rsi)?,
            source_dirfd: None,
            target_dirfd: None,
            flags: None,
            flags_classified: true,
        })
    } else if nr == libc::SYS_renameat {
        Some(RenameDeleteContext::Rename {
            operation: OperationKind::RenameAt,
            source: read_tracee_c_string(pid, regs.rsi)?,
            target: read_tracee_c_string(pid, regs.r10)?,
            source_dirfd: Some(signed_i32_syscall_arg(regs.rdi)),
            target_dirfd: Some(signed_i32_syscall_arg(regs.rdx)),
            flags: None,
            flags_classified: true,
        })
    } else if nr == libc::SYS_renameat2 {
        let flags = regs.r8 as u32;
        Some(RenameDeleteContext::Rename {
            operation: OperationKind::RenameAt2,
            source: read_tracee_c_string(pid, regs.rsi)?,
            target: read_tracee_c_string(pid, regs.r10)?,
            source_dirfd: Some(signed_i32_syscall_arg(regs.rdi)),
            target_dirfd: Some(signed_i32_syscall_arg(regs.rdx)),
            flags: Some(flags),
            flags_classified: known_rename_flags(flags),
        })
    } else if nr == libc::SYS_unlink {
        Some(RenameDeleteContext::Delete {
            operation: OperationKind::Unlink,
            target: read_tracee_c_string(pid, regs.rdi)?,
            dirfd: None,
            flags: None,
            flags_classified: true,
        })
    } else if nr == libc::SYS_unlinkat {
        let flags = regs.rdx as u32;
        Some(RenameDeleteContext::Delete {
            operation: OperationKind::UnlinkAt,
            target: read_tracee_c_string(pid, regs.rsi)?,
            dirfd: Some(signed_i32_syscall_arg(regs.rdi)),
            flags: Some(flags),
            flags_classified: known_unlinkat_flags(flags),
        })
    } else if nr == libc::SYS_rmdir {
        Some(RenameDeleteContext::Delete {
            operation: OperationKind::Rmdir,
            target: read_tracee_c_string(pid, regs.rdi)?,
            dirfd: None,
            flags: None,
            flags_classified: true,
        })
    } else {
        None
    };
    Ok(context)
}

fn trace_fixture(scenario: &str, args: &[String]) -> TraceOutcome {
    let mut command = Command::new(fixture());
    command.arg(scenario).args(args);
    let child = command.spawn().expect("spawn B2 ptrace fixture");
    let pid = child.id() as libc::pid_t;

    let mut status = 0;
    assert_eq!(unsafe { libc::waitpid(pid, &mut status, 0) }, pid);
    assert!(libc::WIFSTOPPED(status));
    assert_eq!(libc::WSTOPSIG(status), libc::SIGSTOP);

    let options = libc::PTRACE_O_TRACESYSGOOD as usize as *mut libc::c_void;
    let rc = unsafe {
        libc::ptrace(
            libc::PTRACE_SETOPTIONS,
            pid,
            std::ptr::null_mut::<libc::c_void>(),
            options,
        )
    };
    assert_eq!(
        rc,
        0,
        "PTRACE_SETOPTIONS: {}",
        std::io::Error::last_os_error()
    );

    let mut entering = true;
    let mut sequence = 0u64;
    let mut pending: Option<PendingEffect> = None;
    let mut events = Vec::new();
    ptrace_resume(pid);

    let exit_code = loop {
        status = 0;
        assert_eq!(unsafe { libc::waitpid(pid, &mut status, 0) }, pid);
        if libc::WIFEXITED(status) {
            break libc::WEXITSTATUS(status);
        }
        if libc::WIFSIGNALED(status) {
            panic!("B2 fixture killed by signal {}", libc::WTERMSIG(status));
        }
        assert!(libc::WIFSTOPPED(status));
        if libc::WSTOPSIG(status) == SYSCALL_STOP {
            sequence += 1;
            let regs = ptrace_regs(pid);
            if entering {
                pending = decode_entry(pid, &regs)
                    .expect("decode B2 syscall entry")
                    .map(|context| PendingEffect {
                        context,
                        entry_sequence: sequence,
                    });
            } else if let Some(effect) = pending.take() {
                events.push(LiveEffect {
                    pid,
                    context: effect.context,
                    entry_sequence: effect.entry_sequence,
                    exit_sequence: sequence,
                    raw_return: regs.rax as i64,
                });
            }
            entering = !entering;
        }
        ptrace_resume(pid);
    };

    drop(child);
    TraceOutcome { events, exit_code }
}

fn write_file(path: &Path) {
    fs::write(path, b"b2").expect("write fixture file");
}

fn single_success(outcome: &TraceOutcome) -> &LiveEffect {
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(outcome.events.len(), 1);
    let record = outcome.events[0].record();
    assert!(record.is_success_authority());
    &outcome.events[0]
}

#[test]
fn b2_live_rename_pairs_exact_context_with_rc_zero() {
    let dir = TempDir::new("rename");
    let source = dir.path().join("source");
    let target = dir.path().join("target");
    write_file(&source);
    let outcome = trace_fixture(
        "rename",
        &[
            source.to_string_lossy().into_owned(),
            target.to_string_lossy().into_owned(),
        ],
    );
    let event = single_success(&outcome);
    assert_eq!(event.context.operation(), OperationKind::Rename);
    assert!(!source.exists());
    assert!(target.exists());
}

#[test]
fn b2_live_renameat_binds_real_dirfds_and_relative_paths() {
    let root = TempDir::new("renameat");
    let source_dir = root.path().join("from");
    let target_dir = root.path().join("to");
    fs::create_dir_all(&source_dir).expect("source dir");
    fs::create_dir_all(&target_dir).expect("target dir");
    write_file(&source_dir.join("item"));
    let outcome = trace_fixture(
        "renameat",
        &[
            source_dir.to_string_lossy().into_owned(),
            "item".to_owned(),
            target_dir.to_string_lossy().into_owned(),
            "moved".to_owned(),
        ],
    );
    let event = single_success(&outcome);
    match &event.context {
        RenameDeleteContext::Rename {
            source,
            target,
            source_dirfd,
            target_dirfd,
            ..
        } => {
            assert_eq!(source, "item");
            assert_eq!(target, "moved");
            assert_ne!(*source_dirfd, Some(i64::from(libc::AT_FDCWD)));
            assert_ne!(*target_dirfd, Some(i64::from(libc::AT_FDCWD)));
        }
        _ => panic!("expected rename context"),
    }
    assert!(target_dir.join("moved").exists());
}

#[test]
fn b2_live_renameat2_noreplace_is_success_or_explicit_kernel_unsupported() {
    let dir = TempDir::new("renameat2");
    let source = dir.path().join("source");
    let target = dir.path().join("target");
    write_file(&source);
    let outcome = trace_fixture(
        "renameat2-noreplace",
        &[
            source.to_string_lossy().into_owned(),
            target.to_string_lossy().into_owned(),
        ],
    );
    assert_eq!(outcome.events.len(), 1);
    let event = &outcome.events[0];
    match &event.context {
        RenameDeleteContext::Rename {
            operation,
            flags,
            flags_classified,
            ..
        } => {
            assert_eq!(*operation, OperationKind::RenameAt2);
            assert_eq!(*flags, Some(RENAME_NOREPLACE));
            assert!(*flags_classified);
        }
        _ => panic!("expected renameat2 context"),
    }
    if event.raw_return == -i64::from(libc::ENOSYS) {
        assert_eq!(outcome.exit_code, 45);
        assert!(matches!(
            event.record().authority,
            RenameDeleteAuthority::FailureObserved { errno } if errno == libc::ENOSYS
        ));
    } else {
        assert_eq!(outcome.exit_code, 0);
        assert!(event.record().is_success_authority());
        assert!(target.exists());
    }
}

#[test]
fn b2_live_unlink_pairs_delete_with_rc_zero() {
    let dir = TempDir::new("unlink");
    let target = dir.path().join("delete-me");
    write_file(&target);
    let outcome = trace_fixture("unlink", &[target.to_string_lossy().into_owned()]);
    let event = single_success(&outcome);
    assert_eq!(event.context.operation(), OperationKind::Unlink);
    assert!(!target.exists());
}

#[test]
fn b2_live_unlinkat_pairs_flags_and_rc_zero() {
    let dir = TempDir::new("unlinkat");
    let target = dir.path().join("delete-me");
    write_file(&target);
    let outcome = trace_fixture("unlinkat", &[target.to_string_lossy().into_owned()]);
    let event = single_success(&outcome);
    match &event.context {
        RenameDeleteContext::Delete {
            operation,
            dirfd,
            flags,
            flags_classified,
            ..
        } => {
            assert_eq!(*operation, OperationKind::UnlinkAt);
            assert_eq!(*dirfd, Some(i64::from(libc::AT_FDCWD)));
            assert_eq!(*flags, Some(0));
            assert!(*flags_classified);
        }
        _ => panic!("expected unlinkat context"),
    }
    assert!(!target.exists());
}

#[test]
fn b2_live_rmdir_pairs_delete_with_rc_zero() {
    let root = TempDir::new("rmdir");
    let target = root.path().join("remove-dir");
    fs::create_dir(&target).expect("create directory");
    let outcome = trace_fixture("rmdir", &[target.to_string_lossy().into_owned()]);
    let event = single_success(&outcome);
    assert_eq!(event.context.operation(), OperationKind::Rmdir);
    assert!(!target.exists());
}

#[test]
fn b2_live_missing_rename_remains_failure() {
    let dir = TempDir::new("missing");
    let source = dir.path().join("missing");
    let target = dir.path().join("target");
    let outcome = trace_fixture(
        "missing-rename",
        &[
            source.to_string_lossy().into_owned(),
            target.to_string_lossy().into_owned(),
        ],
    );
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(outcome.events.len(), 1);
    let record = outcome.events[0].record();
    assert!(!record.is_success_authority());
    assert!(matches!(
        record.authority,
        RenameDeleteAuthority::FailureObserved { errno } if errno == libc::ENOENT
    ));
}

#[test]
fn b2_live_source_substitution_cannot_reuse_success_evidence() {
    let dir = TempDir::new("substitution");
    let source = dir.path().join("source");
    let target = dir.path().join("target");
    write_file(&source);
    let outcome = trace_fixture(
        "rename",
        &[
            source.to_string_lossy().into_owned(),
            target.to_string_lossy().into_owned(),
        ],
    );
    let event = single_success(&outcome);
    let evidence = event.evidence();
    let mut forged = event.context.clone();
    if let RenameDeleteContext::Rename { source, .. } = &mut forged {
        *source = "$WORKSPACE/forged-source".to_owned();
    }
    assert!(RenameDeleteRecord::build(evidence, forged).is_err());
}

#[test]
fn b2_live_proof_is_deterministic_for_same_traced_effect() {
    let dir = TempDir::new("deterministic");
    let source = dir.path().join("source");
    let target = dir.path().join("target");
    write_file(&source);
    let outcome = trace_fixture(
        "rename",
        &[
            source.to_string_lossy().into_owned(),
            target.to_string_lossy().into_owned(),
        ],
    );
    let event = single_success(&outcome);
    let first = event.record();
    let second = event.record();
    assert_eq!(first.authority, second.authority);
    assert_eq!(
        serde_json::to_vec(&first).expect("serialize first"),
        serde_json::to_vec(&second).expect("serialize second")
    );
}

#[test]
fn b2_live_success_is_not_inferred_from_postcondition_without_zero_result() {
    let dir = TempDir::new("postcondition");
    let source = dir.path().join("source");
    let target = dir.path().join("target");
    write_file(&target);
    let outcome = trace_fixture(
        "missing-rename",
        &[
            source.to_string_lossy().into_owned(),
            target.to_string_lossy().into_owned(),
        ],
    );
    assert!(target.exists(), "target postcondition intentionally exists");
    let event = &outcome.events[0];
    assert!(event.raw_return < 0);
    assert!(!event.record().is_success_authority());
}
