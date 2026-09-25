#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use execsurface_normalize::{canonicalize, NormalizationConfig};
use execsurface_observe::{observe_command, CommandSpec};

#[test]
fn repeated_real_observation_produces_identical_canonical_surface() {
    let first = observe_command(&CommandSpec::new("/bin/true")).expect("first observation");
    let second = observe_command(&CommandSpec::new("/bin/true")).expect("second observation");

    assert!(first.complete, "first observation must be complete");
    assert!(second.complete, "second observation must be complete");

    let first_surface =
        canonicalize(&first, &NormalizationConfig::default()).expect("first canonicalization");
    let second_surface =
        canonicalize(&second, &NormalizationConfig::default()).expect("second canonicalization");

    assert_eq!(first_surface, second_surface);

    let first_json = serde_json::to_string(&first_surface).expect("serialize first surface");
    let second_json = serde_json::to_string(&second_surface).expect("serialize second surface");
    assert_eq!(first_json, second_json);
}
