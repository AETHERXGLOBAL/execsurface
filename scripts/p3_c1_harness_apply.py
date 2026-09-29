from pathlib import Path


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one match, found {count}")
    return text.replace(old, new, 1)


path = Path("crates/execsurface-observe/src/linux_ptrace.rs")
text = path.read_text()
anchor = """    #[test]\n    fn normalizes_only_the_tracees_own_proc_identity() {\n"""
insert = r'''    #[test]
    #[ignore = "C1 real ptrace concurrency harness; run in the dedicated Linux gate"]
    fn c1_real_ptrace_clone_modes_are_certified() {
        use std::process::Command;

        let root = std::env::temp_dir().join(format!(
            "execsurface-c1-real-clone-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create C1 harness directory");
        let source = root.join("clone_modes.c");
        let binary = root.join("clone_modes");
        std::fs::write(
            &source,
            r#"#define _GNU_SOURCE
#include <sched.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

static int child_main(void *unused) {
    (void)unused;
    _exit(0);
}

int main(int argc, char **argv) {
    if (argc != 2) return 2;
    int flags = 0;
    if (strcmp(argv[1], "shared") == 0) {
        flags |= CLONE_FILES;
    } else if (strcmp(argv[1], "private") != 0) {
        return 3;
    }

    const size_t stack_size = 1u << 20;
    char *stack = malloc(stack_size);
    if (!stack) return 4;
    pid_t child = clone(child_main, stack + stack_size, flags, NULL);
    if (child < 0) {
        perror("clone");
        free(stack);
        return 5;
    }

    int status = 0;
    if (waitpid(child, &status, __WCLONE) < 0) {
        perror("waitpid");
        free(stack);
        return 6;
    }
    free(stack);
    return WIFEXITED(status) && WEXITSTATUS(status) == 0 ? 0 : 7;
}
"#,
        )
        .expect("write C1 clone harness");

        let compile = Command::new("cc")
            .arg("-O2")
            .arg("-Wall")
            .arg("-Wextra")
            .arg(&source)
            .arg("-o")
            .arg(&binary)
            .status()
            .expect("invoke cc for C1 harness");
        assert!(compile.success(), "C1 clone harness must compile");

        let private = observe(
            &CommandSpec::new(binary.as_os_str()).arg("private"),
            ObserveOptions::default(),
        )
        .expect("observe private clone harness");
        assert_eq!(private.observation.outcome.exit_code, Some(0));
        assert!(private.clone_fd_certification.fully_certified());
        assert!(private.clone_fd_certification.clone_events >= 1);
        assert_eq!(private.clone_fd_certification.shared_fd_transitions, 0);
        assert!(private.clone_fd_certification.cloned_fd_transitions >= 1);
        assert_eq!(
            private
                .observation
                .warnings
                .iter()
                .filter(|warning| warning.code == "clone_flags_unavailable")
                .count(),
            0
        );

        let shared = observe(
            &CommandSpec::new(binary.as_os_str()).arg("shared"),
            ObserveOptions::default(),
        )
        .expect("observe CLONE_FILES harness");
        assert_eq!(shared.observation.outcome.exit_code, Some(0));
        assert!(shared.clone_fd_certification.fully_certified());
        assert!(shared.clone_fd_certification.clone_events >= 1);
        assert!(shared.clone_fd_certification.shared_fd_transitions >= 1);
        assert_eq!(shared.clone_fd_certification.cloned_fd_transitions, 0);
        assert_eq!(
            shared
                .observation
                .warnings
                .iter()
                .filter(|warning| warning.code == "clone_flags_unavailable")
                .count(),
            0
        );

        let _ = std::fs::remove_dir_all(&root);
    }

'''
text = replace_once(text, anchor, insert + anchor, "insert real ptrace harness")
path.write_text(text)
