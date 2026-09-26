from pathlib import Path

model = Path('crates/execsurface-model/src/canonical.rs')
normalize = Path('crates/execsurface-normalize/src/lib.rs')

model_text = model.read_text()
old_version = 'pub const NORMALIZATION_PROFILE_VERSION: u32 = 2;'
new_version = 'pub const NORMALIZATION_PROFILE_VERSION: u32 = 3;'
if model_text.count(old_version) != 1:
    raise SystemExit(f'expected one normalization profile v2 marker, found {model_text.count(old_version)}')
model.write_text(model_text.replace(old_version, new_version))

text = normalize.read_text()
old_value = '''            let value = if suffix.is_empty() {
                root.token.clone()
            } else {
                format!("{}{suffix}", root.token)
            };
            let class = classify_tokenized_path(&value, root.class);
'''
new_value = '''            let value = if suffix.is_empty() {
                root.token.clone()
            } else {
                format!("{}{suffix}", root.token)
            };
            let value = normalize_ephemeral_temp_path(value, root.class);
            let class = classify_tokenized_path(&value, root.class);
'''
if text.count(old_value) != 1:
    raise SystemExit(f'expected one root tokenization block, found {text.count(old_value)}')
text = text.replace(old_value, new_value)

anchor = '''fn canonical_kernel_fd_path(path: &str, roots: &[RootRule]) -> CanonicalPath {
    let mut canonical = canonical_path(path, roots);
    canonical.resolution = PathResolution::KernelFdResolved;
    canonical
}

'''
helper = '''fn normalize_ephemeral_temp_path(value: String, class: PathClass) -> String {
    if class != PathClass::Temp {
        return value;
    }

    let Some(rest) = value.strip_prefix("$TMP/go-build") else {
        return value;
    };
    let digit_count = rest.bytes().take_while(u8::is_ascii_digit).count();
    if digit_count == 0 {
        return value;
    }
    let (_, suffix) = rest.split_at(digit_count);
    if !suffix.is_empty() && !suffix.starts_with('/') {
        return value;
    }

    format!("$TMP/go-build<ephemeral>{suffix}")
}

'''
if text.count(anchor) != 1:
    raise SystemExit(f'expected one kernel-fd anchor, found {text.count(anchor)}')
text = text.replace(anchor, anchor + helper)

unit_anchor = '''    #[test]
    fn credential_path_remains_specific_and_sensitive() {
'''
new_tests = '''    #[test]
    fn randomized_go_build_roots_collapse_but_suffix_remains_specific() {
        let roots = build_root_rules(&config_a()).unwrap();
        let first = canonical_path("/tmp/go-build3008370933/b001/vet.cfg", &roots);
        let second = canonical_path("/tmp/go-build1915336995/b001/vet.cfg", &roots);
        let distinct_suffix = canonical_path("/tmp/go-build1915336995/b002/vet.cfg", &roots);

        assert_eq!(first.value, "$TMP/go-build<ephemeral>/b001/vet.cfg");
        assert_eq!(first, second);
        assert_ne!(first, distinct_suffix);
        assert_eq!(first.class, PathClass::Temp);
    }

    #[test]
    fn go_build_normalization_is_digit_only_and_plain_tmp_only() {
        let roots = build_root_rules(&config_a()).unwrap();
        assert_eq!(
            canonical_path("/tmp/go-buildabc/b001", &roots).value,
            "$TMP/go-buildabc/b001"
        );
        assert_eq!(
            canonical_path("/tmp/go-build123abc/b001", &roots).value,
            "$TMP/go-build123abc/b001"
        );
        assert_eq!(
            canonical_path("/tmp/not-go-build123/b001", &roots).value,
            "$TMP/not-go-build123/b001"
        );
        assert_eq!(
            canonical_path("/tmp/run-A/go-build123/b001", &roots).value,
            "$RUN_TMP/go-build123/b001"
        );
    }

'''
if text.count(unit_anchor) != 1:
    raise SystemExit(f'expected one unit-test anchor, found {text.count(unit_anchor)}')
text = text.replace(unit_anchor, new_tests + unit_anchor)
normalize.write_text(text)

print('M9_GO_TEMP_CANONICALIZATION_PATCH_APPLIED profile=3')
