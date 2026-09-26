from pathlib import Path

path = Path('.github/workflows/m9-1-post-hardening-zero-contact.yml')
text = path.read_text()

old_state = '''          cat > "$evidence/state.env" <<EOF
          install_status=$([ "$install_rc" -eq 0 ] && echo PASS || echo FAIL)
          direct_status=$direct_status
          doctor_status=$doctor_status
          baseline_status=$baseline_status
          check_status=$check_status
          check1_verdict=$check1_verdict
          check2_verdict=$check2_verdict
          performance_status=$performance_status
          operational_error=$operational_error
          EOF
          cat "$evidence/state.env"
'''
new_state = '''          {
            printf 'install_status=%q\\n' "$([ "$install_rc" -eq 0 ] && echo PASS || echo FAIL)"
            printf 'direct_status=%q\\n' "$direct_status"
            printf 'doctor_status=%q\\n' "$doctor_status"
            printf 'baseline_status=%q\\n' "$baseline_status"
            printf 'check_status=%q\\n' "$check_status"
            printf 'check1_verdict=%q\\n' "$check1_verdict"
            printf 'check2_verdict=%q\\n' "$check2_verdict"
            printf 'performance_status=%q\\n' "$performance_status"
            printf 'operational_error=%q\\n' "$operational_error"
          } > "$evidence/state.env"
          cat "$evidence/state.env"
'''
if text.count(old_state) != 1:
    raise SystemExit(f'expected exactly one unsafe state.env block, found {text.count(old_state)}')
text = text.replace(old_state, new_state)

old_failure = '''            else
              direct_status=FAIL
              operational_error="direct compatibility command failed rc=$direct_rc"
            fi
'''
new_failure = '''            else
              direct_status=FAIL
              operational_error="direct compatibility command failed rc=$direct_rc"
              # Diagnostic-only rerun for failed direct workloads. This output is not
              # accepted compatibility evidence and does not alter the frozen command.
              if [ "${{ matrix.project }}" = "ripgrep" ]; then
                set +e
                timeout --signal=TERM --kill-after=5s "${SAMPLE_TIMEOUT}s" bash -lc \
                  "cd '$WORK_ROOT' && cargo +$RUST_TOOLCHAIN test --workspace --all-targets" \
                  >"$evidence/direct-diagnostic.stdout" 2>"$evidence/direct-diagnostic.stderr"
                diagnostic_rc=$?
                set -e
                printf 'diagnostic_only_rc=%s\\n' "$diagnostic_rc" \
                  > "$evidence/direct-diagnostic.status"
              fi
            fi
'''
if text.count(old_failure) != 1:
    raise SystemExit(f'expected exactly one direct-failure block, found {text.count(old_failure)}')
text = text.replace(old_failure, new_failure)

path.write_text(text)
print('M9_ZERO_CONTACT_HARNESS_REPAIR_APPLIED')
