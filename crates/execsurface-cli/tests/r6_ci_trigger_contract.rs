use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

fn workflow_text() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    fs::read_to_string(root.join(".github/workflows/adversarial-regression.yml"))
        .expect("read adversarial regression workflow")
}

fn paths_for_section(document: &str, section: &str) -> BTreeSet<String> {
    let header = format!("  {section}:");
    let mut in_section = false;
    let mut in_paths = false;
    let mut paths = BTreeSet::new();

    for line in document.lines() {
        if line == header {
            in_section = true;
            continue;
        }

        if in_section && line.starts_with("  ") && !line.starts_with("    ") {
            break;
        }

        if !in_section {
            continue;
        }

        if line == "    paths:" {
            in_paths = true;
            continue;
        }

        if in_paths {
            if let Some(value) = line.strip_prefix("      - ") {
                paths.insert(value.trim_matches(['\'', '"']).to_owned());
                continue;
            }

            if !line.trim().is_empty() && !line.starts_with("      ") {
                break;
            }
        }
    }

    assert!(
        in_section && in_paths,
        "workflow must declare {section}.paths explicitly"
    );
    paths
}

#[test]
fn r6_push_and_pull_request_adversarial_paths_are_symmetric() {
    let document = workflow_text();
    let push = paths_for_section(&document, "push");
    let pull_request = paths_for_section(&document, "pull_request");

    assert_eq!(
        push, pull_request,
        "adversarial regression push and pull_request path gates must be identical"
    );
}

#[test]
fn r6_semantic_workspace_changes_trigger_adversarial_regression() {
    let document = workflow_text();
    let required = BTreeSet::from([
        ".github/m12-fixtures/**".to_owned(),
        ".github/workflows/adversarial-regression.yml".to_owned(),
        "crates/**".to_owned(),
        "Cargo.toml".to_owned(),
        "Cargo.lock".to_owned(),
        "README.md".to_owned(),
    ]);

    for section in ["push", "pull_request"] {
        let actual = paths_for_section(&document, section);
        for path in &required {
            assert!(
                actual.contains(path),
                "{section}.paths is missing required adversarial trigger {path:?}; actual={actual:?}"
            );
        }
    }
}
