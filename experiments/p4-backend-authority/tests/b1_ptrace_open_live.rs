#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::Serialize;
use sha2::{Digest, Sha256};

static NEXT_TMP: AtomicU64 = AtomicU64::new(1);
const SYSCALL_STOP: i32 = libc::SIGTRAP | 0x80;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct KernelObjectIdentity {
    dev: u64,
    ino: u64,
    file_type: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
enum LiveAuthority {
    SuccessObjectBounded,
    Failure { errno: i32 },
    Ambiguous { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct LiveOpenEvent {
    operation: String,
    path_argument: String,
    entry_sequence: u64,
    exit_sequence: u64,
    raw_return: i64,
    fd: Option<i32>,
    fd_generation: Option<u64>,
    task_count: usize,
    object: Option<KernelObjectIdentity>,
    object_after_mutation: Option<KernelObjectIdentity>,
    contextual_fd_link: Option<String>,
    authority: LiveAuthority,
}

impl LiveOpenEvent {
    fn proof_digest(&self) -> String {
        let bytes = serde_json::to_vec(self).expect("serialize live evidence");
        format!("sha256:{:x}", Sha256::digest(bytes))
    }

    fn success_object(&self) -> &KernelObjectIdentity {
        assert_eq!(self.authority, LiveAuthority::SuccessObjectBounded);
        self.object.as_ref().expect("successful object identity")
    }
}

struct TempDir(PathBuf);

impl TempDir {
    fn new(label: &str) -> Self {
        let id = NEXT_TMP.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "execsurface-p4-b1-{}-{}-{id}",
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

#[derive(Clone)]
struct RenameMutation {
    from: PathBuf,
    to: PathBuf,
}

#[derive(Debug)]
struct PendingOpen {
    operation: &'static str,
    path_argument: String,
    entry_sequence: u64,
}

fn fixture() -> &'static str {
    env!("CARGO_BIN_EXE_b1-open-fixture")
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
            return Err(format!("peekdata_failed: {}", std::io::Error::last_os_error()));
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

fn open_entry(regs: &libc::user_regs_struct) -> Option<(&'static str, u64)> {
    let nr = regs.orig_rax as libc::c_long;
    if nr == libc::SYS_open {
        Some(("open", regs.rdi))
    } else if nr == libc::SYS_openat {
        Some(("openat", regs.rsi))
    } else if nr == libc::SYS_openat2 {
        Some(("openat2", regs.rsi))
    } else {
        None
    }
}

fn task_count(pid: libc::pid_t) -> Result<usize, String> {
    fs::read_dir(format!("/proc/{pid}/task"))
        .map_err(|error| format!("task_count_failed:{error}"))
        .map(|entries| entries.count())
}

fn object_identity(pid: libc::pid_t, fd: i32) -> Result<(KernelObjectIdentity, Option<String>), String> {
    let fd_path = PathBuf::from(format!("/proc/{pid}/fd/{fd}"));
    let metadata = fs::metadata(&fd_path).map_err(|error| format!("fd_metadata_failed:{error}"))?;
    let identity = KernelObjectIdentity {
        dev: metadata.dev(),
        ino: metadata.ino(),
        file_type: metadata.mode() & libc::S_IFMT,
    };
    let link = fs::read_link(&fd_path)
        .ok()
        .map(|path| path.to_string_lossy().into_owned());
    Ok((identity, link))
}

fn identity_of_path(path: &Path) -> KernelObjectIdentity {
    let metadata = fs::metadata(path).expect("metadata for expected path");
    KernelObjectIdentity {
        dev: metadata.dev(),
        ino: metadata.ino(),
        file_type: metadata.mode() & libc::S_IFMT,
    }
}

fn trace_fixture(
    scenario: &str,
    args: &[String],
    expected_path_arguments: &[String],
    mutation: Option<RenameMutation>,
) -> Vec<LiveOpenEvent> {
    let mut command = Command::new(fixture());
    command.arg(scenario).args(args);
    let child = command.spawn().expect("spawn ptrace fixture");
    let pid = child.id() as libc::pid_t;
    let expected: BTreeSet<&str> = expected_path_arguments.iter().map(String::as_str).collect();

    let mut status = 0;
    assert_eq!(unsafe { libc::waitpid(pid, &mut status, 0) }, pid);
    assert!(libc::WIFSTOPPED(status));
    assert_eq!(libc::WSTOPSIG(status), libc::SIGSTOP);

    let options = libc::PTRACE_O_TRACESYSGOOD as usize as *mut libc::c_void;
    let setoptions = unsafe {
        libc::ptrace(
            libc::PTRACE_SETOPTIONS,
            pid,
            std::ptr::null_mut::<libc::c_void>(),
            options,
        )
    };
    assert_eq!(
        setoptions,
        0,
        "PTRACE_SETOPTIONS: {}",
        std::io::Error::last_os_error()
    );

    let mut entering = true;
    let mut sequence = 0u64;
    let mut pending: Option<PendingOpen> = None;
    let mut generations: BTreeMap<i32, u64> = BTreeMap::new();
    let mut events = Vec::new();
    let mut mutation = mutation;
    ptrace_resume(pid);

    loop {
        status = 0;
        let waited = unsafe { libc::waitpid(pid, &mut status, 0) };
        assert_eq!(waited, pid, "waitpid: {}", std::io::Error::last_os_error());

        if libc::WIFEXITED(status) {
            assert_eq!(libc::WEXITSTATUS(status), 0, "fixture exited non-zero");
            break;
        }
        if libc::WIFSIGNALED(status) {
            panic!("fixture killed by signal {}", libc::WTERMSIG(status));
        }
        assert!(libc::WIFSTOPPED(status));
        let signal = libc::WSTOPSIG(status);
        if signal == SYSCALL_STOP {
            sequence += 1;
            let regs = ptrace_regs(pid);
            if entering {
                pending = open_entry(&regs).and_then(|(operation, pointer)| {
                    let path_argument = read_tracee_c_string(pid, pointer).ok()?;
                    expected.contains(path_argument.as_str()).then_some(PendingOpen {
                        operation,
                        path_argument,
                        entry_sequence: sequence,
                    })
                });
            } else if let Some(open) = pending.take() {
                let raw_return = regs.rax as i64;
                let exit_sequence = sequence;
                if raw_return < 0 {
                    events.push(LiveOpenEvent {
                        operation: open.operation.to_owned(),
                        path_argument: open.path_argument,
                        entry_sequence: open.entry_sequence,
                        exit_sequence,
                        raw_return,
                        fd: None,
                        fd_generation: None,
                        task_count: task_count(pid).unwrap_or(0),
                        object: None,
                        object_after_mutation: None,
                        contextual_fd_link: None,
                        authority: LiveAuthority::Failure {
                            errno: (-raw_return) as i32,
                        },
                    });
                } else {
                    let fd = i32::try_from(raw_return).expect("returned fd fits i32");
                    let count = task_count(pid).unwrap_or(0);
                    let generation = generations.entry(fd).or_insert(0);
                    *generation += 1;
                    let generation = *generation;

                    if count != 1 {
                        events.push(LiveOpenEvent {
                            operation: open.operation.to_owned(),
                            path_argument: open.path_argument,
                            entry_sequence: open.entry_sequence,
                            exit_sequence,
                            raw_return,
                            fd: Some(fd),
                            fd_generation: Some(generation),
                            task_count: count,
                            object: None,
                            object_after_mutation: None,
                            contextual_fd_link: None,
                            authority: LiveAuthority::Ambiguous {
                                reason: "fd_table_relation_unknown_multitask".to_owned(),
                            },
                        });
                    } else {
                        match object_identity(pid, fd) {
                            Ok((object, link)) => {
                                let mut object_after_mutation = None;
                                if let Some(rename) = mutation.take() {
                                    fs::rename(&rename.from, &rename.to)
                                        .expect("rename while tracee stopped");
                                    object_after_mutation = Some(
                                        object_identity(pid, fd)
                                            .expect("object identity after rename")
                                            .0,
                                    );
                                }
                                events.push(LiveOpenEvent {
                                    operation: open.operation.to_owned(),
                                    path_argument: open.path_argument,
                                    entry_sequence: open.entry_sequence,
                                    exit_sequence,
                                    raw_return,
                                    fd: Some(fd),
                                    fd_generation: Some(generation),
                                    task_count: count,
                                    object: Some(object),
                                    object_after_mutation,
                                    contextual_fd_link: link,
                                    authority: LiveAuthority::SuccessObjectBounded,
                                });
                            }
                            Err(reason) => events.push(LiveOpenEvent {
                                operation: open.operation.to_owned(),
                                path_argument: open.path_argument,
                                entry_sequence: open.entry_sequence,
                                exit_sequence,
                                raw_return,
                                fd: Some(fd),
                                fd_generation: Some(generation),
                                task_count: count,
                                object: None,
                                object_after_mutation: None,
                                contextual_fd_link: None,
                                authority: LiveAuthority::Ambiguous { reason },
                            }),
                        }
                    }
                }
            }
            entering = !entering;
        }
        ptrace_resume(pid);
    }

    drop(child);
    events
}

fn write_file(path: &Path, contents: &[u8]) {
    fs::write(path, contents).expect("write fixture file");
}

#[test]
fn b1_live_open_binds_returned_fd_to_kernel_object_identity() {
    let dir = TempDir::new("open");
    let target = dir.path().join("input");
    write_file(&target, b"open");
    let target_text = target.to_string_lossy().into_owned();
    let events = trace_fixture("open", std::slice::from_ref(&target_text), std::slice::from_ref(&target_text), None);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].operation, "open");
    assert_eq!(events[0].success_object(), &identity_of_path(&target));
    assert_eq!(events[0].proof_digest(), events[0].proof_digest());
}

#[test]
fn b1_live_openat_cwd_binds_kernel_object() {
    let dir = TempDir::new("openat-cwd");
    let target = dir.path().join("input");
    write_file(&target, b"openat");
    let target_text = target.to_string_lossy().into_owned();
    let events = trace_fixture("openat-cwd", std::slice::from_ref(&target_text), std::slice::from_ref(&target_text), None);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].operation, "openat");
    assert_eq!(events[0].success_object(), &identity_of_path(&target));
}

#[test]
fn b1_live_openat_real_dirfd_binds_target_not_dir_path() {
    let dir = TempDir::new("openat-dir");
    let target = dir.path().join("item");
    write_file(&target, b"dirfd");
    let dir_text = dir.path().to_string_lossy().into_owned();
    let relative = "item".to_owned();
    let events = trace_fixture(
        "openat-dir",
        &[dir_text, relative.clone()],
        std::slice::from_ref(&relative),
        None,
    );
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].operation, "openat");
    assert_eq!(events[0].success_object(), &identity_of_path(&target));
}

#[test]
fn b1_live_openat2_success_is_bound_on_current_runner_kernel() {
    let dir = TempDir::new("openat2");
    let target = dir.path().join("input");
    write_file(&target, b"openat2");
    let target_text = target.to_string_lossy().into_owned();
    let events = trace_fixture("openat2", std::slice::from_ref(&target_text), std::slice::from_ref(&target_text), None);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].operation, "openat2");
    assert_eq!(events[0].success_object(), &identity_of_path(&target));
}

#[test]
fn b1_live_immediate_close_needs_no_later_io() {
    let dir = TempDir::new("immediate-close");
    let target = dir.path().join("input");
    write_file(&target, b"close");
    let target_text = target.to_string_lossy().into_owned();
    let events = trace_fixture("immediate-close", std::slice::from_ref(&target_text), std::slice::from_ref(&target_text), None);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].success_object(), &identity_of_path(&target));
}

#[test]
fn b1_live_enoent_remains_failure_without_object_identity() {
    let dir = TempDir::new("missing");
    let missing = dir.path().join("does-not-exist");
    let missing_text = missing.to_string_lossy().into_owned();
    let events = trace_fixture("missing", std::slice::from_ref(&missing_text), std::slice::from_ref(&missing_text), None);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].authority, LiveAuthority::Failure { errno: libc::ENOENT });
    assert!(events[0].object.is_none());
}

#[test]
fn b1_live_fd_number_reuse_gets_new_generation_and_new_object_identity() {
    let dir = TempDir::new("fd-reuse");
    let first = dir.path().join("first");
    let second = dir.path().join("second");
    write_file(&first, b"first");
    write_file(&second, b"second");
    let first_text = first.to_string_lossy().into_owned();
    let second_text = second.to_string_lossy().into_owned();
    let events = trace_fixture(
        "fd-reuse",
        &[first_text.clone(), second_text.clone()],
        &[first_text, second_text],
        None,
    );
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].fd, events[1].fd, "fixture should exercise numeric FD reuse");
    assert_ne!(events[0].fd_generation, events[1].fd_generation);
    assert_ne!(events[0].success_object(), events[1].success_object());
}

#[test]
fn b1_live_hardlink_path_differs_while_kernel_identity_matches() {
    let dir = TempDir::new("hardlink");
    let original = dir.path().join("original");
    let alias = dir.path().join("alias");
    write_file(&original, b"hardlink");
    fs::hard_link(&original, &alias).expect("create hard link");
    let alias_text = alias.to_string_lossy().into_owned();
    let events = trace_fixture("open", std::slice::from_ref(&alias_text), std::slice::from_ref(&alias_text), None);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].success_object(), &identity_of_path(&original));
    assert_ne!(original, alias);
}

#[test]
fn b1_live_rename_while_stopped_does_not_change_opened_object_identity() {
    let dir = TempDir::new("rename");
    let before = dir.path().join("before");
    let after = dir.path().join("after");
    write_file(&before, b"rename");
    let before_text = before.to_string_lossy().into_owned();
    let events = trace_fixture(
        "open",
        std::slice::from_ref(&before_text),
        std::slice::from_ref(&before_text),
        Some(RenameMutation {
            from: before.clone(),
            to: after.clone(),
        }),
    );
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].object, events[0].object_after_mutation);
    assert!(!before.exists());
    assert_eq!(events[0].success_object(), &identity_of_path(&after));
}

#[test]
fn b1_live_multitask_fd_table_uncertainty_fails_closed() {
    let dir = TempDir::new("threaded");
    let target = dir.path().join("input");
    write_file(&target, b"threaded");
    let target_text = target.to_string_lossy().into_owned();
    let events = trace_fixture("threaded", std::slice::from_ref(&target_text), std::slice::from_ref(&target_text), None);
    assert_eq!(events.len(), 1);
    assert!(events[0].task_count > 1);
    assert_eq!(
        events[0].authority,
        LiveAuthority::Ambiguous {
            reason: "fd_table_relation_unknown_multitask".to_owned()
        }
    );
    assert!(events[0].object.is_none());
}